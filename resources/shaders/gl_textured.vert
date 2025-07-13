#version 330 core

in vec3 position;
in vec3 normal;
in vec2 tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec2 uv_offset;
uniform vec2 uv_scale;

out vec3 v_normal;
out vec2 v_tex;
out vec3 v_position;

void main() {
    mat4 modelview = view * model;
    v_normal   = transpose(inverse(mat3(modelview))) * normal;
    v_tex      = tex_coords * uv_scale + uv_offset;
    v_position = (modelview * vec4(position, 1.0)).xyz;
    gl_Position = projection * modelview * vec4(position, 1.0);
}