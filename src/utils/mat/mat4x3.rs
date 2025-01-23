use crate::{utils::*, *};

#[derive(Debug, Clone, Copy)]
pub struct Mat4x3<T> {
    pub(crate) data: [[T; 3]; 4],
}

impl<T: From<f32>> Mat4x3<T> {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0.into(), 0.0.into(), 0.0.into()],
                [0.0.into(), 1.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into(), 1.0.into()],
                [0.0.into(), 0.0.into(), 0.0.into()],
            ],
        }
    }
}

impl<T: From<f32> + Copy> Default for Mat4x3<T> {
    fn default() -> Self {
        Self {
            data: [[0.0.into(); 3]; 4],
        }
    }
}

impl SetUniform for Mat4x3<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x3fv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl SetUniform for Mat4x3<f64> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x3dv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl<T> From<[[T; 3]; 4]> for Mat4x3<T> {
    fn from(value: [[T; 3]; 4]) -> Self {
        Self { data: value }
    }
}

impl_mat_ops_add!(Mat4x3<T>, 4, 3);
impl_mat_ops_sub!(Mat4x3<T>, 4, 3);
impl_mat_ops_mul_number!(Mat4x3<T>, 4, 3);
impl_mat_index!(Mat4x3<T>, 4, 3);
impl_mat_ops_div_number!(Mat4x3<T>, 4, 3);
impl_mat_ops_neg!(Mat4x3<T>, 4, 3);
impl_mat_ops_add_assign!(Mat4x3<T>, 4, 3);
impl_mat_ops_sub_assign!(Mat4x3<T>, 4, 3);
impl_mat_ops_mul_assign_number!(Mat4x3<T>, 4, 3);
impl_mat_ops_div_assign_number!(Mat4x3<T>, 4, 3);
