use gl::types::*;

pub struct Texture {
    pub(crate) id: GLuint,
}

impl Texture {
    pub fn new(path: &str) {}

    pub fn bind(&self, id: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        
    }
}