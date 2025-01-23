use crate::{debug, error, info, RustCraftWrapper};
use glfw::*;
use lazy_static::lazy_static;
use std::{
    sync::mpsc::{channel, Receiver},
    thread::spawn,
    time::Instant,
};
lazy_static! {
    static ref DELTA_TIME: RustCraftWrapper<f32> = RustCraftWrapper::new(0.0);
    static ref UNIQUE_APP: RustCraftWrapper<Option<()>> = RustCraftWrapper::new(None);
    static ref APP_TIME: RustCraftWrapper<Instant> = RustCraftWrapper::new(Instant::now());
    static ref WINDOW_SIZE: RustCraftWrapper<(i32, i32)> = RustCraftWrapper::new((0, 0));
}

pub struct AppBuilder {
    size: (u32, u32),
    title: String,

    render_init_func: Option<Box<dyn FnOnce() + Send + 'static>>,
    render_loop_func: Option<Box<dyn FnMut() + Send + 'static>>,
    key_callback: Option<Box<dyn FnMut(&mut Window, Key, i32, Action, Modifiers) + Send + 'static>>,
    event_callback: Option<Box<dyn FnMut(&mut Window) + Send + 'static>>,
    cursor_pos_callback: Option<Box<dyn FnMut(&mut Window, f64, f64) + Send + 'static>>,
    fix_cursor: bool,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            size: (800, 600),
            title: "RustCraft".to_string(),
            render_init_func: None,
            render_loop_func: None,
            key_callback: None,
            event_callback: None,
            cursor_pos_callback: None,
            fix_cursor: false,
        }
    }
}

impl AppBuilder {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        Self {
            size: (width, height),
            title: title.to_string(),
            ..Default::default()
        }
    }

    pub fn disable_cursor(&mut self) -> &mut Self {
        self.fix_cursor = true;
        self
    }

    /// 设置渲染线程初始化回调函数
    ///
    /// # 注解 Note
    /// + 调用此函数时，OpenGL 上下文已完成初始化
    /// + 此函数应在 `App::new()` 之前调用
    pub fn set_render_init_callback<F>(&mut self, func: F) -> &mut Self
    where
        F: FnOnce() + Send + 'static,
    {
        self.render_init_func = Some(Box::new(func));
        self
    }
    /// 设置渲染线程循环回调函数
    /// 此函数通常是渲染函数
    ///
    /// # 注解 Note
    /// 此函数应在 `App::new()` 之前调用
    pub fn set_render_loop_callback<F>(&mut self, func: F) -> &mut Self
    where
        F: FnMut() + Send + 'static,
    {
        self.render_loop_func = Some(Box::new(func));
        self
    }

    /// 设置键盘事件回调函数
    ///
    /// # 注解 Note
    /// 此函数应在 `App::new()` 之前调用
    pub fn set_key_callback<F>(&mut self, func: F) -> &mut Self
    where
        F: FnMut(&mut Window, Key, i32, Action, Modifiers) + Send + 'static,
    {
        self.key_callback = Some(Box::new(func));
        self
    }

    /// 设置事件轮询函数
    ///
    /// # 注解 Note
    /// + 此函数应在 `App::new()` 之前调用
    /// + 该函数在渲染线程执行，每一帧都会被调用一次
    pub fn set_event_callback<F>(&mut self, func: F) -> &mut Self
    where
        F: FnMut(&mut Window) + Send + 'static,
    {
        self.event_callback = Some(Box::new(func));
        self
    }

    /// 设置鼠标移动事件回调函数
    ///
    /// # 注解 Note
    /// 此函数应在 `App::new()` 之前调用
    pub fn set_cursor_pos_callback<F>(&mut self, func: F) -> &mut Self
    where
        F: FnMut(&mut Window, f64, f64) + Send + 'static,
    {
        self.cursor_pos_callback = Some(Box::new(func));
        self
    }

    /// 执行应用程序初始化
    ///
    /// # 参数 Parameters
    /// - `width` - 窗口宽度
    /// - `height` - 窗口高度
    /// - `title` - 窗口标题
    ///
    /// # 返回值 Returns
    /// 返回 `App` 实例
    ///
    /// # 注解 Note
    pub fn build(&mut self) -> App {
        UNIQUE_APP.apply(|data| {
            if data.is_some() {
                panic!("只能创建一个 App 实例！");
            }
            *data = Some(());
        });
        info!("App", "程序已启动");
        debug!("App::new()", "初始化 GLFW ...");
        let mut glfw = if let Ok(glfw) = init(fail_on_errors) {
            glfw
        } else {
            error!("App::new()", "初始化 GLFW 失败！");
            panic!("初始化 GLFW 失败！");
        };
        glfw.window_hint(WindowHint::Visible(false));
        let mut window = if let Some((window, _)) =
            glfw.create_window(self.size.0, self.size.1, &self.title, WindowMode::Windowed)
        {
            window
        } else {
            error!("App::new()", "创建 GLFW 窗口失败！");
            panic!("创建 GLFW 窗口失败！");
        };
        debug!("App::new()", "初始化 GLFW 窗口 ...");
        unsafe {
            use glfw::ffi::*;
            let mode = glfwGetVideoMode(glfwGetPrimaryMonitor()).as_ref().unwrap();
            let (w, h) = window.get_size();
            window.set_pos((mode.width - w) / 2, (mode.height - h) / 2);
        }
        if self.key_callback.is_some() {
            let func = self.key_callback.take().unwrap();
            window.set_key_callback(func);
        }
        if self.cursor_pos_callback.is_some() {
            let func = self.cursor_pos_callback.take().unwrap();
            window.set_cursor_pos_callback(func);
        }
        let (size_tx, size_rx) = channel();
        window.set_size_callback(move |_, w, h| {
            size_tx.send((w, h)).unwrap();
            WINDOW_SIZE.apply(|data| {
                *data = (w, h);
            });
        });
        WINDOW_SIZE.apply(|data| {
            *data = (self.size.0 as i32, self.size.1 as i32);
        });
        if self.fix_cursor {
            window.set_cursor_mode(CursorMode::Disabled);
        }
        debug!("App::new()", "启动 GLFW 渲染线程 ...");
        let (tx, rx) = channel();
        let (initialized_tx, initialized_rx) = channel();
        let (return_tx, return_rx) = channel();
        let render_init_func = self.render_init_func.take();
        let mut render_loop_func = self.render_loop_func.take();
        let mut event_callback = self.event_callback.take();
        spawn(move || {
            debug!("App::new()/render", "GLFW 渲染线程已启动");
            debug!("App::new()/render", "正在初始化 OpenGL 上下文 ...");
            window.make_current();
            debug!("App::new()/render", "加载 OpenGL 函数指针 ...");
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::Viewport::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            debug!("App::new()/render", "初始化渲染依赖 ...");
            // 初始化渲染依赖
            if let Some(func) = render_init_func {
                func();
            }
            // 暂时借出所有权
            initialized_tx.send(window).unwrap();
            let mut window: PWindow = return_rx.recv().unwrap();
            debug!("App::new()/render", "启动渲染循环 ...");
            // FPS 计数器
            let mut now = std::time::Instant::now();
            while !window.should_close() {
                // 允许处理事件
                tx.send(()).unwrap();
                // 计算此帧渲染时间
                let dt = now.elapsed().as_secs_f32();
                now = std::time::Instant::now();
                DELTA_TIME.apply(|data| {
                    *data = dt;
                });
                // 更新窗口尺寸
                if let Ok((w, h)) = size_rx.try_recv() {
                    unsafe { gl::Viewport(0, 0, w as i32, h as i32) }
                }
                // 渲染
                if let Some(func) = render_loop_func.as_mut() {
                    func();
                }
                window.swap_buffers();
                // 处理事件
                if let Some(func) = event_callback.as_mut() {
                    func(&mut window);
                }
            }
            debug!("App::new()/render", "渲染线程已退出");
            info!("App", "程序即将退出");
        });
        {
            // 当渲染线程初始化完成后，显示 GLFW 窗口
            let mut window = initialized_rx.recv().unwrap();
            info!("App", "初始化已完成");
            debug!("App::new()", "显示 GLFW 窗口 ...");
            window.show();
            return_tx.send(window).unwrap();
        }
        debug!("App::new()", "GLFW 渲染线程已初始化");
        App { glfw, rx }
    }
}

pub struct App {
    glfw: Glfw,
    rx: Receiver<()>,
}

impl App {
    /// 启动事件循环
    pub fn exec(&mut self) {
        debug!("App::exec()", "启动事件循环 ...");
        while let Ok(()) = self.rx.recv() {
            self.glfw.poll_events();
        }
        debug!("App::exec()", "事件循环已退出");
    }

    
    /// 获取当前渲染帧率
    pub fn fps() -> f32 {
        let mut fps = 0.0;
        DELTA_TIME.apply(|data| {
            fps = 1.0 / *data;
        });
        fps
    }

    /// 获取当前帧渲染时间
    pub fn delta_time() -> f32 {
        let mut dt = 0.0;
        DELTA_TIME.apply(|data| {
            dt = *data;
        });
        dt
    }

    /// 获取程序运行时间
    pub fn time() -> f32 {
        let mut t = 0.0;
        APP_TIME.apply(|data| {
            t = data.elapsed().as_secs_f32();
        });
        t
    }

    pub fn window_size() -> (i32, i32) {
        let mut size = (0, 0);
        WINDOW_SIZE.apply(|data| {
            size = *data;
        });
        size
    }
}
