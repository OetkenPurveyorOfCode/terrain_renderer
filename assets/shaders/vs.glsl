@vs vs
layout (location = 0) in vec3 in_pos;
layout (location = 1) in vec2 in_uv;
layout (location = 2) in vec4 in_color;
layout (location = 3) in vec3 in_normal;

layout(binding=0) uniform vs_params {
    mat4 model;
    mat4 projection;
};

out vec3 pos;
out vec4 color;
out vec3 normal;
out vec2 texcoord;
void main() {
    gl_Position = projection*model*vec4(in_pos, 1);
    color = vec4(in_pos.y, in_pos.y, in_pos.y, 1.0);
    pos = in_pos;
    normal = in_normal;
    texcoord = in_uv;
}
@end

@fs fs
in vec3 pos;
in vec4 color;
in vec3 normal;
in vec2 texcoord;

layout(binding=1) uniform fs_params {
    vec3 light_dir;
};
layout (binding = 2) uniform texture2D snow_texture;
layout (binding = 2) uniform sampler snow_sampler;
layout (binding = 3) uniform texture2D grass_texture;
layout (binding = 3) uniform sampler grass_sampler;
layout (binding = 4) uniform texture2D rock_texture;
layout (binding = 4) uniform sampler rock_sampler;
layout (binding = 5) uniform texture2D dirt_texture;
layout (binding = 5) uniform sampler dirt_sampler;

out vec4 FragColor;

void main() {
    float diffuse = dot(light_dir, normalize(normal));
    diffuse = max(0.3, diffuse);
    vec4 texcolor;
    if (pos.y < 0.25) {
        texcolor = texture(sampler2D(dirt_texture, dirt_sampler), texcoord);
    }
    else if (0.25 <= pos.y && pos.y < 0.5) {
        float t = 4.*(pos.y-0.25);
        texcolor = mix(texture(sampler2D(dirt_texture, dirt_sampler), texcoord), texture(sampler2D(grass_texture, grass_sampler), texcoord), t);
    }
    else if (0.5 <= pos.y && pos.y < 0.75) {
        float t = 4.*(pos.y-0.5);
        texcolor = mix(texture(sampler2D(grass_texture, grass_sampler), texcoord), texture(sampler2D(rock_texture, rock_sampler), texcoord), t);
    }
    else {
        float t = 4.*(pos.y-0.75);
        texcolor = mix(texture(sampler2D(rock_texture, rock_sampler), texcoord), texture(sampler2D(snow_texture, snow_sampler), texcoord), t);
    }
    FragColor = diffuse*texcolor ;
}
@end

@program main vs fs
