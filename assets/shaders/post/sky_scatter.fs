#version 420


in vec2 uv;
out vec4 frag_color;

layout(binding = 0) uniform sampler2D color;
layout(binding = 1) uniform sampler2D depth;

vec3 sundir;
vec3 I_R, I_M;
vec2 totalDepthRM;


    // consts
    const vec3 rayScatterCoeff = vec3(58e-7, 135e-7, 331e-7);
    const vec3 rayEffectiveCoeff = rayScatterCoeff; // Rayleight doesn't absorb light

    const vec3 mieScatterCoeff = vec3(2e-5);
    const vec3 mieEffectiveCoeff = mieScatterCoeff * 1.1; // Approximate absorption as a factor of scattering

    const vec3 sunIntensity = vec3(7); // sun is modelled as infinitely far away

    // const float earthRadius = 6360e3;
    // const float atmosphereRadius = 6380e3;
	const float earthRadius = 0;
	const float atmosphereRadius = 20e3;
	const float hRay = 17e3;
	const float hMie = 12e2;

    const vec3 center = vec3(0., -earthRadius, 0.); // earth center point


uniform float densityCoeff = 1.2;
uniform float mieCoeff = 1.0;
uniform float rayCoeff = 1.0;

// Basically a ray-sphere intersection. Find distance to where rays escapes a sphere with given radius.
// Used to calculate length at which ray escapes atmosphere
float escape(vec3 p, vec3 d, float R) {
	vec3 v = p - center;
	float b = dot(v, d);
	float det = b * b - dot(v, v) + R*R;
	if (det < 0.) return -1.;
	det = sqrt(det);
	float t1 = -b - det, t2 = -b + det;
	return (t1 >= 0.) ? t1 : t2;
}

vec2 densitiesRM (vec3 p) {
	float h = max(0.,length(p - center) - earthRadius);
    return vec2(
		exp(-h/hRay) * rayCoeff,
		exp(-h/hMie) * mieCoeff
	) * densityCoeff;
}

vec2 scatterDepthInt (vec3 o, vec3 d, float L, float steps) {
    // Approximate sampling the middle
    // return densitiesRM (o + d * (L / 2.)) * L;

    // Approximate by combining 2 samples
   	return (densitiesRM (o) * (L / 2.) + densitiesRM (o + d * L) * (L / 2.));

    // Integrate

    // Accumulator
	vec2 depthRMs = vec2(0.);

	// Set L to be step distance and pre-multiply d with it
	L /= steps; d *= L;

	// Go from point P to A
	for (float i = 0.; i < steps; ++i)
		// Simply accumulate densities
		depthRMs += densitiesRM(o + d * i);

	return depthRMs * L;
}

void scatterIn (vec3 origin, vec3 direction, float depth, float steps) {


    depth = depth / steps;

    for (float i = 0.; i < steps; i++) {
    	vec3 p = origin + direction * (depth * i);
    	vec2 dRM = densitiesRM(p) * depth;
    	totalDepthRM += dRM;

        // Calculate optical depth
        vec2 depthRMsum = totalDepthRM + scatterDepthInt(p, sundir, escape(p, sundir, atmosphereRadius), 4.);

        // Calculate exponent part of both integrals
		vec3 A = exp(-rayEffectiveCoeff * depthRMsum.x - mieEffectiveCoeff * depthRMsum.y);

        I_R += A * dRM.x;
        I_M += A * dRM.y;
    }
}

// Calculate the complete scattering effect for a ray going from origin
// along direction for a distance of depth.
// luminance represents the light coming from that point e.g. in a scene
// If luminance is zero the effective return is in-scattering from the sun
vec3 scatter (vec3 origin, vec3 direction, float depth, vec3 luminance) {

	I_R = I_M = vec3(0.);

    scatterIn (origin, direction, depth, 12.);

    float mu = dot(direction, sundir);

    vec3 extinction = luminance * exp(-rayEffectiveCoeff * totalDepthRM.x - mieEffectiveCoeff * totalDepthRM.y);

    return extinction
        	// Add in-scattering
		+ sunIntensity * (1. + mu * mu) * (
            // 3/16pi = 0.597
			I_R * rayEffectiveCoeff * .0597 +
            // mie phase function is complicated [Nisita et al. 93, Riley et al. 04]
			I_M * mieScatterCoeff * .0196 / pow(1.58 - 1.52 * mu, 1.5));
}

uniform float nearPlane;
uniform float farPlane;

uniform mat4 inverseView;
uniform mat4 inverseProj;

uniform int iTime;
uniform vec3 cameraPosition;

void main ()
{

	float FOV = 75.0;
	float ASPECT = 16.0 / 9.0;

	// calculate view ray
	vec2 duv = uv * 2.0 - 1.0; // [[0, 0], [1, 1]] -> [[-1, -1], [1, 1]]
	duv = tan(FOV * 0.5) * duv;

	vec4 ray_clip = vec4(duv, -1.0, 1.0);
	vec4 ray_eye = inverseProj * ray_clip;
	ray_eye = vec4(ray_eye.xy, -1.0, 0.0);
	vec3 ray_wor = (inverseView * ray_eye).xyz;
	vec3 direction = normalize(ray_wor);

	// 5min cycle: frameCount * seconds * 5
	float cycleTime = 1000 * 15;
	float iTime = float(iTime) / cycleTime;

    float cycle = (iTime);

	sundir = normalize(vec3(
		cos(3.14 / 2.0) * cos(cycle),
		sin(3.14 / 2.0) * cos(cycle),
		sin(cycle)
	));

    vec3 origin = cameraPosition;

	vec3 col = texture(color, uv).rgb;
	float L = texture(depth, uv).r;

	float sampledDepth = texture(depth, uv).r;
	float realDepth = sampledDepth * (farPlane - nearPlane) + nearPlane;
	float depth;

    // Here you would do scene-intersection
	if (realDepth > farPlane * 0.9999999) {
		depth = escape (origin, direction, atmosphereRadius);
	} else {
		depth = realDepth;
	}

    col = scatter (origin, direction, depth, col);
	col = pow(col, vec3(1.0/2.2));
    frag_color = vec4(col, 1.);
}
