use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

pub struct Camera {
    pub projection: Matrix4<f32>,
    pub view: Matrix4<f32>,
}
impl Camera {
    pub fn new(aspect_ratio: f32, fov: f32, near: f32, far: f32, position: Point3<f32>) -> Self {
        let projection = Perspective3::new(aspect_ratio, fov, near, far);
        let view = Matrix4::look_at_rh(&position, &Point3::origin(), &Vector3::y_axis());
        Self {
            projection: projection.to_homogeneous(),
            view,
        }
    }
}
