use crate::utils::SetUniform;

pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl Vec4<f32> {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl SetUniform for Vec4<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4f(location, self.x, self.y, self.z, self.w);
        }
    }
}

impl SetUniform for Vec4<i32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4i(location, self.x, self.y, self.z, self.w);
        }
    }
}

impl SetUniform for Vec4<u32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4ui(location, self.x, self.y, self.z, self.w);
        }
    }
}
