use crate::{utils::SetUniform, *};

#[derive(Debug, Clone, Copy)]
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

impl<T: Copy + Into<f32>> Vec4<T> {
    pub fn length(&self) -> f32 {
        (*self * *self).sqrt().into()
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

impl_vec_ops_add!(Vec4<T>, x, y, z, w);
impl_vec_ops_sub!(Vec4<T>, x, y, z, w);
impl_vec_ops_mul_number!(Vec4<T>, x, y, z, w);
impl_vec_ops_mul_vec!(Vec4<T>, x, y, z, w);
impl_vec_ops_div_number!(Vec4<T>, x, y, z, w);
impl_vec_ops_neg!(Vec4<T>, x, y, z, w);
impl_vec_ops_add_assign!(Vec4<T>, x, y, z, w);
impl_vec_ops_sub_assign!(Vec4<T>, x, y, z, w);
impl_vec_ops_mul_assign_number!(Vec4<T>, x, y, z, w);
impl_vec_ops_div_assign_number!(Vec4<T>, x, y, z, w);
