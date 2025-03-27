#version 330 core
out vec4 FragColor;

in vec3 Color;
in vec3 TexCoord;

uniform sampler3D ourTexture;

void main()
{
    // FragColor = texture(ourTexture, TexCoord) * vec4(Color, 1.0);
    FragColor = texture(ourTexture, TexCoord);
}
