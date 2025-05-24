use std::marker;

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

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLuint;
}

pub struct Buffer<B: BufferType> {
    vbo: gl::types::GLuint,
    _marker: marker::PhantomData<B>,
}
impl<B: BufferType> Buffer<B> {
    pub fn new() -> Buffer<B> {
        let mut id: gl::types::GLuint = 0;
        unsafe { gl::GenBuffers(1, &mut id) };
        Self {
            vbo: id,
            _marker: marker::PhantomData,
        }
    }
    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
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
impl<B> Drop for Buffer<B>
where
    B: BufferType,
{
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &mut self.vbo);
        }
    }
}

pub struct BufferTypeArray;
impl BufferType for BufferTypeArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ARRAY_BUFFER;
}
pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLuint = gl::ELEMENT_ARRAY_BUFFER;
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementArrayBuffer = Buffer<BufferTypeElementArray>;
