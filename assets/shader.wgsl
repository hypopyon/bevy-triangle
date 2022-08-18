struct View {
    proj: mat4x4<f32>,
    world_pos: vec3<f32>,
};
@group(0) @binding(0)
var<uniform> view: View;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) col: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_v_index: u32,
    @location(0) pos: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) col: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = view.proj * vec4<f32>(pos, 1.0);
    out.uv = uv;
    out.col = col;
    return out;
}

// @group(1) @binding(0)
// var sprite_texture: texture_2d<f32>;
// @group(1) @binding(1)
// var sprite_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.col // * textureSample(sprite_texture, sprite_sampler, in.uv)
    ;
}