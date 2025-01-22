use crate::utils::SetUniform;

pub struct Mat2<T> {
    data: [[T; 2]; 2],
}

impl<T: From<f32>> Mat2<T> {
    pub fn new() -> Self {
        Self {
            data: [[1.0.into(), 0.0.into()], [0.0.into(), 1.0.into()]],
        }
    }

    pub fn zero() -> Self {
        Self {
            data: [[0.0.into(), 0.0.into()], [0.0.into(), 0.0.into()]],
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