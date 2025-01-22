use gl::types::*;

use crate::{
    gl_utils::{self},
    utils::SetUniform,
    warn,
};

pub struct Shader {
    pub(crate) program: GLuint,
}

impl Shader {
    pub fn use_program(&self) {
        unsafe {
            gl_utils::use_program(self.program);
        }
    }

    pub fn set_uniform<T: SetUniform>(&self, name: &str, value: T) {
        if let Err(e) = unsafe { gl_utils::set_uniform(self.program, name, value) } {
            warn!("Shader", "{}", e);
        };
    }
}
