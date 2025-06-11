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

// Numbers
impl UniformValue for bool {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self as i32);
        }
    }
}
impl UniformValue for f32 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1f(location, *self);
        }
    }
}
impl UniformValue for i32 {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform1i(location, *self);
        }
    }
}

// Vectors
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

// Tuples
impl UniformValue for (i32, i32) {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform2i(location, self.0, self.1);
        }
    }
}
impl UniformValue for (f32, f32) {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform2f(location, self.0, self.1);
        }
    }
}
impl UniformValue for (i32, i32, i32) {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform3i(location, self.0, self.1, self.2);
        }
    }
}
impl UniformValue for (f32, f32, f32) {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform3f(location, self.0, self.1, self.2);
        }
    }
}
impl UniformValue for (i32, i32, i32, i32) {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform4i(location, self.0, self.1, self.2, self.3);
        }
    }
}
impl UniformValue for (f32, f32, f32, f32) {
    fn set_uniform(&self, location: i32) {
        unsafe {
            gl::Uniform4f(location, self.0, self.1, self.2, self.3);
        }
    }
}

// Matrices
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
