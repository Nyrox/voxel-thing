extern crate gl;

use std::mem;
use std::ptr;


use gl::types::*;

#[derive(Debug)]
pub struct RectangleShape {
	vbo: GLuint,
	vao: GLuint
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
	x: T,
	y: T
}

impl<T> Vec2<T> {
	pub fn new(x: T, y: T) -> Vec2<T> {
		Vec2 { x, y }
	}
}


impl RectangleShape {
	
	pub fn new() -> RectangleShape {
		let mut r = RectangleShape { vbo: 0, vao: 0 };
		r.init_opengl_members();

		
		r
	}
	
	fn init_opengl_members(&mut self) {
		unsafe {
			gl::CreateVertexArrays(1, &mut self.vao);
			gl::CreateBuffers(1, &mut self.vbo);
			
			gl::BindVertexArray(self.vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vao);
			
			gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<GLfloat>() as i32 * 2, ptr::null());
			gl::EnableVertexAttribArray(0);
		}
		
		let mut verts: Vec<Vec2<f32>> = Vec::new();
		
		for i in 0..4 {
			verts.push(self.get_point(i));
		}
		verts.push(self.get_point(0));
		
		unsafe {
			gl::NamedBufferData(self.vbo, mem::size_of::<Vec2<f32>>() as isize * verts.len() as isize, verts.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
		}
	}
	
	fn get_point(&self, index: i32) -> Vec2<f32> {
		match index {
			0 => Vec2::new(0.0, 0.0),
			1 => Vec2::new(1.0, 0.0),
			2 => Vec2::new(1.0, 1.0),
			3 => Vec2::new(0.0, 1.0),
			_ => panic!()
		}
	}
	
	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 5);
			gl::BindVertexArray(0);
		}
	}
}