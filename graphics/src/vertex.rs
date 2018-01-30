extern crate math;
use self::math::{Vector3f, Vector2f};


#[derive(Debug, Clone, Copy)]
pub struct Vertex {
	pub position: Vector3f,
	pub normal: Vector3f,
	pub tangent: Vector3f,
	pub uv: Vector2f
}

impl Vertex {
	pub fn calculate_tangent(x: Vertex, y: Vertex, z: Vertex) -> Vector3f {
		let edge1 = y.position - x.position;
		let edge2 = z.position - x.position;
		
		let uv1 = y.uv - x.uv;
		let uv2 = z.uv - x.uv;
		
		let f = 1.0 / (uv1.x * uv2.y - uv2.x * uv1.y);
		let mut tangent = Vector3f::default();
		tangent.x = f * (uv2.y * edge1.x - uv1.y * edge2.x);
		tangent.y = f * (uv2.y * edge1.y - uv1.y * edge2.y);
		tangent.z = f * (uv2.y * edge1.z - uv1.y * edge2.z);
		
		tangent.normalize()
	}	
}