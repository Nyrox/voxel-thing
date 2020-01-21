#version 410 core

layout(location = 0) in vec2 position;
layout(location = 1) in vec2 _uv;

out vec2 uv;

uniform mat4 projection;

void main() {
	gl_Position = projection * vec4(position.xy, -50.0, 1.0);
	uv = _uv;
}
