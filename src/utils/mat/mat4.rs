use crate::{impl_mat_ops_add, impl_mat_ops_mul_number, impl_mat_ops_sub, utils::SetUniform};

#[derive(Debug, Clone, Copy)]
pub struct Mat4<T> {
    data: [[T; 4]; 4],
}

impl<T: From<f32>> Mat4<T> {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0.into(), 0.0.into(), 0.0.into(), 0.0.into()],
                [0.0.into(), 1.0.into(), 0.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into(), 1.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into(), 0.0.into(), 1.0.into()],
            ],
        }
    }
}

impl<T: From<f32> + Copy> Default for Mat4<T> {
    fn default() -> Self {
        Self {
            data: [[0.0.into(); 4]; 4],
        }
    }
}

impl SetUniform for Mat4<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl SetUniform for Mat4<f64> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3dv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl_mat_ops_add!(Mat4<T>, 4, 4);
impl_mat_ops_sub!(Mat4<T>, 4, 4);
impl_mat_ops_mul_number!(Mat4<T>, 4, 4);
