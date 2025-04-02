#version 330 core
out vec4 FragColor;

in vec3 Color;
in vec2 TexCoord;

uniform sampler2D object_texture;
uniform float texture_percentage;

void main()
{
    vec4 texture_color = texture(object_texture, TexCoord);

    float color_percentage = 1.0 - texture_percentage;
    vec4 color = vec4(Color, 1.0);

    FragColor = texture_color * texture_percentage + color * color_percentage;
}
