use nalgebra::{Point3, Rotation3, Unit, Vector3};

use super::Camera;

impl Camera {
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
    pub fn move_up(&mut self) {
        self.position += Vector3::y() * self.movement_speed;
        self.update_view();
    }
    pub fn move_down(&mut self) {
        self.position -= Vector3::y() * self.movement_speed;
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
