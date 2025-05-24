use vertex_derive::VertexAttribPointers;

#[repr(C)]
#[derive(VertexAttribPointers)]
pub struct Vertex {
    #[location = 0]
    pub pos: (f32, f32, f32),
    #[location = 1]
    pub col: (f32, f32, f32),
}
impl Vertex {
    fn flatten(&self) -> [f32; 6] {
        [
            self.pos.0, self.pos.1, self.pos.2, self.col.0, self.col.1, self.col.2,
        ]
    }
    pub unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        unsafe {
            gl::EnableVertexAttribArray(location as gl::types::GLuint);
            gl::VertexAttribPointer(
                location as gl::types::GLuint,
                3,
                gl::FLOAT,
                gl::FALSE,
                stride as gl::types::GLint,
                offset as *const gl::types::GLvoid,
            );
        }
    }
}

pub struct VBO {
    vertices: Vec<f32>,
    id: gl::types::GLuint,
}
impl VBO {
    pub fn from_vertices(vertices: Vec<Vertex>) -> Self {
        let verts: Vec<f32> = vertices.iter().flat_map(|v| v.flatten()).collect();
        Self::from_vec(verts)
    }
    fn from_vec(vertices: Vec<f32>) -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id) };
        Self { vertices, id }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as gl::types::GLsizeiptr,
                self.vertices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
    pub fn unbind() {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    }
}
