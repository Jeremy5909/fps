use std::{ffi::CString, os::raw::c_void};

use program::Program;
use sdl2::event::Event;
use shader::Shader;
use vao::VAO;
use vbo::{VBO, Vertex};

mod program;
mod shader;
mod vao;
mod vbo;

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

    let vbo = VBO::from_vertices(vec![
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
    ]);
    vbo.bind();

    let vao = VAO::new();
    vao.bind();

    vao.link_attribute(
        vbo.id(),
        0,
        3,
        gl::FLOAT,
        (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
        std::ptr::null(),
    );
    vao.link_attribute(
        vbo.id(),
        1,
        3,
        gl::FLOAT,
        (6 * std::mem::size_of::<f32>()) as gl::types::GLint,
        (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
    );
    vao.unbind();
    vbo.unbind();

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
            gl::BindVertexArray(vao.id());
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
