use crate::{impl_mat_ops_add, impl_mat_ops_sub, utils::SetUniform};

#[derive(Debug, Clone, Copy)]
pub struct Mat2<T> {
    data: [[T; 2]; 2],
}

impl<T: From<f32>> Mat2<T> {
    pub fn new() -> Self {
        Self {
            data: [[1.0.into(), 0.0.into()], [0.0.into(), 1.0.into()]],
        }
    }
}

impl<T: From<f32> + Copy> Default for Mat2<T> {
    fn default() -> Self {
        Self {
            data: [[0.0.into(); 2]; 2],
        }
    }
}

impl SetUniform for Mat2<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2fv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl SetUniform for Mat2<f64> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2dv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl_mat_ops_add!(Mat2<T>, 2, 2);
impl_mat_ops_sub!(Mat2<T>, 2, 2);