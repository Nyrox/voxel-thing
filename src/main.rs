#![deny(unused_must_use)]

#![feature(duration_extras)]

extern crate gl;
extern crate libc;
extern crate glfw;
extern crate cgmath;
extern crate graphics;
extern crate serde_json;

mod rectangle_shape;
mod camera;
mod transform;

use transform::Transform;
use graphics::{OpenGLContext, Shader, Mesh, Texture2D};
use rectangle_shape::RectangleShape;
use camera::{Camera};
use glfw::{Key, Action, Context};

use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time;

use cgmath::prelude::*;
use cgmath::{Vector3, Matrix4, Quaternion, Rad, Deg, PerspectiveFov};

use std::sync::{Mutex, Arc};

use gl::types::*;



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
	let r = RectangleShape::new(1280.0, 720.0);
	println!("{:?}", r);

	// Create framebuffer
	let mut fbo: GLuint = 0;
	let mut cl: GLuint = 0;
	
	unsafe {
		gl::GenFramebuffers(1, &mut fbo);	
		gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
	
		gl::GenTextures(1, &mut cl);
		gl::BindTexture(gl::TEXTURE_2D, cl);
		
		gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, 1280, 720, 0, gl::RGB, gl::UNSIGNED_BYTE, ::std::ptr::null_mut());
		
		gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
		gl::TextureParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
		
		gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, cl, 0);
	}

	let shader = Shader::new();
	shader.attach(&read_file_contents("assets/shaders/tri.vs"), gl::VERTEX_SHADER).unwrap();
	shader.attach(&read_file_contents("assets/shaders/tri.fs"), gl::FRAGMENT_SHADER).unwrap();
	shader.compile().unwrap();
	shader.bind();
	
	let draw_fullscreen = Shader::new();
	draw_fullscreen.attach(&read_file_contents("assets/shaders/draw_fullscreen.vs"), gl::VERTEX_SHADER).unwrap();
	draw_fullscreen.attach(&read_file_contents("assets/shaders/draw_fullscreen.fs"), gl::FRAGMENT_SHADER).unwrap();
	draw_fullscreen.compile().unwrap();
	draw_fullscreen.bind();
	
	// let shader = load_shader(PathBuf::from("assets/shaders/schema.json"));

	let mesh = Mesh::load_ply(PathBuf::from("assets/meshes/cube.ply"));
	println!("{:?}", mesh);

	let albedo = Texture2D::new(PathBuf::from("assets/textures/harshbricks-albedo.png"), gl::SRGB8_ALPHA8);
	let roughness = Texture2D::new(PathBuf::from("assets/textures/harshbricks-roughness.png"), gl::R8);
	let normal = Texture2D::new(PathBuf::from("assets/textures/harshbricks-normal.png"), gl::RGB8);

    unsafe {
        gl::ClearColor(0.05, 0.05, 0.05, 1.0);
    }

	let mut camera = Camera::new(Transform::default(), PerspectiveFov { fovy: Rad::from(Deg(75.0)), aspect: 1280.0 / 720.0, near: 0.1, far: 100.0 });
	camera.transform.position.z = -3.0;

	shader.setUniform("model", Matrix4::<f32>::from_translation(Vector3::new(1.0, 0.0, 0.0)));
	shader.setUniform("perspective", camera.get_projection_matrix());

	let command_buffer = Arc::new(Mutex::new(Vec::new()));
	{
		let command_buffer = command_buffer.clone();
		let input_thread = ::std::thread::spawn(move || {
			loop {
				let mut buffer = String::new();
				::std::io::stdin().read_line(&mut buffer).expect("Failed to read line from stdin");
				command_buffer.lock().expect("Failed to get lock on command buffer.").push(buffer);
			}
		});
	}
	
	let mut last_time = time::Instant::now();
	
    while !opengl.window.should_close() {
		let time = time::Instant::now();
		let delta_time = time.duration_since(last_time).subsec_millis() as f32 / 1000.0;
		last_time = time;
		
		{
			let mut command_buffer = command_buffer.lock().expect("Failed to get lock on command buffer.");
			for command in command_buffer.drain(..) {
				println!("{:?}", command.as_str());
				match command.as_str() {
					"quit\r\n" => { print!("Shutting down..."); opengl.window.set_should_close(true); }
					_ => { println!("Unknown command: {}", command); }
				}
			}
		}

		opengl.poll_events();
		for(_, event) in glfw::flush_messages(&opengl.events) {
			match event {
				glfw::WindowEvent::Close => { opengl.window.set_should_close(true); }
				glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
					opengl.window.set_should_close(true);
				},
				_ => {}
			}
		}
		
		let movement_speed = 1.0;
		
		if opengl.window.get_key(Key::W) == Action::Press {
			camera.transform.position += camera.transform.forward() * movement_speed * delta_time;
		}
		if opengl.window.get_key(Key::S) == Action::Press {
			camera.transform.position += camera.transform.forward() *  -movement_speed * delta_time;
		}
		if opengl.window.get_key(Key::A) == Action::Press {
			camera.transform.position += camera.transform.right() *  -movement_speed * delta_time;
		}
		if opengl.window.get_key(Key::D) == Action::Press {
			camera.transform.position += camera.transform.right() * movement_speed * delta_time;
		}

		if opengl.window.get_key(Key::Q) == Action::Press {
			camera.transform.rotation = camera.transform.rotation * Quaternion::from_angle_y(Deg(-15.0 * delta_time));
		}
		if opengl.window.get_key(Key::E) == Action::Press {
			camera.transform.rotation = camera.transform.rotation * Quaternion::from_angle_y(Deg(15.0 * delta_time));
		}

		shader.setUniform("view", camera.get_view_matrix());

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

		albedo.bind(0);
		roughness.bind(1);
		normal.bind(3);
		shader.bind();
		unsafe {
			gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);
			mesh.draw();

			gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
			draw_fullscreen.bind();
			draw_fullscreen.setUniform("projection", Matrix4::from(cgmath::Ortho { left: 0.0, top: 0.0, bottom: 720.0, right: 1280.0, near: 0.0, far: 100.0 }));
			gl::BindTexture(gl::TEXTURE0, cl);
			r.draw();
		}

		opengl.window.swap_buffers();
    }
}
