use crate::{utils::*, *};

#[derive(Debug, Clone, Copy)]
pub struct Mat4x2<T> {
    pub(crate) data: [[T; 2]; 4],
}

impl<T: From<f32>> Mat4x2<T> {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into()],
            ],
        }
    }
}

impl<T: From<f32> + Copy> Default for Mat4x2<T> {
    fn default() -> Self {
        Self {
            data: [[0.0.into(); 2]; 4],
        }
    }
}

impl SetUniform for Mat4x2<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x2fv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl SetUniform for Mat4x2<f64> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x2dv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl<T> From<[[T; 2]; 4]> for Mat4x2<T> {
    fn from(value: [[T; 2]; 4]) -> Self {
        Self { data: value }
    }
}

impl_mat_ops_add!(Mat4x2<T>, 4, 2);
impl_mat_ops_sub!(Mat4x2<T>, 4, 2);
impl_mat_ops_mul_number!(Mat4x2<T>, 4, 2);
impl_mat_index!(Mat4x2<T>, 4, 2);
impl_mat_ops_div_number!(Mat4x2<T>, 4, 2);
impl_mat_ops_neg!(Mat4x2<T>, 4, 2);
impl_mat_ops_add_assign!(Mat4x2<T>, 4, 2);
impl_mat_ops_sub_assign!(Mat4x2<T>, 4, 2);
impl_mat_ops_mul_assign_number!(Mat4x2<T>, 4, 2);
impl_mat_ops_div_assign_number!(Mat4x2<T>, 4, 2);
