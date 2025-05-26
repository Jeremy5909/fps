use std::marker;

pub trait BufferType {
    const BUFFER_TYPE: gl::types::GLenum;
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
            gl::BindBuffer(B::BUFFER_TYPE, self.vbo);
        }
    }
    pub fn static_draw_data<T>(&self, data: &[T]) {
        unsafe {
            gl::BufferData(
                B::BUFFER_TYPE,
                (data.len() * std::mem::size_of::<T>()) as gl::types::GLsizeiptr,
                data.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            )
        };
    }
    pub fn unbind(&self) {
        unsafe { gl::BindBuffer(B::BUFFER_TYPE, 0) };
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
    const BUFFER_TYPE: gl::types::GLenum = gl::ARRAY_BUFFER;
}
pub struct BufferTypeElementArray;
impl BufferType for BufferTypeElementArray {
    const BUFFER_TYPE: gl::types::GLenum = gl::ELEMENT_ARRAY_BUFFER;
}

pub type ArrayBuffer = Buffer<BufferTypeArray>;
pub type ElementBuffer = Buffer<BufferTypeElementArray>;
