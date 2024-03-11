#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(3) var prepass_texture: texture_2d<f32>;

struct PostProcessSettings {
    intensity: f32,
}

@group(0) @binding(2) var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Chromatic aberration strength
    let offset_strength = settings.intensity;
    let normal = textureSample(prepass_texture, texture_sampler, in.uv);
    //let normal = textureLoad(prepass_texture, vec2<i32>(1,1), 4);
    let out = normalize(normal.xyz * 2.0 - 1.0);
    return vec4(out, 1.0);

    // Sample each color channel with an arbitrary shift
    // return vec4<f32>(
        // textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(offset_strength, -offset_strength)).r,
        // textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(-offset_strength, 0.0)).g,
        // textureSample(screen_texture, texture_sampler, in.uv + vec2<f32>(0.0, offset_strength)).b,
        // 1.0
    // );
}
