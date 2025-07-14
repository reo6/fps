#version 330 core

in vec3 direction;

out vec4 frag_color;

uniform sampler2D equirect;

const vec2 inv_atan = vec2(0.15915494309, 0.31830988618);

vec2 sample_spherical_map(vec3 v) {
    vec2 uv = vec2(atan(v.z, v.x), asin(v.y));
    uv *= inv_atan;
    uv += 0.5;
    return uv;
}

void main() {
    vec2 uv = sample_spherical_map(normalize(direction));
    vec3 color = texture(equirect, uv).rgb;
    frag_color = vec4(color, 1.0);
} 