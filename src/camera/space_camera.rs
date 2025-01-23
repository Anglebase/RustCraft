use math::*;

use super::*;
use crate::{utils::*, App};

pub struct SpaceCamera {
    pos: Vec3<f32>,

    /// 偏航角
    yaw: f32,
    /// 俯仰角
    pitch: f32,

    last_x: f64,
    last_y: f64,
    first_mouse: bool,

    speed: f32,
    sen: f32,
}

impl Camera for SpaceCamera {
    fn view_matrix(&self) -> Mat4<f32> {
        let front = Vec4::from([0.0, 0.0, -0.1, 1.0])
            * rotate3_x(-radian(self.pitch))
            * rotate3_y(radian(self.yaw));
        let up = Vec4::from([0.0, 1.0, 0.0, 1.0])
            * rotate3_x(-radian(self.pitch))
            * rotate3_y(radian(self.yaw));
        look_at(self.pos, self.pos + front.xyz(), up.xyz())
    }

    fn update(&mut self, window: &mut glfw::Window) {
        let dt = App::delta_time();
        let speed = self.speed * dt;
        let up = Vec3::from([0.0, 1.0, 0.0]);
        let front = (Vec4::from([0.0, 0.0, -1.0, 1.0]) * rotate3_y(radian(self.yaw))).xyz();
        let right = front.cross(up).normalize();
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.pos += front * speed;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.pos -= front * speed;
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.pos += right * speed;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.pos -= right * speed;
        }
        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            self.pos += up * speed;
        }
        if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            self.pos -= up * speed;
        }
    }

    fn mouse_move(&mut self, xpos: f64, ypos: f64) {
        if self.first_mouse {
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse = false;
        }
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

    fn mouse_scroll(&mut self, xoffset: f64, yoffset: f64) {
        let _ = (xoffset, yoffset);
    }
}

impl SpaceCamera {
    /// 创建一个空间相机
    /// 
    /// # 参数
    /// 
    /// * `pos` - 相机位置
    /// * `speed` - 相机移动速度
    /// * `sen` - 相机旋转速度
    pub fn new(pos: Vec3<f32>, speed: f32, sen: f32) -> Self {
        Self {
            pos,
            yaw: -90.0,
            pitch: 0.0,
            last_x: 0.0,
            last_y: 0.0,
            first_mouse: true,
            speed,
            sen,
        }
    }
}
