#version 330 core

in vec3 frag_normal;
in vec3 frag_position;
in vec2 uv;

out vec4 out_color;

uniform sampler2D t_albedo;
uniform sampler2D t_roughness;
uniform sampler2D t_metalness;

const float PI = 3.14159265359;

float square(float v) {
	return v * v;
}

// Normal distribution function
float ggx_distribution(vec3 n, vec3 h, float a) {
	float a_squared = square(a);
	float nominator = a_squared;
	float NdotH = max(dot(n, h), 0.0);
	float demoninator = PI * square(square(NdotH) * (a_squared - 1) + 1);
	
	return nominator / demoninator;
}

// Geometry shadowing function over a single direction
float geometry_schlick_ggx(vec3 n, vec3 v, float k) {
	float NdotV = max(dot(n, v), 0.0);
	float nominator = NdotV;
	float denominator = NdotV * (1.0 - k) + k;
	
	return nominator / denominator;
}

// Geometry shadowing function over incident angle
// Basically runs the geometry function once both for 
// 	- Incoming rays shadowed by obstructing microfacets
//	- Outgoing rays shadowed by obstructing microfacets
float geometry_smith(vec3 n, vec3 v, vec3 l, float k) {
	return geometry_schlick_ggx(n, v, k) * geometry_schlick_ggx(n, l, k);
}

// Fresnel implementation
vec3 fresnel_schlick(float cos_theta, vec3 F0) {
	return F0 + (1.0 - F0) * pow(1.0 - cos_theta, 5.0);
}

// Fresnel function
// Todo: Test the significance of F0 ad potentially make it possible to override
vec3 fresnel(vec3 object_color, vec3 n, vec3 l, float metalness) {
	vec3 F0 = vec3(0.04);
	F0		= mix(F0, object_color, metalness);
	return fresnel_schlick(max(dot(n, l), 0.0), F0);
}

void main() {
	vec3 light_pos = vec3(0, 1.5, -6);
	vec3 light_strength = vec3(8, 8, 8);
	vec3 object_color = texture(t_albedo, uv).rgb;
	float roughness = texture(t_roughness, uv).r;
	float metallic = 0;
	
	vec3 camera_pos = vec3(0);
	
	vec3 wi = normalize(light_pos - frag_position);
	vec3 N = normalize(frag_normal);
	vec3 V = normalize(camera_pos - frag_position);
	vec3 H = normalize(wi + V);
	
	float _distance = distance(light_pos, frag_position);
	
	float cos_theta = max(dot(N, wi), 0.0);
	float attenuation = 1.0 / (_distance * _distance);
	vec3 radiance = light_strength * attenuation * cos_theta;
	
	float NdotL = max(dot(N, wi), 0.0);


	vec3 F = fresnel(object_color, N, wi, metallic);
	vec3 specular_part = F;
	vec3 diffuse_part = vec3(1.0) - F;
	diffuse_part *= 1.0 - metallic;
	
	float D = ggx_distribution(N, H, roughness);
	float G = geometry_smith(N, V, wi, roughness);
	
	vec3 nominator = D * G * F;
	float denominator = 4 * max(dot(N, V), 0.0) * cos_theta + 0.001;
	vec3 specular = nominator / denominator;
	
	vec3 final = (diffuse_part * object_color / PI + specular) * radiance * cos_theta; 
	vec3 ambient = vec3(0.03) * object_color;
	
	final = final + ambient;
	final = pow(final, vec3(1.0 / 2.2));
	
	out_color = vec4(final, 1.0);
}