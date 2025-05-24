use crate::array_buffer::Vertex;
use std::{ffi::CString, os::raw::c_void};

use array_buffer::ArrayBuffer;
use program::Program;
use sdl2::event::Event;

mod array_buffer;
mod program;
mod shader;

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

    let shader_program = Program::from_name("triangle").unwrap();
    shader_program.set_used();

    let vertices = vec![
        Vertex {
            pos: (0.5, -0.5, 0.0),
            col: (1.0, 0.0, 0.0),
        },
        Vertex {
            pos: (-0.5, -0.5, 0.0),
            col: (0.0, 1.0, 0.0),
        },
        Vertex {
            pos: (0.0, 0.5, 0.0),
            col: (0.0, 0.0, 1.0),
        },
    ];

    let vbo = ArrayBuffer::new();
    vbo.bind();
    vbo.static_draw_data(&vertices);
    vbo.unbind();

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);

        gl::BindVertexArray(vao);
        vbo.bind();
        Vertex::vertex_attrib_pointers();
        vbo.unbind();
        gl::BindVertexArray(0);
    }

    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.gl_swap_window();
    }
}

fn whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<_> = Vec::with_capacity(len as usize + 1);
    // Fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}
