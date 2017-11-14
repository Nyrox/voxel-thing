#version 410 core

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 _uv;

out vec3 frag_normal;
out vec3 frag_position;
out vec2 uv;

uniform mat4 perspective;

void main() {
	gl_Position = perspective * vec4(position.x + 3, position.y + 1, position.z - 5, 1.0);
	frag_position = vec3(position.x + 3, position.y + 1, position.z - 5);
	frag_normal = normalize(normal);
	uv = _uv;
}