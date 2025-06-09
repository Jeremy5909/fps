use std::ptr;

use nalgebra::Matrix4;
use vertex_attrib::VertexAttribPointers;

use crate::{
    buffer::{ArrayBuffer, ElementBuffer},
    camera::Camera,
    program::Program,
    texture::Texture,
    vertex_arrray::VertexArray,
};
pub struct Element {
    program: Program,
    vao: VertexArray,
    textures: Vec<Texture>,
    index_count: i32,
}

impl Element {
    pub fn new<V: VertexAttribPointers>(
        vertices: Vec<V>,
        indices: Vec<i32>,
        shader_program: Program,
    ) -> Result<Self, String> {
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
        V::vertex_attrib_pointers();
        vao.unbind();
        vbo.unbind();
        ebo.unbind();

        Ok(Self {
            program: shader_program,
            vao,
            textures: Vec::new(),
            index_count: indices.len() as i32,
        })
    }
    pub fn add_texture(&mut self, texture_path: &str) -> Result<(), String> {
        let texture = Texture::new();
        texture.load(texture_path).map_err(|e| e.to_string())?;
        self.textures.push(texture);
        Ok(())
    }
    pub(crate) fn render(&self, camera: &Camera) {
        self.program.set_used();
        self.vao.bind();

        self.textures.iter().for_each(|texture| texture.bind());

        self.program
            .set_uniform_matrix4("projection", &camera.projection);
        self.program.set_uniform_matrix4("view", &camera.view);
        self.program
            .set_uniform_matrix4("model", &Matrix4::identity());

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}

pub mod primitives {
    pub mod textured_cube {
        use crate::TextureVertex;

        pub fn verts() -> Vec<TextureVertex> {
            vec![
                TextureVertex {
                    pos: (-0.5, 0.5, -0.5).into(),
                    tex_coords: (0.0, 1.0).into(),
                },
                TextureVertex {
                    pos: (0.5, 0.5, -0.5).into(),
                    tex_coords: (1.0, 1.0).into(),
                },
                TextureVertex {
                    pos: (0.5, -0.5, -0.5).into(),
                    tex_coords: (1.0, 0.0).into(),
                },
                TextureVertex {
                    pos: (-0.5, -0.5, -0.5).into(),
                    tex_coords: (0.0, 0.0).into(),
                },
                TextureVertex {
                    pos: (-0.5, 0.5, 0.5).into(),
                    tex_coords: (0.0, 1.0).into(),
                },
                TextureVertex {
                    pos: (0.5, 0.5, 0.5).into(),
                    tex_coords: (1.0, 1.0).into(),
                },
                TextureVertex {
                    pos: (0.5, -0.5, 0.5).into(),
                    tex_coords: (1.0, 0.0).into(),
                },
                TextureVertex {
                    pos: (-0.5, -0.5, 0.5).into(),
                    tex_coords: (0.0, 0.0).into(),
                },
            ]
        }
        pub fn indices() -> Vec<i32> {
            vec![
                0, 1, 2, 0, 2, 3, // Back
                4, 5, 6, 4, 6, 7, // Front
                0, 3, 4, 3, 4, 7, // Left
                1, 2, 5, 2, 5, 6, // Right
                0, 1, 4, 1, 4, 5, // Top
                2, 3, 7, 2, 6, 7, // Bottom
            ]
        }
    }
}
