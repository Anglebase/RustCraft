use gl::types::*;
use std::ffi::CString;

use crate::utils::SetUniform;

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

pub unsafe fn create_model_context(
    vertices: &Vec<f32>,
    indices: &Vec<u32>,
    description: &str,
) -> (GLuint, GLuint, GLuint) {
    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;
    let mut ebo: GLuint = 0;
    gl::GenVertexArrays(1, &mut vao);
    gl::GenBuffers(1, &mut vbo);
    gl::GenBuffers(1, &mut ebo);
    gl::BindVertexArray(vao);
    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BufferData(
        gl::ARRAY_BUFFER,
        (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
        vertices.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW,
    );
    gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
    gl::BufferData(
        gl::ELEMENT_ARRAY_BUFFER,
        (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
        indices.as_ptr() as *const GLvoid,
        gl::STATIC_DRAW,
    );
    let mut offset = 0;
    let mut layout = vec![];
    for desc in description.split(';') {
        let type_id = String::from(&desc[desc.len() - 1..]);
        let size = desc[..desc.len() - 1].parse::<GLint>().unwrap();
        let type_id = match type_id.as_str() {
            "f" => gl::FLOAT,
            "i" => gl::INT,
            "b" => gl::BOOL,
            _ => panic!("Invalid type id"),
        };
        layout.push((type_id, size, offset));
        offset += match type_id {
            gl::FLOAT => size as i32 * std::mem::size_of::<GLfloat>() as i32,
            gl::INT => size as i32 * std::mem::size_of::<GLint>() as i32,
            gl::BOOL => size as i32 * std::mem::size_of::<GLboolean>() as i32,
            _ => panic!("Invalid type id"),
        };
    }
    let sum = offset;
    for (i, &(type_id, size, offset)) in layout.iter().enumerate() {
        gl::VertexAttribPointer(
            i as GLuint,
            size,
            type_id,
            gl::FALSE,
            sum as GLsizei,
            offset as *const GLvoid,
        );
        gl::EnableVertexAttribArray(i as GLuint);
    }

    gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    gl::BindVertexArray(0);
    (vao, vbo, ebo)
}

pub unsafe fn load_texture_from_file(path: &str) -> Result<GLuint, Box<dyn std::error::Error>> {
    use image::*;
    let img = open(path)?.to_rgba8();
    let (width, height) = img.dimensions();
    let data = img.into_raw();

    let mut texture: GLuint = 0;
    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGBA as i32,
        width as i32,
        height as i32,
        0,
        gl::RGBA,
        gl::UNSIGNED_BYTE,
        data.as_ptr() as *const GLvoid,
    );
    gl::GenerateMipmap(gl::TEXTURE_2D);
    gl::BindTexture(gl::TEXTURE_2D, 0);

    Ok(texture)
}

pub unsafe fn set_uniform<T: SetUniform>(
    program: GLuint,
    name: &str,
    value: T,
) -> Result<(), String> {
    let c_name = match CString::new(name) {
        Ok(name) => name,
        Err(e) => return Err(format!("无效的 uniform 名称: {}", e)),
    };
    let local = gl::GetUniformLocation(program, c_name.as_ptr());
    if local == -1 {
        return Err(format!("未找到 uniform 变量: {}", name));
    }
    value.give(local);
    let err = gl::GetError();
    match err {
        gl::NO_ERROR => Ok(()),
        gl::INVALID_OPERATION => Err(format!("此次设置 uniform 变量 {} 为无效操作", name)),
        _ => Err(format!("未能设置 uniform 变量: {}, 错误码: {}", name, err)),
    }
}
