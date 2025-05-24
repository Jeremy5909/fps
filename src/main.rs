use std::{ffi::CString, os::raw::c_void};

use sdl2::event::Event;
use triangle::Triangle;

mod buffer;
mod program;
mod shader;
mod triangle;
mod vertex_arrray;

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

    let triangle = Triangle::new().unwrap();

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
            triangle.render();
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
