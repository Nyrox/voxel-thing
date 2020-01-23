#version 330

in vec2 position;
out vec2 uv;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    uv = (position + 1.0) / 2.0;
}
