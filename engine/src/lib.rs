use std::ffi::CString;

use vertex_attrib::VertexAttribPointers;

mod buffer;
pub mod element;
pub mod engine;
pub mod program;
mod shader;
mod texture;
mod vertex_arrray;
pub use sdl2::event;

fn whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<_> = Vec::with_capacity(len as usize + 1);
    // Fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}

#[repr(C)]
#[derive(VertexAttribPointers)]
pub struct Vertex {
    #[location = 0]
    pub pos: (f32, f32, f32),
}

#[repr(C)]
#[derive(VertexAttribPointers)]
pub struct TextureVertex {
    #[location = 0]
    pub pos: (f32, f32, f32),
    #[location = 1]
    pub tex_coords: (f32, f32),
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
