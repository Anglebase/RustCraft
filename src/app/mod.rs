use std::{
    sync::mpsc::{channel, Receiver},
    thread::spawn,
};

mod events;
mod render;

use crate::{debug, error};
use glfw::*;

pub struct App {
    glfw: Glfw,
    rx: Receiver<()>,
}

impl App {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        debug!("App::new()", "初始化 GLFW ...");
        let mut glfw = if let Ok(glfw) = init(fail_on_errors) {
            glfw
        } else {
            error!("App::new()", "初始化 GLFW 失败！");
            panic!("初始化 GLFW 失败！");
        };
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
        spawn(move || {
            debug!("App::new()/render", "GLFW 渲染线程已启动");
            debug!("App::new()/render", "正在初始化 OpenGL 上下文 ...");
            window.make_current();
            debug!("App::new()/render", "加载 OpenGL 函数指针 ...");
            gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            gl::Viewport::load_with(|symbol| window.get_proc_address(symbol) as *const _);
            debug!("App::new()/render", "初始化渲染依赖 ...");
            // 初始化渲染依赖
            render::init();
            debug!("App::new()/render", "启动渲染循环 ...");
            while !window.should_close() {
                if let Ok((w, h)) = size_rx.try_recv() {
                    unsafe { gl::Viewport(0, 0, w as i32, h as i32) }
                }
                render::render();
                window.swap_buffers();
            }
            debug!("App::new()/render", "渲染线程已退出");
            tx.send(()).unwrap();
        });

        Self { glfw, rx }
    }

    pub fn exec(&mut self) {
        debug!("App::exec()", "启动事件循环 ...");
        loop {
            self.glfw.poll_events();
            if let Ok(()) = self.rx.try_recv() {
                debug!("App::exec()", "事件循环将退出...");
                break;
            }
        }
    }
}
