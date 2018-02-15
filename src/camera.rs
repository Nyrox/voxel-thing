use cgmath::{PerspectiveFov, Matrix4, Vector3, Vector4};
use transform::Transform;

pub struct Camera {
	pub transform: Transform,
	pub projection: PerspectiveFov<f32>
}

impl Camera {
	pub fn new(transform: Transform, projection: PerspectiveFov<f32>) -> Camera {
		Camera { transform, projection }
	}

	pub fn get_view_matrix(&self) -> Matrix4<f32> {
		Matrix4::look_at_dir(self.transform.position, (Matrix4::from(self.transform.rotation) * Vector4::new(0.0, 0.0, -1.0, 0.0)).truncate(), Vector3::new(0.0, 1.0, 0.0))
	}

	pub fn get_projection_matrix(&self) -> Matrix4<f32> {
		Matrix4::from(self.projection)
	}
}
