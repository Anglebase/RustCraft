use crate::{utils::*, *};

#[derive(Debug, Clone, Copy)]
pub struct Mat3x4<T> {
    pub(crate) data: [[T; 4]; 3],
}

impl<T: From<f32>> Mat3x4<T> {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0.into(), 0.0.into(), 0.0.into(), 0.0.into()],
                [0.0.into(), 1.0.into(), 0.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into(), 1.0.into(), 0.0.into()],
            ],
        }
    }
}

impl<T: From<f32> + Copy> Default for Mat3x4<T> {
    fn default() -> Self {
        Self {
            data: [[0.0.into(); 4]; 3],
        }
    }
}

impl SetUniform for Mat3x4<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x4fv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl SetUniform for Mat3x4<f64> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x4dv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl<T> From<[[T; 4]; 3]> for Mat3x4<T> {
    fn from(value: [[T; 4]; 3]) -> Self {
        Self { data: value }
    }
}

impl_mat_ops_add!(Mat3x4<T>, 3, 4);
impl_mat_ops_sub!(Mat3x4<T>, 3, 4);
impl_mat_ops_mul_number!(Mat3x4<T>, 3, 4);
impl_mat_index!(Mat3x4<T>, 3, 4);
impl_mat_ops_div_number!(Mat3x4<T>, 3, 4);
impl_mat_ops_neg!(Mat3x4<T>, 3, 4);
impl_mat_ops_add_assign!(Mat3x4<T>, 3, 4);
impl_mat_ops_sub_assign!(Mat3x4<T>, 3, 4);
impl_mat_ops_mul_assign_number!(Mat3x4<T>, 3, 4);
impl_mat_ops_div_assign_number!(Mat3x4<T>, 3, 4);
