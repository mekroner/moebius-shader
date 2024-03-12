#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(3) var prepass_texture: texture_2d<f32>;
@group(0) @binding(4) var prepass_depth_texture: texture_depth_2d;
@group(0) @binding(5) var paper_texture: texture_2d<f32>;

struct PostProcessSettings {
    intensity: f32,
}

@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

var<private> SOBEL_X: array<f32,9> = array<f32, 9>(
    1, 0, -1,
    2, 0, -2,
    1, 0, -1,
);

var<private> SOBEL_Y: array<f32,9> = array<f32, 9>(
    1, 2, 1,
    0, 0, 0,
    -1, -2, -1,
);

var<private> uv_offsets: array<vec2<f32>,9> = array<vec2<f32>, 9>(
    vec2(-1, 1), vec2(0, 1), vec2(1, 1),
    vec2(-1, 0), vec2(0, 0), vec2(1, 0),
    vec2(-1, -1), vec2(0, -1), vec2(1, -1),
);

var<private> UV_OFFSET_CROSS: array<vec2<f32>,4> = array<vec2<f32>, 4>(
    vec2(1, 1), vec2(-1, -1),
    vec2(-1, 1), vec2(1, -1),
);

fn square(value: f32) -> f32 {
    return value * value;
}

fn dot_square(value: vec3<f32>) -> f32 {
    return dot(value, value);
}

fn depth_outline_cross(in_uv: vec2<f32>) -> f32 {
    let top_right = in_uv + UV_OFFSET_CROSS[0] * settings.intensity * 0.001;
    let bot_left = in_uv + UV_OFFSET_CROSS[1] * settings.intensity * 0.001;
    let top_left = in_uv + UV_OFFSET_CROSS[2] * settings.intensity * 0.001;
    let bot_right = in_uv + UV_OFFSET_CROSS[3] * settings.intensity * 0.001;

    let tr_sample = textureSample(prepass_depth_texture, texture_sampler, top_right) * 10;
    let bl_sample = textureSample(prepass_depth_texture, texture_sampler, bot_left) * 10;
    let tl_sample = textureSample(prepass_depth_texture, texture_sampler, top_left) * 10;
    let br_sample = textureSample(prepass_depth_texture, texture_sampler, bot_right) * 10;

    return sqrt(square(tr_sample - bl_sample) + square(tl_sample - br_sample)) * settings.intensity;
}

fn normal_outline_cross(in_uv: vec2<f32>) -> f32 {
    let top_right = in_uv + UV_OFFSET_CROSS[0] * settings.intensity * 0.001;
    let bot_left = in_uv + UV_OFFSET_CROSS[1] * settings.intensity * 0.001;
    let top_left = in_uv + UV_OFFSET_CROSS[2] * settings.intensity * 0.001;
    let bot_right = in_uv + UV_OFFSET_CROSS[3] * settings.intensity * 0.001;

    let tr_sample = textureSample(prepass_texture, texture_sampler, top_right).xyz;
    let bl_sample = textureSample(prepass_texture, texture_sampler, bot_left).xyz;
    let tl_sample = textureSample(prepass_texture, texture_sampler, top_left).xyz;
    let br_sample = textureSample(prepass_texture, texture_sampler, bot_right).xyz;

    return sqrt(dot_square(tr_sample - bl_sample) + dot_square(tl_sample - br_sample)) * settings.intensity;
}

fn normal_outline(in_uv: vec2<f32>) -> f32 {
    var sum_x = vec3(0.0);
    var sum_y = vec3(0.0);
    for (var i: u32 = 0u; i < 9u; i = i + 1) {
        let uv = in_uv + uv_offsets[i] * settings.intensity * 0.001;
        let sample = textureSample(prepass_texture, texture_sampler, uv);
        let normal = normalize(sample.xyz * 2.0 - 1.0);
        sum_x += normal * SOBEL_X[i];
        sum_y += normal * SOBEL_Y[i];
    }
    let X = length(sum_x);
    let Y = length(sum_y);
    let magnitude = sqrt(X * X + Y * Y);
    return magnitude;
}

fn depth_outline(in_uv: vec2<f32>) -> f32 {
    var sum_x = 0.0;
    var sum_y = 0.0;
    for (var i: u32 = 0u; i < 9u; i = i + 1) {
        let uv = in_uv + uv_offsets[i] * settings.intensity * 0.001;
        let sample = textureSample(prepass_depth_texture, texture_sampler, uv) * 10;
        sum_x += sample * SOBEL_X[i];
        sum_y += sample * SOBEL_Y[i];
    }
    let magnitude = sqrt(sum_x * sum_x + sum_y * sum_y);
    return magnitude;
}

fn step(x: f32, min: f32) -> f32 {
    if x > min {
        return 1.0;
    }
    return 0.0;
}

fn outline(uv: vec2<f32>) -> f32 {
    let depth_threashold = 0.5;
    let depth_sample = textureSample(prepass_depth_texture, texture_sampler, uv) * depth_threashold;
    let depth_out = step(depth_outline_cross(uv), depth_sample);
    let normal_out = step(normal_outline_cross(uv), 0.9);
    let outline = max(depth_out, normal_out);
    return outline;
}

fn grayscale(vec: vec4<f32>) -> f32 {
    let r_scale = 0.2989;
    let g_scale = 0.5870;
    let b_scale = 0.1140;
    return r_scale * vec.x + g_scale * vec.y + b_scale * vec.z;
}

fn invert(vec: vec4<f32>, cutoff: f32) -> vec4<f32> {
    return vec4(
        max(0.0, cutoff - vec.x),
        max(0.0, cutoff - vec.y),
        max(0.0, cutoff - vec.z),
        vec.w
    );
}

fn mix4x1(a: vec4<f32>, b: f32, c: f32) -> vec4<f32> {
    return vec4(
        mix(a.x, b, c),
        mix(a.y, b, c),
        mix(a.z, b, c),
        1.0
    );
}



@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let screen = textureSample(screen_texture, texture_sampler, in.uv);
    // paper
    let paper_strength = 1.0;
    let paper_inverse_strength = 0.4;
    let paper = mix4x1(vec4(1.0), grayscale(textureSample(paper_texture, texture_sampler, in.uv)), paper_strength);
    let paper_invert = invert(textureSample(paper_texture, texture_sampler, in.uv), 0.6);

    let outline = vec4(1.0 - vec3(outline(in.uv)), 1.0);
    return paper_invert * paper_inverse_strength + screen * outline * paper;
    //return sample;
}
