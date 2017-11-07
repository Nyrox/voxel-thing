#![deny(unused_must_use)]

extern crate gl;
extern crate glutin;
extern crate libc;

extern crate graphics;
use graphics::OpenGLContext;
use graphics::Shader;

extern crate math;
use math::matrix::Matrix4f;
use math::vector::Vector4f;

use glutin::GlContext;

mod rectangle_shape;
use rectangle_shape::RectangleShape;

use std::fs::File;
use std::io::Read;

fn read_file_contents(filename: &str) -> String {
	let mut f = File::open(filename).unwrap();
	let mut buffer = String::new();
	f.read_to_string(&mut buffer).unwrap();
	buffer
}

fn main() {
	let mut opengl = OpenGLContext::new();
	let r = RectangleShape::new();
	println!("{:?}", r);
	
	let shader = Shader::new();
	shader.attach(&read_file_contents("assets/shaders/tri.vs"), gl::VERTEX_SHADER).unwrap();
	shader.attach(&read_file_contents("assets/shaders/tri.fs"), gl::FRAGMENT_SHADER).unwrap();
	shader.compile().unwrap();
	shader.bind();
	
    unsafe {
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }
	
	
	let mat = Matrix4f::translate(1.0, 1.5, 1.0);
	let vec = Vector4f::position(1.2, 0.7, 0.25);
	
	let perspective = Matrix4f::perspective(1.2, 1280.0 / 720.0, 0.01, 100.0);
	println!("{:?}", perspective);
	
	println!("{:?}", vec);
	let vec = mat.mul_vec(vec);
	println!("{:?}", vec);
	
	shader.setUniform("perspective", perspective);
	
    let mut running = true;
    while running {
		opengl.poll_events();
		while let Some(event) = opengl.poll_event() {
			    match event {
	                glutin::Event::WindowEvent{ event, .. } => match event {
	                    glutin::WindowEvent::Closed => running = false,
	                    glutin::WindowEvent::Resized(w, h) => opengl.window.resize(w, h),
	                    _ => ()
	                },
	                _ => ()
	            }
		}
		
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
		
		r.draw();
		
        opengl.window.swap_buffers().unwrap();
    }
}