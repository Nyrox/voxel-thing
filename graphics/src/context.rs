extern crate glutin;
extern crate gl;

use self::glutin::GlContext;


pub struct OpenGLContext {
	pub events_loop: glutin::EventsLoop,
	pub window: glutin::GlWindow,
	
	event_buffer: Vec<glutin::Event>
}

impl OpenGLContext {
	pub fn poll_events(&mut self) {
		// Borrow the event buffer explicitly, to avoid rusts borrow checker from complaining about borrowing 'self' in the closure
		// This will hopefully be fixed in a future rust version
		let event_buffer = &mut self.event_buffer;
		self.events_loop.poll_events(|event| {
			event_buffer.push(event);
		});
	}
	
	pub fn poll_event(&mut self) -> Option<glutin::Event> {
		self.event_buffer.pop()
	}
	
	pub fn new() -> OpenGLContext {
		let events_loop = glutin::EventsLoop::new();
		let window = glutin::WindowBuilder::new().with_title("Praise, kek!").with_dimensions(1280, 720);
		
		let context = glutin::ContextBuilder::new()
		.with_vsync(true)
		.with_gl_profile(glutin::GlProfile::Core)
		.with_gl_debug_flag(true);
		
		let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
		
		unsafe { 
			gl_window.make_current().unwrap();
			gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
			gl::Enable(gl::DEPTH_TEST);
			gl::DepthFunc(gl::LESS);
		}
		
		OpenGLContext { events_loop, window: gl_window, event_buffer: Vec::new() }	
	}
}