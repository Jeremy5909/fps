use std::{ffi::CString, os::raw::c_void};

use program::Program;
use sdl2::event::Event;
use shader::Shader;
use vao::VAO;
use vbo::VBO;

mod program;
mod shader;
mod vao;
mod vbo;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video
        .window("fps", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const c_void);

    let vert_shader = Shader::from_source(
        &CString::new(include_str!("triangle.vert")).unwrap(),
        gl::VERTEX_SHADER,
    )
    .unwrap();
    let frag_shader = Shader::from_source(
        &CString::new(include_str!("triangle.frag")).unwrap(),
        gl::FRAGMENT_SHADER,
    )
    .unwrap();
    let shader_program = Program::from_shaders(&[vert_shader, frag_shader]).unwrap();
    shader_program.set_used();

    let vbo = VBO::from_verts(vec![
        -0.5, -0.5, 0.0, //
        0.5, -0.5, 0.0, //
        0.0, 0.5, 0.0, //
    ]);
    vbo.bind();

    let vao = VAO::new();
    vao.bind();

    vao.link_attribute(
        vbo.id(),
        0,
        3,
        gl::FLOAT,
        (3 * std::mem::size_of::<f32>()) as gl::types::GLint,
        std::ptr::null(),
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
