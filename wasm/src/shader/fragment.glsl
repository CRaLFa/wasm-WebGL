#version 300 es

precision highp float;

in vec4 vertexColor;
in vec3 vertexNormal;

uniform mat4 invMatrix;
uniform vec3 lightDirection;
uniform vec3 eyeDirection;
uniform vec3 ambientColor;

out vec4 fragmentColor;

void main() {
    vec3 invLight = normalize(invMatrix * vec4(lightDirection, 1.0)).xyz;
    vec3 invEye = normalize(invMatrix * vec4(eyeDirection, 1.0)).xyz;
    vec3 halfVector = normalize(invLight + invEye);

    float diffuse = clamp(dot(invLight, vertexNormal), 0.1, 1.0);
    float specular = pow(clamp(dot(halfVector, vertexNormal), 0.0, 1.0), 25.0);

    fragmentColor = vec4(vertexColor.rgb * diffuse + specular + ambientColor, vertexColor.a);
}
