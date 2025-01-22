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
    /// 应用着色器程序
    pub fn use_program(&self) {
        unsafe {
            gl_utils::use_program(self.program);
        }
    }

    /// 向着色器程序中设置uniform变量
    pub fn set_uniform<T: SetUniform>(&self, name: &str, value: T) {
        if let Err(e) = unsafe { gl_utils::set_uniform(self.program, name, value) } {
            warn!("Shader", "{}", e);
        };
    }
}
