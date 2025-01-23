use crate::gl_utils;
use gl::types::*;

use super::Model;

pub struct ElementModel {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
}

impl ElementModel {
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
}

impl Model for ElementModel {
    fn draw(&self) {
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

impl Drop for ElementModel {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

impl ElementModel {
    pub fn load_from_json(path: &str) -> Result<(String, Vec<f32>, Vec<u32>, String), String> {
        let string = match std::fs::read_to_string(path) {
            Ok(string) => string,
            Err(err) => return Err(format!("读取文件时错误: {}", err)),
        };
        let json = match json::parse(&string) {
            Ok(json) => json,
            Err(err) => return Err(format!("JSON 解析错误: {}", err)),
        };
        if !json.is_object() {
            return Err("JSON 不是对象".to_string());
        }
        if !json.has_key("name") {
            return Err("JSON 中缺少 name 字段".to_string());
        }
        if !json.has_key("description") {
            return Err("JSON 中缺少 description 字段".to_string());
        }
        if !json.has_key("vertices") {
            return Err("JSON 中缺少 vertices 字段".to_string());
        }
        if !json.has_key("indices") {
            return Err("JSON 中缺少 indices 字段".to_string());
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
        let indices = &json["indices"];
        let mut ret_vertices = vec![];
        let mut ret_indices = vec![];
        if !vertices.is_array() {
            return Err("JSON 中 vertices 字段不是数组".to_string());
        }
        if !indices.is_array() {
            return Err("JSON 中 indices 字段不是数组".to_string());
        }
        for vertex in vertices.members() {
            if vertex.is_number() {
                ret_vertices.push(vertex.as_f32().unwrap());
            } else {
                return Err("JSON 中 vertices 字段数组元素不是数字".to_string());
            }
        }
        for index in indices.members() {
            if index.is_number() {
                ret_indices.push(index.as_u32().unwrap());
            } else {
                return Err("JSON 中 indices 字段数组元素不是数字".to_string());
            }
        }
        Ok((name, ret_vertices, ret_indices, description))
    }
}
