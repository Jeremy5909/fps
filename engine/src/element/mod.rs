use std::ptr;

use nalgebra::Matrix4;
use vertex_attrib::VertexAttribPointers;

mod from_file;
mod funcs;

use crate::{
    buffer::{ArrayBuffer, ElementBuffer},
    camera::Camera,
    program::Program,
    texture::Texture,
    vertex_arrray::VertexArray,
};
pub struct Element {
    program: Option<Program>,
    vao: VertexArray,
    textures: Vec<Texture>,
    index_count: i32,
    pub model: Matrix4<f32>,
}

impl Element {
    pub fn new<V: VertexAttribPointers>(
        vertices: Vec<V>,
        indices: Vec<i32>,
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
            program: None,
            vao,
            textures: Vec::new(),
            index_count: indices.len() as i32,
            model: Matrix4::identity(),
        })
    }
    pub(crate) fn render(&self, camera: &Camera) {
        self.vao.bind();

        self.textures.iter().for_each(|texture| texture.bind());

        if let Some(program) = &self.program {
            program.set_uniform_matrix4("projection", &camera.projection);
            program.set_uniform_matrix4("view", &camera.view);
            program.set_uniform_matrix4("model", &self.model);
        }

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
}
