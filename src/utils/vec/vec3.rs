use crate::{impl_vec_ops_add, impl_vec_ops_sub, utils::SetUniform};

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl SetUniform for Vec3<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self.x, self.y, self.z);
        }
    }
}

impl SetUniform for Vec3<i32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3i(location, self.x, self.y, self.z);
        }
    }
}

impl SetUniform for Vec3<u32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3ui(location, self.x, self.y, self.z);
        }
    }
}

impl_vec_ops_add!(Vec3<T>, x, y, z);
impl_vec_ops_sub!(Vec3<T>, x, y, z);