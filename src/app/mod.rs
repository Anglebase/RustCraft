use crate::{debug, error, info, RustCraftWrapper};
use glfw::*;
use lazy_static::lazy_static;
use std::{
    sync::mpsc::{channel, Receiver},
    thread::spawn,
};
lazy_static! {
    static ref RENDER_INIT_FUNC: RustCraftWrapper<Option<Box<dyn FnOnce() + Send + 'static>>> =
        RustCraftWrapper::new(None);
    static ref RENDER_LOOP_FUNC: RustCraftWrapper<Option<Box<dyn FnMut() + Send + 'static>>> =
        RustCraftWrapper::new(None);
}

mod events;

pub struct App {
    glfw: Glfw,
    rx: Receiver<()>,
    fps_rx: Receiver<u32>,
    fps: u32,
}

impl App {
    /// 设置渲染线程初始化回调函数
    /// 
    /// # 注解 Note
    /// + 调用此函数时，OpenGL 上下文已完成初始化
    /// + 此函数应在 `App::new()` 之前调用
    pub fn set_render_init_callback<F>(func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        RENDER_INIT_FUNC.apply(|data| {
            *data = Some(Box::new(func));
        });
    }
    /// 设置渲染线程循环回调函数
    /// 此函数通常是渲染函数
    /// 
    /// # 注解 Note
    /// 此函数应在 `App::new()` 之前调用
    pub fn set_render_loop_callback<F>(func: F)
    where
        F: FnMut() + Send + 'static,
    {
        RENDER_LOOP_FUNC.apply(|data| {
            *data = Some(Box::new(func));
        });
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
    pub fn new(width: u32, height: u32, title: &str) -> Self {
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
            glfw.create_window(width, height, title, WindowMode::Windowed)
        {
            window
        } else {
            error!("App::new()", "创建 GLFW 窗口失败！");
            panic!("创建 GLFW 窗口失败！");
        };
        debug!("App::new()", "初始化 GLFW 窗口 ...");
        window.set_key_callback(events::key_callback);
        let (size_tx, size_rx) = channel();
        window.set_size_callback(move |_, w, h| {
            size_tx.send((w, h)).unwrap();
        });
        debug!("App::new()", "启动 GLFW 渲染线程 ...");
        let (tx, rx) = channel();
        let (fps_tx, fps_rx) = channel();
        let (initialized_tx, initialized_rx) = channel();
        let (return_tx, return_rx) = channel();
        spawn(move || {
            debug!("App::new()/render", "GLFW 渲染线程已启动");
            debug!("App::new()/render", "正在初始化 OpenGL 上下文 ...");
            window.make_current();
            debug!("App::new()/render", "加载 OpenGL 函数指针 ...");
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::Viewport::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            debug!("App::new()/render", "初始化渲染依赖 ...");
            // 初始化渲染依赖
            RENDER_INIT_FUNC.apply(|data| {
                if data.is_some() {
                    let func = data.take().unwrap();
                    func();
                }
            });
            // 暂时借出所有权
            initialized_tx.send(window).unwrap();
            let mut window: PWindow = return_rx.recv().unwrap();
            debug!("App::new()/render", "启动渲染循环 ...");
            // FPS 计数器
            let mut frame_count = 0;
            let mut now = std::time::Instant::now();
            while !window.should_close() {
                // 允许处理事件
                tx.send(()).unwrap();
                // 统计 FPS
                frame_count += 1;
                let elapsed = now.elapsed();
                if elapsed.as_secs() >= 1 {
                    let fps = frame_count;
                    fps_tx.send(fps).unwrap();
                    frame_count = 0;
                    now = std::time::Instant::now();
                }
                // 更新窗口尺寸
                if let Ok((w, h)) = size_rx.try_recv() {
                    unsafe { gl::Viewport(0, 0, w as i32, h as i32) }
                }
                // 渲染
                RENDER_LOOP_FUNC.apply(|data| {
                    if let Some(func) = data.as_mut() {
                        func();
                    }
                });
                window.swap_buffers();
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

        Self {
            glfw,
            rx,
            fps_rx,
            fps: 0,
        }
    }

    /// 启动事件循环
    pub fn exec(&mut self) {
        debug!("App::exec()", "启动事件循环 ...");
        while let Ok(()) = self.rx.recv() {
            self.glfw.poll_events();
            if let Ok(fps) = self.fps_rx.try_recv() {
                self.fps = fps;
                debug!("App::exec()", "FPS: {}", self.fps);
            }
        }
        debug!("App::exec()", "事件循环已退出");
    }

    /// 获取当前帧率
    pub fn get_fps(&self) -> u32 {
        self.fps
    }
}
