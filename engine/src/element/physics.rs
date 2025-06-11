use nalgebra::Point3;
use rapier3d::prelude::{ColliderBuilder, RigidBodyBuilder, RigidBodyType};

use super::Element;

pub enum ColliderShape {
    ConvexHull,
    Cuboid(f32, f32, f32),
}

impl<'a> Element<'a> {
    pub fn add_collider(mut self, shape: ColliderShape) -> Result<Self, String> {
        let mesh_verts: Vec<_> = self
            .mesh
            .positions
            .chunks_exact(3)
            .map(|i| (i[0], i[1], i[2]))
            .collect();

        let points: Vec<_> = mesh_verts
            .iter()
            .map(|v| Point3::new(v.0, v.1, v.2))
            .collect();
        let collider = match shape {
            ColliderShape::ConvexHull => {
                ColliderBuilder::convex_hull(&points).ok_or("Error building collider")?
            }
            ColliderShape::Cuboid(hx, hy, hz) => ColliderBuilder::cuboid(hx, hy, hz),
        }
        .build();
        self.collider = Some(collider);
        eprintln!("Collider added");
        Ok(self)
    }
    pub fn add_rigid_body(mut self, rigid_body_type: RigidBodyType) -> Self {
        self.rigid_body = Some(RigidBodyBuilder::new(rigid_body_type).build());
        eprintln!("Rigid body added");
        self
    }
}
