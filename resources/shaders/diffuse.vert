#version 330 core

layout(location = 0) in vec3 position1;
layout(location = 1) in vec3 normal1;
layout(location = 2) in vec3 color1;
layout(location = 3) in vec2 vertexTexCoord1;

out vec3 position;
out vec3 normal;
out vec3 color;
out vec2 vertexTexCoord;

uniform mat4 projection;
uniform mat4 view;
uniform mat4 model;

void main() {
    gl_Position = projection * view * model * vec4(position1, 1.0);
    position = position1;
    normal = normal1;
    color = color1;
    vertexTexCoord = vertexTexCoord1;
}
