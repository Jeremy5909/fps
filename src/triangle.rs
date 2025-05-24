use crate::{
    buffer::{ArrayBuffer, Vertex},
    program::Program,
    vertex_arrray::VertexArray,
};

pub struct Triangle {
    program: Program,
    _vbo: ArrayBuffer,
    vao: VertexArray,
}

impl Triangle {
    pub fn new() -> Result<Triangle, String> {
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
        Ok(Triangle {
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
