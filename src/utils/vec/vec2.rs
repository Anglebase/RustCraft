use crate::{utils::SetUniform, *};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy + Into<f32>> Vec2<T> {
    pub fn length(&self) -> f32 {
        (*self * *self).sqrt().into()
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

impl_vec_ops_add!(Vec2<T>, x, y);
impl_vec_ops_sub!(Vec2<T>, x, y);
impl_vec_ops_mul_number!(Vec2<T>, x, y);
impl_vec_ops_mul_vec!(Vec2<T>, x, y);
impl_vec_ops_div_number!(Vec2<T>, x, y);
impl_vec_ops_neg!(Vec2<T>, x, y);
impl_vec_ops_add_assign!(Vec2<T>, x, y);
impl_vec_ops_sub_assign!(Vec2<T>, x, y);
impl_vec_ops_mul_assign_number!(Vec2<T>, x, y);
impl_vec_ops_div_assign_number!(Vec2<T>, x, y);