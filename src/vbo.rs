pub struct VBO {
    vertices: Vec<f32>,
    id: gl::types::GLuint,
}
impl VBO {
    pub fn from_verts(vertices: Vec<f32>) -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id) };
        Self { vertices, id }
    }
    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                self.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    }
}
