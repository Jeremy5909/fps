use std::ffi::CString;

use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};

use crate::program::Program;

impl Program {
    pub fn set_uniform<T: UniformValue>(&self, name: &str, val: &T) {
        self.set_used();
        let location =
            unsafe { gl::GetUniformLocation(self.id, CString::new(name).unwrap().as_ptr()) };
        val.set_uniform(location);
    }
}

pub trait UniformValue {
    fn set_uniform(&self, location: i32);
}
impl UniformValue for i32 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}
impl UniformValue for Vector3<f32> {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self.x, self.y, self.z);
        }
    }
}
impl UniformValue for Vector4<f32> {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform4f(location, self.x, self.y, self.z, self.w);
        }
    }
}
impl UniformValue for Matrix3<f32> {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::UniformMatrix3fv(location, 1, gl::FALSE, self.as_ptr());
        }
    }
}
impl UniformValue for Matrix4<f32> {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::UniformMatrix4fv(location, 1, gl::FALSE, self.as_ptr());
        }
    }
}
