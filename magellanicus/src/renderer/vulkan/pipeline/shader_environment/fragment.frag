#version 450

#include "shader_environment_data.glsl"

#define USE_LIGHTMAPS
#include "../include/material.frag"
#include "../include/blend.frag"

layout(location = 0) out vec4 f_color;

layout(location = 0) in vec2 base_map_texture_coordinates;
layout(location = 1) in vec2 lightmap_texture_coordinates;

layout(set = 2, binding = 1) uniform sampler map_sampler;
layout(set = 2, binding = 2) uniform texture2D base_map;
layout(set = 2, binding = 3) uniform texture2D primary_detail_map;
layout(set = 2, binding = 4) uniform texture2D secondary_detail_map;
layout(set = 2, binding = 5) uniform texture2D micro_detail_map;
layout(set = 2, binding = 6) uniform texture2D bump_map;

vec4 blend_with_mix_type(vec4 color, vec4 with, uint blend_type, float alpha) {
    vec4 blender;

    switch(blend_type) {
        case 0:
            blender = double_biased_multiply(color, with);
            break;
        case 1:
            blender = multiply(color, with);
            break;
        case 2:
            blender = double_biased_add(color, with);
            break;
        default:
            return vec4(0.0);
    }

    return mix(color, vec4(blender.rgb, 1.0), alpha * with.a);
}

void main() {
    vec4 base_map_color = texture(sampler2D(base_map, map_sampler), base_map_texture_coordinates);

    vec4 bump_color = texture(
        sampler2D(bump_map, map_sampler),
        base_map_texture_coordinates * shader_environment_data.bump_map_scale
    );

    if((shader_environment_data.flags & 1) == 1 && (base_map_color.a <= 0.0 || bump_color.a <= 0.0)) {
        discard;
    }

    vec4 primary_detail_map_color = texture(
        sampler2D(primary_detail_map, map_sampler),
        base_map_texture_coordinates * shader_environment_data.primary_detail_map_scale
    );

    vec4 secondary_detail_map_color = texture(
        sampler2D(secondary_detail_map, map_sampler),
        base_map_texture_coordinates * shader_environment_data.secondary_detail_map_scale
    );

    vec4 micro_detail_map_color = texture(
        sampler2D(micro_detail_map, map_sampler),
        base_map_texture_coordinates * shader_environment_data.micro_detail_map_scale
    );

    vec4 lightmap_color = texture(
        sampler2D(lightmap_texture, lightmap_sampler),
        lightmap_texture_coordinates
    );

    vec4 scratch_color = base_map_color;
    scratch_color = blend_with_mix_type(scratch_color, primary_detail_map_color, shader_environment_data.detail_map_function, base_map_color.a);
    scratch_color = blend_with_mix_type(scratch_color, secondary_detail_map_color, shader_environment_data.detail_map_function, 1.0 - base_map_color.a);
    scratch_color = blend_with_mix_type(scratch_color, micro_detail_map_color, shader_environment_data.micro_detail_map_function, micro_detail_map_color.a);
    scratch_color = vec4(scratch_color.rgb * lightmap_color.rgb, 1.0);
    f_color = scratch_color;
}
