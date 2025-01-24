use super::{Camera, CAMERA_SYSTEM};
use crate::utils::{math::*, *};

pub struct GodCamera {
    view_matrix: Mat4<f32>,

    x: f32,
    yaw: f32,
    pitch: f32,

    sen: f32,
    scl: f32,

    move_camera: bool,
    last_x: f64,
    last_y: f64,
}

impl Camera for GodCamera {
    fn view_matrix(&self) -> Mat4<f32> {
        self.view_matrix
    }

    fn update(&mut self, window: &mut glfw::Window) {
        self.move_camera = window.get_mouse_button(glfw::MouseButtonLeft) == glfw::Action::Press;
        let pos = Vec4::<f32>::from([0.0, 0.0, 0.0, 1.0])
            * tranlate3(Vec3::from([0.0, 0.0, 2.0_f32.powf(self.x)]))
            * rotate3_x(radian(self.pitch))
            * rotate3_y(radian(-self.yaw));
        let target = Vec3::<f32>::new();
        let up = Vec4::<f32>::from([0.0, 1.0, 0.0, 1.0])
            * rotate3_x(radian(self.pitch))
            * rotate3_y(radian(-self.yaw));
        self.view_matrix = look_at(pos.xyz(), target, up.xyz());
    }

    fn mouse_move(&mut self, xpos: f64, ypos: f64) {
        if !self.move_camera {
            self.last_x = xpos;
            self.last_y = ypos;
            return;
        } else {
            let xoffset = xpos - self.last_x;
            let yoffset = self.last_y - ypos;
            self.last_x = xpos;
            self.last_y = ypos;

            self.yaw += xoffset as f32 * self.sen;
            self.pitch += yoffset as f32 * self.sen;

            // limit pitch
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            }
            if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }
    }

    fn mouse_scroll(&mut self, _: f64, yoffset: f64) {
        self.x -= yoffset as f32 / 2.0 * self.scl;
    }
}

impl GodCamera {
    pub fn new(x: f32, sen: f32, scl: f32) -> Self {
        Self {
            view_matrix: Mat4::I(),
            x,
            yaw: 0.0,
            pitch: 0.0,
            sen,
            scl,
            move_camera: false,
            last_x: 0.0,
            last_y: 0.0,
        }
    }
}
