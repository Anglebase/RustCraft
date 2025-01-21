use gl::types::*;
use std::ffi::CString;

pub unsafe fn complie_shader(shader_type: GLenum, source: &str) -> Result<GLuint, String> {
    let shader = gl::CreateShader(shader_type);
    let source = CString::new(source).unwrap();
    gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
    gl::CompileShader(shader);
    let mut status = gl::FALSE as GLint;
    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    if status != gl::TRUE as GLint {
        let mut len = 0;
        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer = Vec::with_capacity(len as usize);
        buffer.set_len(len as usize);
        gl::GetShaderInfoLog(
            shader,
            len,
            std::ptr::null_mut(),
            buffer.as_mut_ptr() as *mut GLchar,
        );
        let error = String::from_utf8(buffer).unwrap();
        gl::DeleteShader(shader);
        return Err(error);
    }
    Ok(shader)
}

pub unsafe fn link_program(vshader: GLuint, fshader: GLuint) -> Result<GLuint, String> {
    let program = gl::CreateProgram();
    gl::AttachShader(program, vshader);
    gl::AttachShader(program, fshader);
    gl::LinkProgram(program);
    let mut status = gl::FALSE as GLint;
    gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    if status != gl::TRUE as GLint {
        let mut len = 0;
        gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        let mut buffer = Vec::with_capacity(len as usize);
        buffer.set_len(len as usize);
        gl::GetProgramInfoLog(
            program,
            len,
            std::ptr::null_mut(),
            buffer.as_mut_ptr() as *mut GLchar,
        );
        let error = String::from_utf8(buffer).unwrap();
        gl::DeleteProgram(program);
        return Err(error);
    }
    gl::DeleteShader(vshader);
    gl::DeleteShader(fshader);
    Ok(program)
}

pub unsafe fn use_program(program: GLuint) {
    gl::UseProgram(program);
}