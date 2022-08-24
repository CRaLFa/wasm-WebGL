#version 300 es

layout(location = 0) in vec3 position;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 texCoord;

uniform mat4 modelMatrix;
uniform mat4 mvpMatrix;
uniform mat4 normalMatrix;

out vec3 vertexPosition;
out vec3 vertexNormal;
out vec2 vertexCoord;

void main() {
    vertexPosition = (modelMatrix * vec4(position, 1.0)).xyz;
    vertexNormal = (normalMatrix * vec4(normal, 1.0)).xyz;
    vertexCoord = texCoord;

    gl_Position = mvpMatrix * vec4(position, 1.0);
}
