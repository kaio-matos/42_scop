#version 330 core
layout(location = 0) in vec4 aPos;
layout(location = 1) in vec3 aColor;
layout(location = 2) in vec3 aTexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 Color;
out vec3 TexCoord;

void main()
{
    vec4 pos = aPos;
    pos.w = 1.0;
    gl_Position = projection * view * model * pos;
    Color = aColor;
    TexCoord = aTexCoord;
}
