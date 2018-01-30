#![deny(unused_must_use)]

extern crate gl;
extern crate glutin;
extern crate libc;

extern crate graphics;
use graphics::{OpenGLContext, Shader, Mesh, Texture2D};


extern crate math;
use math::matrix::Matrix4f;
use math::vector4::Vector4f;
use math::vector3::Vector3;

use glutin::GlContext;
use glutin::VirtualKeyCode;
use glutin::{KeyboardInput};

mod rectangle_shape;
use rectangle_shape::RectangleShape;

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod camera;
use camera::{Camera};

extern crate serde_json;

struct X();


fn read_file_contents(filename: &str) -> String {
	let mut f = File::open(filename).unwrap();
	let mut buffer = String::new();
	f.read_to_string(&mut buffer).unwrap();
	buffer
}

fn load_shader(path: PathBuf) -> Shader {
	let file = read_file_contents(path.to_str().unwrap());
	let data: serde_json::Value = serde_json::from_str(&file).expect("Failed to read shader file.");

	let shader = Shader::new();
	shader.attach(&read_file_contents(data["vertex"].as_str().unwrap()), gl::VERTEX_SHADER).unwrap();
	shader.attach(&read_file_contents(data["fragment"].as_str().unwrap()), gl::FRAGMENT_SHADER).unwrap();
	shader.compile().unwrap();
	return shader;
}

fn main() {
	let mut opengl = OpenGLContext::new();
	let r = RectangleShape::new();
	println!("{:?}", r);

	// let shader = Shader::new();
	// shader.attach(&read_file_contents("assets/shaders/tri.vs"), gl::VERTEX_SHADER).unwrap();
	// shader.attach(&read_file_contents("assets/shaders/tri.fs"), gl::FRAGMENT_SHADER).unwrap();
	// shader.compile().unwrap();
	// shader.bind();

	let shader = load_shader(PathBuf::from("assets/shaders/schema.json"));

	let mesh = Mesh::load_ply(PathBuf::from("assets/meshes/cube.ply"));
	println!("{:?}", mesh);

	let albedo = Texture2D::new(PathBuf::from("assets/textures/harshbricks-albedo.png"), gl::SRGB8);
	let roughness = Texture2D::new(PathBuf::from("assets/textures/harshbricks-roughness.png"), gl::R8);
	let normal = Texture2D::new(PathBuf::from("assets/textures/harshbricks-normal.png"), gl::RGB8);

	albedo.bind(0);
	roughness.bind(1);
	normal.bind(3);

    unsafe {
        gl::ClearColor(0.05, 0.05, 0.05, 1.0);
    }


	let mat = Matrix4f::translate(9.0, 1.5, 1.0);
	let vec = Vector4f::position(1.2, 0.7, 0.25);
	let view = Matrix4f::look_at(Vector3::new(0.0, 0.0, 3.0), Vector3::default(), Vector3::up());

	let mut camera = Camera::new();
	camera.position.z = 3.0;

	let perspective = Matrix4f::perspective(1.2, 1280.0 / 720.0, 0.01, 100.0);
	println!("{:?}", perspective);

	println!("{:?}", vec);
	let vec = mat.mul_vec(vec);
	println!("{:?}", vec);

	shader.setUniform("perspective", perspective);
	shader.setUniform("view", view);

    let mut running = true;
    while running {
		opengl.poll_events();
		while let Some(event) = opengl.poll_event() {
			match event {
	            glutin::Event::WindowEvent{ event, .. } => match event {
	                glutin::WindowEvent::Closed => running = false,
	                glutin::WindowEvent::Resized(w, h) => opengl.window.resize(w, h),
					glutin::WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(VirtualKeyCode::W), ..}, ..} => {
						camera.position.z += 0.01;
					}
	                _ => ()
	            },
	            _ => ()
			}
		}

		shader.setUniform("view", camera.get_view_matrix());

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

		r.draw();
		// mesh.draw();

        opengl.window.swap_buffers().unwrap();
    }
}
