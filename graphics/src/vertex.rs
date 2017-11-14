extern crate math;
use self::math::{Vector3f, Vector2f};


#[derive(Debug)]
pub struct Vertex {
	pub position: Vector3f,
	pub normal: Vector3f,
	pub uv: Vector2f
}

impl Vertex {
	
	
// 	glm::vec3 calculateTangent(const Vertex& vx, const Vertex& vy, const Vertex& vz) {
// 	// Calculate edges
// 	glm::vec3 edge1 = vy.position - vx.position;
// 	glm::vec3 edge2 = vz.position - vx.position;
// 
// 	// Calculate delta UV
// 	glm::vec2 deltaUV1 = vy.uv - vx.uv;
// 	glm::vec2 deltaUV2 = vz.uv - vx.uv;
// 
// 	// Some pythagoras shit dunno
// 	float f = 1.f / (deltaUV1.x * deltaUV2.y - deltaUV2.x * deltaUV1.y);
// 	glm::vec3 tangent;
// 	tangent.x = f * (deltaUV2.y * edge1.x - deltaUV1.y * edge2.x);
// 	tangent.y = f * (deltaUV2.y * edge1.y - deltaUV1.y * edge2.y);
// 	tangent.z = f * (deltaUV2.y * edge1.z - deltaUV1.y * edge2.z);
// 	tangent = glm::normalize(tangent);
// 
// 	return tangent;
// }
	
}