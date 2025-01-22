use crate::{
    impl_mat_index, impl_mat_mul_mat, impl_mat_ops_add, impl_mat_ops_mul_number, impl_mat_ops_sub,
    utils::SetUniform,
};

#[derive(Debug, Clone, Copy)]
pub struct Mat3<T> {
    pub(crate) data: [[T; 3]; 3],
}

impl<T: From<f32>> Mat3<T> {
    pub fn new() -> Self {
        Self {
            data: [
                [1.0.into(), 0.0.into(), 0.0.into()],
                [0.0.into(), 1.0.into(), 0.0.into()],
                [0.0.into(), 0.0.into(), 1.0.into()],
            ],
        }
    }
}

impl<T: From<f32> + Copy> Default for Mat3<T> {
    fn default() -> Self {
        Self {
            data: [[0.0.into(); 3]; 3],
        }
    }
}

impl SetUniform for Mat3<f32> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl SetUniform for Mat3<f64> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3dv(location, 1, gl::TRUE, &self.data[0][0]);
        }
    }
}

impl<T> From<[[T; 3]; 3]> for Mat3<T> {
    fn from(value: [[T; 3]; 3]) -> Self {
        Self { data: value }
    }
}

impl_mat_ops_add!(Mat3<T>, 3, 3);
impl_mat_ops_sub!(Mat3<T>, 3, 3);
impl_mat_ops_mul_number!(Mat3<T>, 3, 3);
impl_mat_mul_mat!(Mat3<T>, 3, 3);
impl_mat_index!(Mat3<T>, 3, 3);
