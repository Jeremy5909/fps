#version 330 core

in vec3 position;
in vec3 normal;
in vec3 color;
in vec2 vertexTexCoord;

out vec4 FragColor;

uniform sampler2D diffuse0;
uniform vec4 lightColor;
uniform vec3 lightPos;

void main() {
    float ambient = 0.20f;

    vec3 normal = normalize(normal);
    vec3 lightDirection = normalize(lightPos - position);
    float diffuse = max(dot(normal, lightDirection), 0.0f);

    FragColor = texture(diffuse0, vertexTexCoord) * lightColor * (diffuse + ambient);
}
