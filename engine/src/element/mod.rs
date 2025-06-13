use std::ptr;

use nalgebra::{Matrix4, Vector3};
use rapier3d::prelude::{Collider, ColliderHandle, RigidBody, RigidBodyHandle};
use tobj::Mesh;
use vertex_attrib::VertexAttribPointers;

mod from_file;
pub mod physics;

use crate::{
    buffer::{ArrayBuffer, ElementBuffer},
    camera::Camera,
    program::Program,
    texture::Texture,
    vertex_arrray::VertexArray,
};
pub struct Element<'a> {
    program: Option<&'a Program>,
    vao: VertexArray,
    texture: Option<Texture>,
    index_count: usize,
    pub model: Matrix4<f32>,
    pub(crate) collider: Option<Collider>,
    pub(crate) rigid_body: Option<RigidBody>,
    pub(crate) rigid_body_handle: Option<RigidBodyHandle>,
    pub(crate) collider_handle: Option<ColliderHandle>,
    mesh: Mesh,
}

impl<'a> Element<'a> {
    pub fn new<V: VertexAttribPointers>(vertices: Vec<V>, mesh: Mesh) -> Result<Self, String> {
        let ebo = ElementBuffer::new();
        ebo.bind();
        ebo.static_draw_data(&mesh.indices);
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
            model: Matrix4::identity(),
            index_count: mesh.indices.len(),
            vao,
            mesh,
            collider: None,
            rigid_body: None,
            program: None,
            texture: None,
            rigid_body_handle: None,
            collider_handle: None,
        })
    }

    pub fn position(&self) -> Vector3<f32> {
        self.model.column(3).xyz()
    }
    pub fn scale(&self) -> Vector3<f32> {
        Vector3::from_iterator(
            self.model
                .fixed_view::<3, 3>(0, 0)
                .column_iter()
                .map(|col| col.norm()),
        )
    }

    pub(crate) fn render(&self, camera: &Camera) {
        self.vao.bind();

        if let Some(texture) = &self.texture {
            texture.bind();
        } else {
            Texture::unbind();
        }
        self.texture.as_ref().inspect(|texture| texture.bind());

        if let Some(program) = &self.program {
            program.set_uniform("projection", &camera.projection);
            program.set_uniform("view", &camera.view);
            program.set_uniform("model", &self.model);
        }

        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as i32,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
        }
    }
    pub fn add_texture(&mut self, texture_path: &str) -> Result<(), String> {
        let texture = Texture::new();
        let file_type = texture_path
            .split(|x| x == '.')
            .skip(1)
            .next()
            .ok_or("Must have file type")?;
        match file_type {
            "jpg" => texture.load_jpg(texture_path).map_err(|e| e.to_string())?,
            "png" => texture.load_png(texture_path).map_err(|e| e.to_string())?,
            _ => return Err(String::from("Unkown file type")),
        }
        self.texture = Some(texture);
        Ok(())
    }
    pub fn add_program(&mut self, program: &'a Program) -> Result<(), String> {
        self.program = Some(&program);
        Ok(())
    }
}
