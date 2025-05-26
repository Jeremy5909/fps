use std::ffi::CString;

use element::Element;
use engine::Engine;
use program::Program;
use sdl2::event::Event;
use vertex_attrib::VertexAttribPointers;

mod buffer;
mod element;
mod engine;
mod program;
mod shader;
mod texture;
mod triangle;
mod vertex_arrray;

#[repr(C)]
#[derive(VertexAttribPointers)]
pub struct Vertex {
    #[location = 0]
    pub pos: (f32, f32),
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

fn main() {
    let mut engine = Engine::new("fps").unwrap();
    let square = Element::new(
        vec![
            Vertex { pos: (-0.5, -0.5) },
            Vertex { pos: (0.5, -0.5) },
            Vertex { pos: (0.5, 0.5) },
            Vertex { pos: (-0.5, 0.5) },
        ],
        vec![0, 1, 2, 2, 3, 0],
        Program::from_name("shaders/white").unwrap(),
    )
    .unwrap();

    engine.clear_color(0.7, 0.5, 1.0);
    'main: loop {
        for event in engine.events() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    engine.update_size(w, h);
                }
                _ => {}
            }
        }
        engine.clear();
        square.render();
        engine.swap_window();
    }
}

fn whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<_> = Vec::with_capacity(len as usize + 1);
    // Fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
