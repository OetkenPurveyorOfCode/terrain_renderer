#version 300 es

uniform vec4 vs_params[8];
layout(location = 0) in vec3 in_pos;
out vec4 color;
out vec3 pos;
out vec3 normal;
layout(location = 3) in vec3 in_normal;
out vec2 texcoord;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in vec4 in_color;

void main()
{
    gl_Position = (mat4(vs_params[4], vs_params[5], vs_params[6], vs_params[7]) * mat4(vs_params[0], vs_params[1], vs_params[2], vs_params[3])) * vec4(in_pos, 1.0);
    color = vec4(in_pos.y, in_pos.y, in_pos.y, 1.0);
    pos = in_pos;
    normal = in_normal;
    texcoord = in_uv;
}

