#version 420 core

in vec2 uv;

out vec4 color;

layout(binding = 0) uniform sampler2D _texture;

void main() {
	vec3 _color = texture(_texture, uv).rgb;
	_color = pow(_color, vec3(1.0 / 2.2));


	color = vec4(_color, 1.0);
}
