#version 300 es

precision highp float;

in vec3 vertexPosition;
in vec3 vertexNormal;
in vec2 vertexCoord;

uniform vec3 lightPosition;
uniform vec3 eyePosition;
uniform vec3 ambientColor;
uniform sampler2D sampler;

out vec4 fragmentColor;

void main() {
    vec3 light = normalize(lightPosition - vertexPosition);
    vec3 eye = normalize(vertexPosition - eyePosition);
    vec3 reflection = normalize(reflect(eye, vertexNormal));

    float diffuse = max(dot(light, vertexNormal), 0.2);
    float specular = pow(max(dot(light, reflection), 0.0), 25.0);

    vec4 samplerColor = texture(sampler, vertexCoord);
    fragmentColor = vec4(samplerColor.rgb * diffuse + specular + ambientColor, samplerColor.a);
}
