#version 410 core

layout(location = 0) in vec3 position;

out vec3 frag_pos;

uniform mat4 projection;
uniform mat4 view;

uniform ivec2 chunkDims;
uniform ivec2 chunkIndex;

void main() {
	vec2 chunk_origin_xz = chunkIndex * chunkDims;
	vec3 chunk_origin = vec3(chunk_origin_xz.x, 0, chunk_origin_xz.y);

	gl_Position = projection * view * vec4(chunk_origin + position, 1.0);
	frag_pos = position;
}
