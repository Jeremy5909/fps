mod file_types;

pub struct Texture {
    id: gl::types::GLuint,
}
impl Texture {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id) };
        Self { id }
    }
    pub fn bind(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.id) };
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
