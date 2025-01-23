use crate::utils::*;

pub struct Camera {
    pos: Vec3<f32>,
    front: Vec3<f32>,
    up: Vec3<f32>,

    last_x: f32,
    last_y: f32,
    first_mouse: bool,

    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new(pos: Vec3<f32>, front: Vec3<f32>, up: Vec3<f32>) -> Self {
        Self {
            pos,
            front,
            up,
            last_x: 0.0,
            last_y: 0.0,
            first_mouse: true,
            yaw: -90.0,
            pitch: 0.0,
        }
    }

    pub fn update(&mut self, window: &mut glfw::Window) {
        use crate::App;
        let speed = 2.5 * App::delta_time();
        if window.get_key(glfw::Key::W) == glfw::Action::Press {
            self.pos = self.pos + self.front * speed;
        }
        if window.get_key(glfw::Key::S) == glfw::Action::Press {
            self.pos = self.pos - self.front * speed;
        }
        if window.get_key(glfw::Key::A) == glfw::Action::Press {
            self.pos = self.pos - self.front.cross(self.up) * speed;
        }
        if window.get_key(glfw::Key::D) == glfw::Action::Press {
            self.pos = self.pos + self.front.cross(self.up) * speed;
        }
        if window.get_key(glfw::Key::Space) == glfw::Action::Press {
            self.pos = self.pos + self.up * speed;
        }
        if window.get_key(glfw::Key::LeftShift) == glfw::Action::Press {
            self.pos = self.pos - self.up * speed;
        }
    }

    pub fn mouse_update(&mut self, xpos: f32, ypos: f32) {
        if self.first_mouse {
            self.last_x = xpos;
            self.last_y = ypos;
            self.first_mouse = false;
        }
        let mut xoffset = xpos - self.last_x;
        let mut yoffset = self.last_y - ypos;
        self.last_x = xpos;
        self.last_y = ypos;

        let sensitivity = 0.1;
        xoffset *= sensitivity;
        yoffset *= sensitivity;

        self.yaw += xoffset;
        self.pitch += yoffset;

        if self.pitch > 89.0 {
            self.pitch = 89.0;
        }
        if self.pitch < -89.0 {
            self.pitch = -89.0;
        }

        let front = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.front = front.normalize();
    }

    pub fn view_matrix(&self) -> Mat4<f32> {
        look_at(self.pos, self.pos + self.front, self.up)
    }
}
