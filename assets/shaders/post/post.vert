#version 330

in vec2 position;
out vec2 uv;

void main() {
    gl_Position = vec4(position, 1.0, 1.0);
    vec2 _uv = (position + 1.0) / 2.0;
	uv = vec2(_uv.x, 1.0 - _uv.y);
}
