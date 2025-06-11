use crate::{DiffuseVertex, element::Element};

impl<'a> Element<'a> {
    pub fn from_obj(file_name: &str, textures_path: &str) -> Result<Vec<Element<'a>>, String> {
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

            let mesh_verts: Vec<_> = model
                .mesh
                .positions
                .chunks_exact(3)
                .map(|i| (i[0], i[1], i[2]))
                .collect();
            let mesh_tex: Vec<_> = model
                .mesh
                .texcoords
                .chunks_exact(2)
                .map(|i| (i[0], i[1]))
                .collect();
            let mesh_norms: Vec<_> = model
                .mesh
                .normals
                .chunks_exact(3)
                .map(|i| (i[0], i[1], i[2]))
                .collect();

            let vertexes: Vec<_> = mesh_verts
                .iter()
                .zip(&mesh_tex)
                .zip(&mesh_norms)
                .map(|((a, b), c)| ((*a).into(), (*b).into(), (*c).into()))
                .collect();
            let vertexes: Vec<_> = vertexes
                .iter()
                .map(|(vert, tex_coord, normals)| DiffuseVertex {
                    // Vertex should be generic or something
                    // so it can have any vertex
                    pos: *vert,
                    tex_coord: *tex_coord,
                    normal: *normals,
                    color: (1.0, 1.0, 1.0).into(),
                })
                .collect();

            let id = model.mesh.material_id;
            let mut element = Element::new(vertexes, model.mesh)?;
            if let Some(id) = id {
                let material = materials
                    .get(id)
                    .ok_or(String::from("Material not found"))?;
                if let Some(diffuse_texture_name) = &material.diffuse_texture {
                    eprintln!("Loading texture '{}'", diffuse_texture_name);
                    element.add_texture(&format!("{textures_path}/{diffuse_texture_name}"))?;
                }
            }
            elements.push(element);
        }

        Ok(elements)
    }
    pub fn from_fbx(file_name: &str) -> Result<Vec<Element>, String> {
        todo!("{file_name}");
    }
}
