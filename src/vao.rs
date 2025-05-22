use std::os::raw::c_void;

pub struct VAO {
    id: gl::types::GLuint,
}
impl VAO {
    pub fn new() -> Self {
        unsafe {
            let mut id: gl::types::GLuint = 0;
            gl::GenVertexArrays(1, &mut id);
            Self { id }
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }
    pub fn link_attribute(
        &self,
        vbo: gl::types::GLuint,
        layout: gl::types::GLuint,
        num_components: gl::types::GLint,
        data_type: gl::types::GLenum,
        stride: gl::types::GLint,
        offset: *const c_void,
    ) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::EnableVertexAttribArray(layout);
            gl::VertexAttribPointer(layout, num_components, data_type, gl::FALSE, stride, offset);
        }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindVertexArray(0) };
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
