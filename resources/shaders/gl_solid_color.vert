#version 330 core
in  vec3 position;
in  vec3 normal;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
out vec3 v_normal;
void main() {
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    gl_Position = projection * modelview * vec4(position, 1.0);
}
