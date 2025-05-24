use std::ptr;

use crate::{
    buffer::{ArrayBuffer, ElementBuffer},
    program::Program,
    vertex_arrray::VertexArray,
};
use vertex_derive::VertexAttribPointers;

#[repr(C)]
#[derive(VertexAttribPointers)]
pub struct Vertex {
    #[location = 0]
    pub pos: (f32, f32, f32),
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

pub struct Square {
    program: Program,
    _vbo: ArrayBuffer,
    vao: VertexArray,
}

impl Square {
    pub fn new() -> Result<Self, String> {
        let program = Program::from_name("shaders/square")?;
        let vertices = vec![
            Vertex {
                pos: (-0.5, 0.5, 0.0),
            },
            Vertex {
                pos: (0.5, 0.5, 0.0),
            },
            Vertex {
                pos: (0.5, -0.5, 0.0),
            },
            Vertex {
                pos: (-0.5, -0.5, 0.0),
            },
        ];

        let indices = vec![0, 1, 2, 2, 3, 0];

        let ebo = ElementBuffer::new();
        ebo.bind();
        ebo.static_draw_data(&indices);
        ebo.unbind();
        let vbo = ArrayBuffer::new();
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();
        let vao = VertexArray::new();
        vao.bind();
        vbo.bind();
        ebo.bind();
        Vertex::vertex_attrib_pointers();
        vao.unbind();
        vbo.unbind();
        ebo.unbind();

        Ok(Self {
            program,
            _vbo: vbo,
            vao,
        })
    }
    pub fn render(&self) {
        self.program.set_used();
        self.vao.bind();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
