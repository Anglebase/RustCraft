pub trait SetUniform {
    fn give(&self, location: i32);
}

impl SetUniform for i32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

impl SetUniform for f32 {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
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

mod vec;
mod mat;
pub use vec::*;
pub use mat::*;