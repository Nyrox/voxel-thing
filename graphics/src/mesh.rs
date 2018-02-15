extern crate gl;

use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::io;
use std::mem;
use std::ptr;

use vertex::Vertex;
use cgmath::{Vector3, Vector2};

use gl::types::*;

macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize }
    }
}

#[derive(Debug, Default)]
pub struct Mesh {
	vao: GLuint,
	vbo: GLuint,
	ebo: GLuint,
	index_count: i32
}

impl Mesh {
	pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>) -> Mesh {
		let mut mesh = Mesh::default();
		mesh.index_count = indices.len() as i32;

		unsafe {
			gl::CreateVertexArrays(1, &mut mesh.vao);
			gl::CreateBuffers(1, &mut mesh.vbo);
			gl::CreateBuffers(1, &mut mesh.ebo);

			gl::BindVertexArray(mesh.vao);

			gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vao);
			gl::NamedBufferData(mesh.vbo, (mem::size_of::<Vertex>() * vertices.len()) as isize, vertices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);

			gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, mesh.ebo);
			gl::NamedBufferData(mesh.ebo, (mem::size_of::<u32>() * indices.len()) as isize, indices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);

			// Positions
			gl::VertexAttribPointer(0, 3, gl::FLOAT, 0, mem::size_of::<Vertex>() as i32, ptr::null());
			gl::EnableVertexAttribArray(0);

			// Normals
			gl::VertexAttribPointer(1, 3, gl::FLOAT, 0, mem::size_of::<Vertex>() as i32, offset_of!(Vertex, normal) as *const GLvoid);
			gl::EnableVertexAttribArray(1);

			// Tangents
			gl::VertexAttribPointer(2, 3, gl::FLOAT, 0, mem::size_of::<Vertex>() as i32, offset_of!(Vertex, tangent) as *const GLvoid);
			gl::EnableVertexAttribArray(2);

			// Uv's
			gl::VertexAttribPointer(3, 2, gl::FLOAT, 0, mem::size_of::<Vertex>() as i32, offset_of!(Vertex, uv) as *const GLvoid);
			gl::EnableVertexAttribArray(3);

			gl::BindVertexArray(0);
		}

		mesh
	}

	pub fn draw(&self) {
		unsafe {
			gl::BindVertexArray(self.vao);
			gl::DrawElements(gl::TRIANGLES, self.index_count, gl::UNSIGNED_INT, 0 as *const GLvoid);
			gl::BindVertexArray(0);
		}
	}

	pub fn load_ply(path: PathBuf) -> Mesh {
		let buffer = read_file_to_string(path).unwrap();

		let mut vertices 	= Vec::<Vertex>::new();
		let mut indices 	= Vec::<u32>::new();

		let mut lines = buffer.lines();

		// Parse header
		'header: while let Some(line) = lines.next() {
			let mut tokens = line.split_whitespace();

			match tokens.next().unwrap() {
				"element" => {
					match tokens.next().unwrap() {
						"vertex" => vertices.reserve_exact(tokens.next().unwrap().parse::<usize>().unwrap()),
						_ => { }
					}
				}
				"end_header" => break 'header,
				_ => { }
			}
		};

		// Parse vertices
		for _ in 0..vertices.capacity() {
			let mut line = lines.next().unwrap();
			let mut tokens = line.split_whitespace();
			let values = tokens.map(|t| t.parse::<f32>().unwrap()).collect::<Vec<f32>>();
			vertices.push(Vertex {
				position: Vector3::new(values[0], values[1], values[2]),
				normal: Vector3::new(values[3], values[4], values[5]),
				uv: Vector2::new(values[6], values[7]),
				tangent: Vector3::new(0.0, 0.0, 0.0)
			});
		};

		// Parse faces
		'faces: while let Some(line) = lines.next() {
			let mut tokens = line.split_whitespace();
			let values = tokens.map(|t| t.parse::<u32>().unwrap()).collect::<Vec<u32>>();

			match values[0] {
				3 => {
					let mut face = [values[1], values[2], values[3]];

					let tangent = Vertex::calculate_tangent(vertices[face[0] as usize], vertices[face[1] as usize], vertices[face[2] as usize]);
					vertices[face[0] as usize].tangent = tangent;
					vertices[face[1] as usize].tangent = tangent;
					vertices[face[2] as usize].tangent = tangent;

					indices.append(&mut face.to_vec());
				}
				_ => { }
			}
		};

		Mesh::new(vertices, indices)
	}
}

fn read_file_to_string(file: PathBuf) -> Result<String, io::Error> {
	let mut file = File::open(file)?;
	let mut buffer = String::new();
	file.read_to_string(&mut buffer)?;
	Ok(buffer)
}
