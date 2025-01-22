use crate::{gl_utils, warn};
use gl::types::*;

pub struct Texture {
    pub(crate) id: GLuint,
}

impl Texture {
    pub fn new_from(path: &str) -> Option<Self> {
        let tex = match unsafe { gl_utils::load_texture_from_file(path) } {
            Ok(tex) => tex,
            Err(e) => {
                warn!(
                    "Texture::new_from",
                    "无法从文件 {} 加载纹理, 由于 {}", path, e
                );
                return None;
            }
        };
        Some(Self { id: tex })
    }

    pub fn bind(&self, id: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
