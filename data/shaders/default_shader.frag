#version 330 core

#define MAX_POINT_LIGHTS 20

struct Material {
    sampler2D texture;
    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
    float shininess;
};

struct PointLight {
    vec3 position;

    float constant;
    float linear;
    float quadratic;

    vec3 ambient;
    vec3 diffuse;
    vec3 specular;
};


in vec2 frag_TextureCoordinates;
in vec3 frag_Normal;
in vec3 frag_FragPosition;


out vec4 Color;

uniform Material material;
uniform int pointLightCount;
uniform PointLight pointLights[MAX_POINT_LIGHTS];

vec3 calculate_point_light(PointLight light, vec3 normal, vec3 fragment_position, vec3 view_direction)
{
    vec3 lightDirection = normalize(light.position - fragment_position);

    vec3 fragColor = vec3(mix(texture(material.texture, frag_TextureCoordinates), vec4(material.ambient, 1.0), 0.5));
    // Ambient
    vec3 ambient = light.ambient * fragColor;

    // Diffuse
    float diff = max(dot(normal, lightDirection), 0.0);
    vec3 diffuse = light.diffuse * diff * fragColor;

    // Specular
    float specularStrength = 0.5;
    vec3 reflectDir = reflect(-lightDirection, normal);
    float spec = pow(max(dot(view_direction, reflectDir), 0.0), material.shininess);
    vec3 specular = light.specular * (spec * material.specular);

    // Attenuation
    float distance = length(light.position - frag_FragPosition);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
    ambient *= attenuation;
    diffuse *= attenuation;
    specular *= attenuation;

    return ambient + diffuse + specular;
}

void main()
{
    vec3 norm = normalize(frag_Normal);
    vec3 viewDir = normalize(-frag_FragPosition);

    vec3 outputColor = vec3(0);
    for(int i = 0; i < pointLightCount; i++)
    outputColor += calculate_point_light(pointLights[i], norm, frag_FragPosition, viewDir);

    Color = vec4(outputColor, 1.0);
}
