pub use self::context::OpenGLContext;
pub use self::shader::Shader;

pub mod context;
pub mod shader;
pub mod vertex;

use std::ffi::CString;

extern crate math;
use math::matrix::{Matrix4};
use shader::{Uniform};

extern crate gl;
use self::gl::types::{GLuint};

impl<f32> Uniform for Matrix4<f32> where f32: std::fmt::Debug {
	fn set(&self, id: &str, handle: GLuint) {		
		unsafe {
			let name = CString::new(id.as_bytes()).unwrap();
			let location = gl::GetUniformLocation(handle, name.as_ptr());
			println!("{:?}, {:?}, {:?}", id, location, *self);
			gl::ProgramUniformMatrix4fv(handle, location, 1, gl::FALSE, self.data.as_ptr() as *const _);
		}
	}
}