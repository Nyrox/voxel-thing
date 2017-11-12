#version 330 core

in vec3 frag_normal;
in vec3 frag_position;

out vec4 out_color;

void main() {
	vec3 light_pos = vec3(1, 1, -3.5);
	vec3 l = normalize(light_pos - frag_position);
	float diff = max(dot(frag_normal, l), 0.10);
	
	vec3 color = vec3(diff);
	
	out_color = vec4(color, 1.0);
}