use std::ptr::null;

use gl::types::GLuint;

use crate::debug;
use crate::shader::*;
use crate::RustCraftWrapper;
use lazy_static::lazy_static;

lazy_static! {
    static ref MODEL: RustCraftWrapper<GLuint> = RustCraftWrapper::new(0);
}

pub fn init() {
    debug!("render::init()", "正在初始化着色器...");
    SHADER_MANAGER.load_from("shader/");

    const VERTEX_ARRAY: [f32; 12] = [
        0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0,
    ];
    const INDEX_ARRAY: [u32; 6] = [0, 1, 2, 2, 3, 0];

    unsafe {
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
            (VERTEX_ARRAY.len() * std::mem::size_of::<f32>()) as isize,
            &VERTEX_ARRAY[0] as *const f32 as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (INDEX_ARRAY.len() * std::mem::size_of::<u32>()) as isize,
            &INDEX_ARRAY[0] as *const u32 as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);

        MODEL.apply(|vao_| {
            *vao_ = vao;
        });
    }
}

pub fn render() {
    unsafe {
        gl::ClearColor(0.3, 0.5, 0.4, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        SHADER_MANAGER.use_program("test");
        MODEL.apply(|vao| {
            gl::BindVertexArray(*vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        });
    }
}
