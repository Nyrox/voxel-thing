#version 420 core

in vec3 frag_position;
in vec3 frag_normal;
in vec2 uv;

out vec4 color;

layout(binding = 0) uniform sampler2D t_color;


const float PI = 3.1415926;

float square(float v) {
    return v * v;
}

uniform vec3 cameraPos;
uniform int gTime;

void main() {
    vec3 lightPos = vec3(sin(gTime / 1000.0) * 20, 5, 15);
    vec3 lightStrength = vec3(3, 3, 3);

    vec3 objColor = vec3(0.3, 0.5, 0.2);
    objColor = texture(t_color, uv).rgb;

    vec3 wi = normalize(lightPos - frag_position);
    vec3 V = normalize(cameraPos - frag_position);
    vec3 H = normalize(wi + V);
    vec3 N = frag_normal;

    float NdotH = max(dot(N, H), 0.0);
    float specularHardness = 16.0;

    float distance = distance(lightPos, frag_position);
    float cos_theta = max(dot(N, wi), 0.0);

    float attenuation = 1.0 / square(distance);
    vec3 ambient = vec3(0.01, 0.01, 0.01);

    vec3 radiance = lightStrength * attenuation * cos_theta + ambient;

    vec3 diffuse = radiance * objColor;
    vec3 specular = pow(NdotH, specularHardness) * radiance;

    color = vec4(diffuse + specular, 1.0);
    //color = vec4(uv.xy, 0.0, 1.0);
}
