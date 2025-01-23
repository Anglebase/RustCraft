use super::{Mat, SetUniform};

impl SetUniform for Mat<u32, 1, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2uiv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<u32, 1, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3uiv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<u32, 1, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4uiv(location, 1, self[0].as_ptr());
        }
    }
}
