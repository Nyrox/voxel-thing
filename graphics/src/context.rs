extern crate gl;
extern crate glfw;

use std::sync::mpsc::Receiver;

use context::glfw::Context;
use self::glfw::{Glfw, Window, WindowEvent};

pub struct OpenGLContext {
	pub glfw: Glfw,
	pub window: Window,
	pub events: Receiver<(f64, WindowEvent)>
}

impl OpenGLContext {
	pub fn new() -> OpenGLContext {
		let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

		let (mut window, events) = glfw.create_window(1280, 720, "hello this is window", glfw::WindowMode::Windowed).expect("failed to create glfw window");
		window.set_key_polling(true);
		window.make_current();

		unsafe {
			gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::LESS);
		}

		OpenGLContext { glfw, window, events }
	}

	pub fn poll_events(&mut self) {
		self.glfw.poll_events();
	}

}
