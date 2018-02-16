extern crate gl;

use std::mem;
use std::ptr;


use gl::types::*;

#[derive(Debug)]
pub struct RectangleShape {
	vbo: GLuint,
	vao: GLuint,
	pub width: f32,
	pub height: f32
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
	
	pub fn new(width: f32, height: f32) -> RectangleShape {
		let mut r = RectangleShape { vbo: 0, vao: 0, width, height };
		r.init_opengl_members();

		
		r
	}
	
	fn init_opengl_members(&mut self) {
		unsafe {
			gl::CreateVertexArrays(1, &mut self.vao);
			gl::CreateBuffers(1, &mut self.vbo);
			
			gl::BindVertexArray(self.vao);
			gl::BindBuffer(gl::ARRAY_BUFFER, self.vao);
			
			gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<GLfloat>() as i32 * 4, ptr::null());
			gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE as GLboolean, mem::size_of::<GLfloat>() as i32 * 4, (mem::size_of::<GLfloat>() as i32 * 2) as *const _);
			
			gl::EnableVertexAttribArray(0);
			gl::EnableVertexAttribArray(1);
		}
		
		let mut verts: Vec<(Vec2<f32>, Vec2<f32>)> = Vec::new();
		
		for i in 0..4 {
			verts.push(self.get_point(i));
		}
		verts.push(self.get_point(0));
		
		unsafe {
			gl::NamedBufferData(self.vbo, mem::size_of::<(Vec2<f32>, Vec2<f32>)>() as isize * verts.len() as isize, verts.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
		}
	}
	
	fn get_point(&self, index: i32) -> (Vec2<f32>, Vec2<f32>) {
		let uv = match index {
			0 => Vec2::new(0.0, 0.0),
			1 => Vec2::new(1.0, 0.0),
			2 => Vec2::new(1.0, 1.0),
			3 => Vec2::new(0.0, 1.0),
			_ => panic!()
		};
		let pos = Vec2::new(uv.x * self.width, uv.y * self.height);
		(pos, uv)
	}
	
	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 5);
			gl::BindVertexArray(0);
		}
	}
}