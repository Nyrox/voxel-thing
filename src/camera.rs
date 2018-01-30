use math::matrix::Matrix4f;
use math::vector3::Vector3;
use math::Vector3f;

pub struct Camera {
	pub position: Vector3f,
	
}

impl Camera {
	pub fn new() -> Camera {
		Camera { position: Vector3f::default() }
	}
	
	pub fn get_view_matrix(&self) -> Matrix4f {
		Matrix4f::look_at(self.position, Vector3::default(), Vector3::up())
	}
}