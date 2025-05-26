use image::{EncodableLayout, ImageError};

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
    pub fn load(&self, name: &str) -> Result<(), ImageError> {
        self.bind();

        let img = image::open(name)?.into_rgb8();
        unsafe {
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                img.as_bytes().as_ptr() as *const _,
            )
        };
        Ok(())
    }
}
impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &mut self.id);
        }
    }
}
