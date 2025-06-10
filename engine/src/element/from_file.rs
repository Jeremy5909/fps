use crate::{DiffuseVertex, element::Element};

impl Element {
    pub fn from_obj(file_name: &str, textures_path: &str) -> Result<Vec<Element>, String> {
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
            let normals: Vec<_> = mesh
                .normals
                .chunks_exact(3)
                .map(|i| (i[0], i[1], i[2]).into())
                .collect();
            let indices = mesh.indices.iter().map(|i| *i as i32).collect();

            let verts: Vec<_> = verts
                .iter()
                .zip(&tex_coords)
                .zip(&normals)
                .map(|((a, b), c)| (a, b, c))
                .collect();
            let verts: Vec<_> = verts
                .iter()
                .map(|(vert, tex_coord, normals)| DiffuseVertex {
                    // Vertex should be generic or something
                    // so it can have any vertex
                    pos: **vert,
                    tex_coord: **tex_coord,
                    normal: **normals,
                    color: (1.0, 1.0, 1.0).into(),
                })
                .collect();

            let mut element = Element::new(verts, indices)?;
            if let Some(id) = mesh.material_id {
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
        todo!();
    }
}
