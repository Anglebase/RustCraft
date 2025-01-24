use std::collections::HashMap;

use crate::{utils::Mat4, warn, RustCraftWrapper};

pub trait Camera {
    fn view_matrix(&self) -> Mat4<f32>;
    fn update(&mut self, window: &mut glfw::Window) {
        let _ = window;
    }
    fn mouse_move(&mut self, xpos: f64, ypos: f64) {
        let _ = (xpos, ypos);
    }
    fn mouse_scroll(&mut self, xoffset: f64, yoffset: f64) {
        let _ = (xoffset, yoffset);
    }
}

mod god_camera;
mod space_camera;
pub use god_camera::GodCamera;
pub use space_camera::SpaceCamera;

pub struct CameraSystem {
    cameras: HashMap<String, Box<dyn Camera + Send + 'static>>,
    active_camera: Option<String>,
}

impl Camera for CameraSystem {
    /// 获取当前摄像机的视图矩阵
    ///
    /// # 注解
    ///
    /// 此函数于通常在渲染线程中执行
    fn view_matrix(&self) -> Mat4<f32> {
        if let Some(name) = self.active_camera.as_ref() {
            self.cameras.get(name).unwrap().view_matrix()
        } else {
            Mat4::I()
        }
    }

    /// 更新摄像机状态
    ///
    /// # 注解
    ///
    /// 此函数于轮询线程中执行
    fn update(&mut self, window: &mut glfw::Window) {
        if let Some(name) = self.active_camera.as_ref() {
            self.cameras.get_mut(name).unwrap().update(window);
        }
    }

    /// 处理鼠标移动事件
    ///
    /// # 注解
    ///
    /// 此函数于主线程(鼠标移动回调函数)中执行
    fn mouse_move(&mut self, xpos: f64, ypos: f64) {
        if let Some(name) = self.active_camera.as_ref() {
            self.cameras.get_mut(name).unwrap().mouse_move(xpos, ypos);
        }
    }

    /// 处理鼠标滚轮事件
    ///
    /// # 注解
    ///
    /// 此函数于主线程(鼠标滚轮回调函数)中执行
    fn mouse_scroll(&mut self, xoffset: f64, yoffset: f64) {
        if let Some(name) = self.active_camera.as_ref() {
            self.cameras
                .get_mut(name)
                .unwrap()
                .mouse_scroll(xoffset, yoffset);
        }
    }
}

impl CameraSystem {
    pub fn new() -> Self {
        Self {
            cameras: HashMap::new(),
            active_camera: None,
        }
    }

    pub fn add_camera(&mut self, name: &str, camera: Box<dyn Camera + Send + 'static>) {
        if self.cameras.contains_key(name) {
            warn!("CameraSystem", "名称 {} 已经被占用", name);
        } else {
            self.cameras.insert(name.to_string(), camera);
        }
    }

    pub fn active_camera(&mut self, name: &str) {
        if self.cameras.contains_key(name) {
            self.active_camera = Some(name.to_string());
        } else {
            warn!("CameraSystem", "没有名为 {} 的摄像机实例", name);
        }
    }
}

use lazy_static::lazy_static;

lazy_static! {
    pub static ref CAMERA_SYSTEM: RustCraftWrapper<CameraSystem> =
        RustCraftWrapper::new(CameraSystem::new());
}

impl RustCraftWrapper<CameraSystem> {
    pub fn add_camera<T: Camera + Send + 'static>(&self, name: &str, camera: T) {
        self.apply(|sys| {
            sys.add_camera(name, Box::new(camera));
        });
    }

    pub fn active_camera(&self, name: &str) {
        self.apply(|sys| {
            sys.active_camera(name);
        });
    }

    pub fn view_matrix(&self) -> Mat4<f32> {
        let mut mat = Mat4::I();
        self.apply(|sys| {
            mat = sys.view_matrix();
        });
        mat
    }
}
