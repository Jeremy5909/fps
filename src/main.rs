use std::{ffi::CString, os::raw::c_void};

use color_buffer::ColorBuffer;
use element::Element;
use program::Program;
use sdl2::event::Event;
use vertex_attrib::VertexAttribPointers;
use viewport::Viewport;

mod buffer;
mod color_buffer;
mod element;
mod program;
mod shader;
mod texture;
mod triangle;
mod vertex_arrray;
mod viewport;

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
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 1);

    let window = video
        .window("fps", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const c_void);

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
    let mut viewport = Viewport::for_window(900, 800);
    let color_buffer = ColorBuffer::from_color(nalgebra::Vector3::new(0.3, 0.5, 1.0));

    color_buffer.set_used();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used();
                }
                _ => {}
            }
        }
        color_buffer.clear();
        square.render();
        window.gl_swap_window();
    }
}

fn whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<_> = Vec::with_capacity(len as usize + 1);
    // Fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
