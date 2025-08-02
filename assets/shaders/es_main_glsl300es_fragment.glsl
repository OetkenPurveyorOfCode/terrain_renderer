#version 300 es
precision mediump float;
precision highp int;

uniform highp vec4 fs_params[1];
uniform highp sampler2D dirt_texture_dirt_sampler;
uniform highp sampler2D grass_texture_grass_sampler;
uniform highp sampler2D rock_texture_rock_sampler;
uniform highp sampler2D snow_texture_snow_sampler;

in highp vec3 normal;
in highp vec3 pos;
in highp vec2 texcoord;
layout(location = 0) out highp vec4 FragColor;
in highp vec4 color;

void main()
{
    highp vec4 texcolor;
    if (pos.y < 0.25)
    {
        texcolor = texture(dirt_texture_dirt_sampler, texcoord);
    }
    else
    {
        bool _58 = 0.25 <= pos.y;
        bool _65;
        if (_58)
        {
            _65 = pos.y < 0.5;
        }
        else
        {
            _65 = _58;
        }
        if (_65)
        {
            texcolor = mix(texture(dirt_texture_dirt_sampler, texcoord), texture(grass_texture_grass_sampler, texcoord), vec4(4.0 * (pos.y - 0.25)));
        }
        else
        {
            bool _92 = 0.5 <= pos.y;
            bool _99;
            if (_92)
            {
                _99 = pos.y < 0.75;
            }
            else
            {
                _99 = _92;
            }
            if (_99)
            {
                texcolor = mix(texture(grass_texture_grass_sampler, texcoord), texture(rock_texture_rock_sampler, texcoord), vec4(4.0 * (pos.y - 0.5)));
            }
            else
            {
                texcolor = mix(texture(rock_texture_rock_sampler, texcoord), texture(snow_texture_snow_sampler, texcoord), vec4(4.0 * (pos.y - 0.75)));
            }
        }
    }
    FragColor = texcolor * max(0.300000011920928955078125, dot(fs_params[0].xyz, normalize(normal)));
}

