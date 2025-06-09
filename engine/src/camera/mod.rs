use std::f32;

use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

mod funcs;

pub struct Camera {
    pub(crate) projection: Matrix4<f32>,
    pub(crate) view: Matrix4<f32>,
    position: Point3<f32>,
    orientation: Vector3<f32>,
    pub movement_speed: f32,
}
impl Camera {
    pub fn new(
        position: Point3<f32>,
        aspect_ratio: f32,
        fov: f32,
        near: f32,
        far: f32,
        movement_speed: f32,
    ) -> Self {
        let projection = Perspective3::new(aspect_ratio, fov, near, far).to_homogeneous();
        let orientation = -Vector3::z();
        let mut camera = Self {
            projection,
            view: Matrix4::identity(),
            position,
            orientation,
            movement_speed,
        };
        camera.update_view();
        camera
    }
    fn update_view(&mut self) {
        self.orientation.normalize_mut();

        self.view = Matrix4::look_at_rh(
            &self.position,
            &(self.position + self.orientation),
            &Vector3::y_axis(),
        )
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self::new(
            Point3::new(0.0, 0.0, 3.0),
            1.0,
            f32::consts::PI / 3.0,
            0.1,
            100.0,
            0.01,
        )
    }
}
