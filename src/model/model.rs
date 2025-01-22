use crate::shader::gl_utils;
use gl::types::*;

pub struct Model {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl Model {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>, description: &str) -> Self {
        let mut ret = Self {
            vertices,
            indices,
            vao: 0,
            vbo: 0,
            ebo: 0,
        };
        let (vao, vbo, ebo) =
            unsafe { gl_utils::create_model_context(&ret.vertices, &ret.indices, description) };
        ret.vao = vao;
        ret.vbo = vbo;
        ret.ebo = ebo;
        ret
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                0 as _,
            );
        }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
