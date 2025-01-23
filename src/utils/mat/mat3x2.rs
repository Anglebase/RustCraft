use crate::{utils::*, *};

#[derive(Debug, Clone, Copy)]
pub struct Mat3x2<T> {
    pub(crate) data: [[T; 2]; 3],
}

impl<T: From<f32>> Mat3x2<T> {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into()],
            ],
        }
    }
}

impl<T: From<f32> + Copy> Default for Mat3x2<T> {
    fn default() -> Self {
        Self {
            data: [[0.0.into(); 2]; 3],
        }
    }
}

impl SetUniform for Mat3x2<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x2fv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl SetUniform for Mat3x2<f64> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x2dv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl<T> From<[[T; 2]; 3]> for Mat3x2<T> {
    fn from(value: [[T; 2]; 3]) -> Self {
        Self { data: value }
    }
}

impl_mat_ops_add!(Mat3x2<T>, 3, 2);
impl_mat_ops_sub!(Mat3x2<T>, 3, 2);
impl_mat_ops_mul_number!(Mat3x2<T>, 3, 2);
impl_mat_index!(Mat3x2<T>, 3, 2);
impl_mat_ops_div_number!(Mat3x2<T>, 3, 2);
impl_mat_ops_neg!(Mat3x2<T>, 3, 2);
impl_mat_ops_add_assign!(Mat3x2<T>, 3, 2);
impl_mat_ops_sub_assign!(Mat3x2<T>, 3, 2);
impl_mat_ops_mul_assign_number!(Mat3x2<T>, 3, 2);
impl_mat_ops_div_assign_number!(Mat3x2<T>, 3, 2);
