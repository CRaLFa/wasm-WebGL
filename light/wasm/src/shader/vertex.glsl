#version 300 es

layout(location = 0) in vec3 position;
layout(location = 1) in vec4 color;
layout(location = 2) in vec3 normal;

uniform mat4 mvpMatrix;

out vec4 vertexColor;
out vec3 vertexNormal;

void main() {
    vertexColor = color;
    vertexNormal = normal;

    gl_Position = mvpMatrix * vec4(position, 1.0);
}
