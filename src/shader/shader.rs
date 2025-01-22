use gl::types::*;

use super::gl_utils;

pub struct Shader {
    pub(crate) program: GLuint,
}

impl Shader {
    pub fn use_program(&self) {
        unsafe {
            gl_utils::use_program(self.program);
        }
    }
}
