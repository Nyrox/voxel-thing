
use cgmath::prelude::*;
use cgmath::{Point3, Vector3, Quaternion};

pub struct Transform {
	pub position: Point3<f32>,
	pub rotation: Quaternion<f32>
}

impl Default for Transform {
	fn default() -> Transform {
		Transform {
			position: Point3::new(0.0, 0.0, 0.0),
			rotation: Quaternion::from_sv(0.0, Vector3::new(0.0, 0.0, 1.0))
		}
	}
}

impl Transform {
	pub fn forward(&self) -> Vector3<f32> {
		self.rotation.rotate_vector(Vector3::new(0.0, 0.0, 1.0))
	}

	pub fn right(&self) -> Vector3<f32> {
		self.rotation.rotate_vector(Vector3::new(1.0, 0.0, 0.0))
	}
}
