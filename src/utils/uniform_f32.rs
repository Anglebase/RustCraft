use super::{Mat, SetUniform};

impl SetUniform for Mat<f32, 1, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2fv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 1, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3fv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 1, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4fv(location, 1, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 2, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 3, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 4, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 2, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2x3fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 2, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2x4fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 3, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x2fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 3, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x4fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 4, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x2fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f32, 4, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x3fv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}
