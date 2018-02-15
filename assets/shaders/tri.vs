#version 410 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec3 tangent;
layout(location = 3) in vec2 _uv;

out vec3 frag_normal;
out vec3 frag_position;
out vec2 uv;
out mat3 TBN;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
	gl_Position = perspective * view * model * vec4(position, 1.0);
	frag_position = vec3(position.x, position.y, position.z);
	frag_normal = normalize(normal);
	uv = _uv;

	vec3 T = normalize(tangent);
	vec3 N = normalize(normal);
	T = normalize(T - dot(T, N) * N);
	vec3 B = cross(T, N);
	TBN = mat3(T, B, N);
}
