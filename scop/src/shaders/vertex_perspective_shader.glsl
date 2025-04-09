#version 330 core
layout(location = 0) in vec4 aPos;
layout(location = 1) in vec3 aColor;
layout(location = 2) in vec2 aTexCoord;
layout(location = 3) in float aFaceId;
layout(location = 4) in float aMaxFaceId;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 Color;
out vec2 TexCoord;
out float FaceId;
out float MaxFaceId;

void main()
{
    vec4 pos = aPos;
    pos.w = 1.0;
    gl_Position = projection * view * model * pos;
    Color = aColor;
    TexCoord = aTexCoord;
    FaceId = aFaceId;
    MaxFaceId = aMaxFaceId;
}
