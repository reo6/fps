#version 330 core

in vec3 position;

uniform mat4 view;

uniform mat4 projection;

out vec3 direction;

void main() {
    direction = position;
    vec4 pos = projection * view * vec4(position, 1.0);
    gl_Position = pos.xyww;
} 