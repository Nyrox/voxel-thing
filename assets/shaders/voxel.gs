#version 420 core
layout (triangles) in;
layout (triangle_strip, max_vertices=3) out;

in vec3 frag_pos[];

out vec3 frag_position;
out vec3 frag_normal;
out vec2 uv;

vec2 get_uv(vec3 N, vec3 pos) {
	const vec3 UP = vec3(0.0, 1.0, 0.0);
	const vec3 RIGHT = vec3(1.0, 0.0, 0.0);


	if (abs(dot(N, UP)) > 0.9) {
		return pos.xz;
	}
	else if (abs(dot(N, RIGHT)) > 0.9) {
		return pos.yz;
	}
	else {
		return pos.xy;
	}
}

void main() {
	vec3 U = frag_pos[1] - frag_pos[0];
	vec3 V = frag_pos[2] - frag_pos[0];

	vec3 N = vec3(
		(U.y * V.z) - (U.z * V.y),
		(U.z * V.x) - (U.x * V.z),
		(U.x * V.y) - (U.y * V.x)
	);


    gl_Position = gl_in[0].gl_Position;
	frag_normal = N;
	uv = get_uv(N, frag_pos[0]);
	frag_position = frag_pos[0];
    EmitVertex();

    gl_Position = gl_in[1].gl_Position;
	frag_normal = N;
	uv = get_uv(N, frag_pos[1]);
	frag_position = frag_pos[1];
    EmitVertex();

	gl_Position = gl_in[2].gl_Position;
	uv = get_uv(N, frag_pos[2]);
	frag_position = frag_pos[2];
	frag_normal = N;

	EmitVertex();


    EndPrimitive();
}
