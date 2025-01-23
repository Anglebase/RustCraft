use crate::gl_utils;
use gl::types::*;
use json::JsonValue;

use super::Model;

pub struct ArrayModel {
    vertices: Vec<f32>,
    vao: GLuint,
    vbo: GLuint,
}

impl ArrayModel {
    pub fn new(vertices: Vec<f32>, description: &str) -> Self {
        let mut ret = Self {
            vertices,
            vao: 0,
            vbo: 0,
        };
        let (vao, vbo) =
            unsafe { gl_utils::create_array_model_context(&ret.vertices, description) };
        ret.vao = vao;
        ret.vbo = vbo;
        ret
    }
}

impl Model for ArrayModel {
    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, (self.vertices.len() / 3) as i32);
        }
    }
}

impl Drop for ArrayModel {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

impl ArrayModel {
    pub fn load_from_json(json: &JsonValue) -> Result<(String, Vec<f32>, String), String> {
        if !json.has_key("name") {
            return Err("JSON 中缺少 name 字段".to_string());
        }
        if !json.has_key("description") {
            return Err("JSON 中缺少 description 字段".to_string());
        }
        if !json.has_key("vertices") {
            return Err("JSON 中缺少 vertices 字段".to_string());
        }
        let name = if let Some(name) = json["name"].as_str() {
            name.to_string()
        } else {
            return Err("JSON 中 name 字段无效".to_string());
        };
        let description = if let Some(description) = json["description"].as_str() {
            description.to_string()
        } else {
            return Err("JSON 中 description 字段无效".to_string());
        };
        let vertices = &json["vertices"];
        let mut ret_vertices = vec![];
        if !vertices.is_array() {
            return Err("JSON 中 vertices 字段不是数组".to_string());
        }
        for vertex in vertices.members() {
            if vertex.is_number() {
                ret_vertices.push(vertex.as_f32().unwrap());
            } else {
                return Err("JSON 中 vertices 字段数组元素不是数字".to_string());
            }
        }
        Ok((name, ret_vertices, description))
    }
}
