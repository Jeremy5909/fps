use std::f32;

use nalgebra::{Matrix4, Perspective3, Point3, Rotation3, Unit, Vector3};

pub struct Camera {
    pub(crate) projection: Matrix4<f32>,
    pub(crate) view: Matrix4<f32>,
    position: Point3<f32>,
    orientation: Vector3<f32>,
    pub movement_speed: f32,
}
impl Camera {
    pub fn new(aspect_ratio: f32, fov: f32, near: f32, far: f32, movement_speed: f32) -> Self {
        let projection = Perspective3::new(aspect_ratio, fov, near, far).to_homogeneous();
        let position = Point3::new(0.0, 0.0, 3.0);
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
    pub fn position(&self) -> Point3<f32> {
        self.position
    }
    pub fn set_position(&mut self, pos: Point3<f32>) {
        self.position = pos;
        self.update_view();
    }
    pub fn set_orientation(&mut self, orientation: Vector3<f32>) {
        self.orientation = orientation;
    }
    pub fn move_forward(&mut self) {
        self.position += self.orientation * self.movement_speed;
        self.update_view();
    }
    pub fn move_backward(&mut self) {
        self.position -= self.orientation * self.movement_speed;
        self.update_view();
    }
    pub fn move_right(&mut self) {
        self.position += self.orientation.cross(&Vector3::y()).normalize() * self.movement_speed;
        self.update_view();
    }
    pub fn move_left(&mut self) {
        self.position -= self.orientation.cross(&Vector3::y()).normalize() * self.movement_speed;
        self.update_view();
    }
    pub fn rotate(&mut self, xrel: i32, yrel: i32) {
        let sensitivity = 0.002;
        let yaw = -xrel as f32 * sensitivity;
        let pitch = -yrel as f32 * sensitivity;

        let right = Unit::new_normalize(self.orientation.cross(&Vector3::y()));

        self.orientation = Rotation3::from_axis_angle(&Vector3::y_axis(), yaw) * self.orientation;
        self.orientation = Rotation3::from_axis_angle(&right, pitch) * self.orientation;

        self.update_view();
    }
}
impl Default for Camera {
    fn default() -> Self {
        Self::new(1.0, f32::consts::PI / 3.0, 0.1, 100.0, 0.01)
    }
}
