#version 330 core

in vec2 position;

uniform mat4 perspective;

void main() {
	gl_Position = perspective * vec4(position.x, position.y - position.x, -5, 1.0);
}