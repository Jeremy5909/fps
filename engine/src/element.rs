use std::ptr;

use nalgebra::Matrix4;
use vertex_attrib::VertexAttribPointers;

use crate::{
    TextureVertex,
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
    pub model: Matrix4<f32>,
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
            model: Matrix4::identity(),
        })
    }
    pub fn from_obj(file_name: &str) -> Result<Vec<Element>, String> {
        let load_options = tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };
        let (models, materials) = tobj::load_obj(file_name, &load_options)
            .map_err(|_| String::from("Error loading object"))?;
        let materials = materials.map_err(|_| String::from("Error loading material"))?;

        let mut elements = Vec::new();
        for model in models {
            eprintln!("model '{}' loaded", model.name);
            let mesh = model.mesh;

            let verts: Vec<_> = mesh
                .positions
                .chunks_exact(3)
                .map(|i| (i[0], i[1], i[2]).into())
                .collect();
            let tex_coords: Vec<_> = mesh
                .texcoords
                .chunks_exact(2)
                .map(|i| (i[0], i[1]).into())
                .collect();
            let indices = mesh.indices.iter().map(|i| *i as i32).collect();

            let verts: Vec<_> = verts.iter().zip(&tex_coords).collect();
            let verts: Vec<_> = verts
                .iter()
                .map(|(vert, tex_coord)| TextureVertex {
                    pos: **vert,
                    tex_coords: **tex_coord,
                })
                .collect();

            let mut element =
                Element::new(verts, indices, Program::from_name("shaders/textured")?)?;
            if let Some(id) = mesh.material_id {
                let material = materials
                    .get(id)
                    .ok_or(String::from("Material not found"))?;
                if let Some(diffuse_texture_name) = &material.diffuse_texture {
                    eprintln!("Loading texture '{}'", diffuse_texture_name);
                    element.add_texture(&format!("textures/{diffuse_texture_name}"))?;
                }
            }
            elements.push(element);
        }

        Ok(elements)
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
        self.program.set_uniform_matrix4("model", &self.model);

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
