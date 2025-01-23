use crate::{utils::*, *};

#[derive(Debug, Clone, Copy)]
pub struct Mat2x4<T> {
    pub(crate) data: [[T; 4]; 2],
}

impl<T: From<f32>> Mat2x4<T> {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0.into(), 0.0.into(), 0.0.into(), 0.0.into()],
                [0.0.into(), 1.0.into(), 0.0.into(), 0.0.into()],
            ],
        }
    }
}

impl<T: From<f32> + Copy> Default for Mat2x4<T> {
    fn default() -> Self {
        Self {
            data: [[0.0.into(); 4]; 2],
        }
    }
}

impl SetUniform for Mat2x4<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2x4fv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl SetUniform for Mat2x4<f64> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2x4dv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl<T> From<[[T; 4]; 2]> for Mat2x4<T> {
    fn from(value: [[T; 4]; 2]) -> Self {
        Self { data: value }
    }
}

impl_mat_ops_add!(Mat2x4<T>, 2, 4);
impl_mat_ops_sub!(Mat2x4<T>, 2, 4);
impl_mat_ops_mul_number!(Mat2x4<T>, 2, 4);
impl_mat_index!(Mat2x4<T>, 2, 4);
impl_mat_ops_div_number!(Mat2x4<T>, 2, 4);
impl_mat_ops_neg!(Mat2x4<T>, 2, 4);
impl_mat_ops_add_assign!(Mat2x4<T>, 2, 4);
impl_mat_ops_sub_assign!(Mat2x4<T>, 2, 4);
impl_mat_ops_mul_assign_number!(Mat2x4<T>, 2, 4);
impl_mat_ops_div_assign_number!(Mat2x4<T>, 2, 4);
