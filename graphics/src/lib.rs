#![feature(ord_max_min)]

extern crate cgmath;

pub use self::context::OpenGLContext;
pub use self::shader::Shader;
pub use self::mesh::Mesh;
pub use self::texture::Texture2D;

pub mod context;
pub mod shader;
pub mod vertex;
pub mod mesh;
pub mod texture;

use std::ffi::CString;

use cgmath::Matrix4;
use cgmath::Vector3;
use shader::{Uniform};


extern crate gl;
use self::gl::types::{GLuint};

impl Uniform for Matrix4<f32> {
	fn set(&self, id: &str, handle: GLuint) {
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			gl::ProgramUniformMatrix4fv(handle, location, 1, gl::FALSE, ::std::mem::transmute(self));
		}
	}
}

impl Uniform for Vector3<f32> {
	fn set(&self, id: &str, handle: GLuint) {
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			gl::ProgramUniformMatrix3fv(handle, location, 1, gl::FALSE, ::std::mem::transmute(self));
		}
	}
}

impl Uniform for i32 {
	fn set(&self, id: &str, handle: GLuint) {
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			gl::ProgramUniform1i(handle, location, *self);
		}
	}
}
