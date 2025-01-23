use super::{Mat, SetUniform};

impl SetUniform for Mat<f64, 1, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform2dv(location, 1, self[0].as_ptr() as *const _);
        }
    }
}

impl SetUniform for Mat<f64, 1, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform3dv(location, 1, self[0].as_ptr() as *const _);
        }
    }
}

impl SetUniform for Mat<f64, 1, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::Uniform4dv(location, 1, self[0].as_ptr() as *const _);
        }
    }
}

impl SetUniform for Mat<f64, 2, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f64, 3, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f64, 4, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f64, 2, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2x3dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f64, 2, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix2x4dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f64, 3, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x2dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f64, 3, 4> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3x4dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f64, 4, 2> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x2dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}

impl SetUniform for Mat<f64, 4, 3> {
    fn give(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4x3dv(location, 1, gl::TRUE, self[0].as_ptr());
        }
    }
}
