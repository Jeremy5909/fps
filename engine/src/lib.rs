use std::ffi::CString;

use vertex_attrib::VertexAttribPointers;

mod buffer;
pub mod camera;
pub mod element;
pub mod engine;
pub mod hooks;
pub mod program;
mod shader;
mod texture;
mod vertex_arrray;
pub use nalgebra::*;
pub use sdl2::event;
pub use sdl2::keyboard;
pub use sdl2::mouse;

fn whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<_> = Vec::with_capacity(len as usize + 1);
    // Fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}

// Even more temporary
trait VertexAttrib {
    const COMPONENTS: i32;
    unsafe fn vertex_attrib_pointer(stride: usize, location: usize, offset: usize) {
        unsafe {
            gl::EnableVertexAttribArray(location as gl::types::GLuint);
            gl::VertexAttribPointer(
                location as gl::types::GLuint,
                Self::COMPONENTS,
                gl::FLOAT,
                gl::FALSE,
                stride as gl::types::GLint,
                offset as *const gl::types::GLvoid,
            );
        }
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Vec2(pub (f32, f32));
impl VertexAttrib for Vec2 {
    const COMPONENTS: i32 = 2;
}
impl From<(f32, f32)> for Vec2 {
    fn from(value: (f32, f32)) -> Self {
        Self(value)
    }
}
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Vec3(pub (f32, f32, f32));
impl VertexAttrib for Vec3 {
    const COMPONENTS: i32 = 3;
}
impl From<(f32, f32, f32)> for Vec3 {
    fn from(value: (f32, f32, f32)) -> Self {
        Self(value)
    }
}

#[repr(C)]
#[derive(VertexAttribPointers)]
pub struct Vertex {
    #[location = 0]
    pub pos: Vec3,
}

#[repr(C)]
#[derive(VertexAttribPointers)]
pub struct TextureVertex {
    #[location = 0]
    pub pos: Vec3,
    #[location = 1]
    pub tex_coords: Vec2,
}
