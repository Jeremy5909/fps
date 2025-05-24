use crate::{buffer::ArrayBuffer, program::Program, vertex_arrray::VertexArray};
use vertex_derive::VertexAttribPointers;

#[repr(C)]
#[derive(VertexAttribPointers)]
pub struct Vertex {
    #[location = 0]
    pub pos: (f32, f32, f32),
    #[location = 1]
    pub col: (f32, f32, f32),
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

pub struct Triangle {
    program: Program,
    _vbo: ArrayBuffer,
    vao: VertexArray,
}

impl Triangle {
    pub fn new() -> Result<Self, String> {
        let program = Program::from_name("triangle")?;
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

        let vao = VertexArray::new();
        vao.bind();
        vbo.bind();
        Vertex::vertex_attrib_pointers();
        vbo.unbind();
        vao.unbind();
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
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}
