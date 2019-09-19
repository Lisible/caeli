#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Normal;
layout (location = 2) in vec2 TexCoords;

out vec2 frag_TextureCoordinates;
out vec3 frag_FragPosition;
out vec3 frag_Normal;

uniform mat4 world;
uniform mat4 view;
uniform mat4 proj;

void main()
{
    gl_Position = proj * view * world * vec4(Position, 1.0);
    frag_FragPosition = vec3(view * world * vec4(Position, 1.0));
    frag_TextureCoordinates = TexCoords;
    frag_Normal = mat3(transpose(inverse(view * world))) * Normal;
}
