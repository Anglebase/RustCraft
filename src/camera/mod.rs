use std::collections::HashMap;

use crate::{utils::Mat4, warn, RustCraftWrapper};

pub trait Camera {
    fn view_matrix(&self) -> Mat4<f32>;
    fn update(&mut self, window: &mut glfw::Window);
    fn mouse_move(&mut self, xpos: f64, ypos: f64);
    fn mouse_scroll(&mut self, xoffset: f64, yoffset: f64);
}

mod space_camera;
pub use space_camera::SpaceCamera;

pub struct CameraSystem {
    cameras: HashMap<String, Box<dyn Camera + Send + 'static>>,
    active_camera: Option<String>,
    enable_mouse: bool,
}

impl Camera for CameraSystem {
    fn view_matrix(&self) -> Mat4<f32> {
        if self.active_camera.is_none() {
            Mat4::I()
        } else {
            self.cameras[self.active_camera.as_ref().unwrap()].view_matrix()
        }
    }

    fn update(&mut self, window: &mut glfw::Window) {
        if let Some(name) = self.active_camera.as_ref() {
            self.cameras.get_mut(name).unwrap().update(window);
        }
    }

    fn mouse_move(&mut self, xpos: f64, ypos: f64) {
        if self.enable_mouse {
            return;
        }
        if let Some(name) = self.active_camera.as_ref() {
            self.cameras.get_mut(name).unwrap().mouse_move(xpos, ypos);
        }
    }

    fn mouse_scroll(&mut self, xoffset: f64, yoffset: f64) {
        if self.enable_mouse {
            return;
        }
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
            enable_mouse: true,
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

    pub fn enable_mouse(&self, enable: bool) {
        self.apply(|sys| {
            sys.enable_mouse = enable;
        });
    }
}
