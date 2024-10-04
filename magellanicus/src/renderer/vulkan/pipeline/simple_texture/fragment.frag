#version 450

layout(location = 0) in vec2 tex_coords;
layout(location = 1) in vec2 lightmap_texcoords;
layout(location = 0) out vec4 f_color;

layout(set = 1, binding = 0) uniform sampler lightmap_sampler;
layout(set = 1, binding = 1) uniform texture2D lightmap_texture;

layout(set = 2, binding = 0) uniform sampler s;
layout(set = 2, binding = 1) uniform texture2D tex;

void main() {
    vec4 lightmap_color = texture(sampler2D(lightmap_texture, lightmap_sampler), lightmap_texcoords);
    vec4 color = texture(sampler2D(tex, s), tex_coords);

    f_color = vec4(color.rgb * lightmap_color.rgb, 1.0);
}
