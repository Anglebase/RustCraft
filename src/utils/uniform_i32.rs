use super::{Mat, SetUniform};

impl SetUniform for Mat<i32, 1, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2iv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<i32, 1, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3iv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<i32, 1, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4iv(location, 1, self[0].as_ptr());
        }
    }
}
