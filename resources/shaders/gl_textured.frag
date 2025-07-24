#version 330 core

in  vec3 v_normal;
in  vec2 v_tex;
in  vec3 v_position;

out vec4 frag_color;

uniform vec3 u_light;
uniform sampler2D tex;
uniform vec3 color; // base colour factor (acts as solid colour when no texture)

void main() {
    // Combine base texture (or constant white) with colour factor supplied by CPU.
    vec3 base_col = texture(tex, v_tex).rgb * color;

    vec3 ambient_color  = base_col * 0.2;
    vec3 diffuse_color  = base_col * 0.6;
    vec3 specular_color = vec3(1.0);

    // u_light is the direction **from the light towards the fragment**.
    float diffuse = max(dot(normalize(v_normal), normalize(u_light)), 0.0);

    vec3 camera_dir = normalize(-v_position);
    vec3 half_dir   = normalize(normalize(u_light) + camera_dir);
    float specular  = pow(max(dot(half_dir, normalize(v_normal)), 0.0), 16.0);

    vec3 result = ambient_color + diffuse * diffuse_color + specular * specular_color;

    // Convert from linear to sRGB for display (approximate γ-correction)
    result = pow(result, vec3(1.0 / 2.2));

    frag_color = vec4(result, 1.0);
}