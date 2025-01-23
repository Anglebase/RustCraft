pub trait SetUniform {
    fn give(&self, location: i32);
}

impl SetUniform for f32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}

impl SetUniform for f64 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1d(location, *self);
        }
    }
}

impl SetUniform for i32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl SetUniform for u32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1ui(location, *self);
        }
    }
}

mod mat;
pub mod math;
mod types;
mod uniform_f32;
mod uniform_f64;
mod uniform_i32;
mod uniform_u32;
pub use types::*;

pub fn radian(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

pub fn look_at(eye: Vec3<f32>, target: Vec3<f32>, up: Vec3<f32>) -> Mat4<f32> {
    let z = (eye - target).normalize(); // 计算z轴方向向量
    let x = up.cross(z).normalize(); // 计算x轴方向向量
    let y = z.cross(x); // 计算y轴方向向量

    let translation = [
        [1.0, 0.0, 0.0, -eye.x()],
        [0.0, 1.0, 0.0, -eye.y()],
        [0.0, 0.0, 1.0, -eye.z()],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let rotation = [
        [*x.x(), *x.y(), *x.z(), 0.0],
        [*y.x(), *y.y(), *y.z(), 0.0],
        [*z.x(), *z.y(), *z.z(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    Mat4::from(translation) * Mat4::from(rotation)
}

pub fn perspective(fov: f32, aspect: f32, z_near: f32, z_far: f32) -> Mat4<f32> {
    let f = 1.0 / (fov / 2.0).tan();
    let mut result = [[0.0; 4]; 4];

    result[0][0] = f / aspect;
    result[1][1] = f;
    result[2][2] = (z_far + z_near) / (z_near - z_far);
    result[2][3] = (2.0 * z_far * z_near) / (z_near - z_far);
    result[3][2] = -1.0;

    Mat4::from(result)
}
