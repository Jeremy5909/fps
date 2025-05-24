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

pub struct ArrayBuffer {
    id: gl::types::GLuint,
}
impl ArrayBuffer {
    pub fn new() -> Self {
        let mut id: gl::types::GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id) };
        Self { id }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }
    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        };
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
    }
}
impl Drop for ArrayBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.id);
        }
    }
}
