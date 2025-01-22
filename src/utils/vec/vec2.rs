use crate::utils::SetUniform;

pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl SetUniform for Vec2<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2f(location, self.x, self.y);
        }
    }
}

impl SetUniform for Vec2<i32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2i(location, self.x, self.y);
        }
    }
}

impl SetUniform for Vec2<u32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2ui(location, self.x, self.y);
        }
    }
}
