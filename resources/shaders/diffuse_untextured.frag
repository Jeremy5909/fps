#version 330 core

in vec3 position;
in vec3 normal;
in vec3 color;

out vec4 FragColor;

struct Light {
    vec3 position;
    vec4 color;
};

uniform int lightCount;
uniform Light lights[8];
uniform vec4 albedo;

void main() {
    vec3 result = vec3(0.0);
    vec3 norm = normalize(normal);

    for (int i = 0; i < lightCount; i++) {
        vec3 lightDir = normalize(lights[i].position - position);

        // diffuse
        float diff = max(dot(norm, lightDir), 0.0);

        vec3 lightColor = lights[i].color.rgb;
        result += (0.2 + diff) * lightColor;
    }
    FragColor = albedo * vec4(result, 1.0);
}
