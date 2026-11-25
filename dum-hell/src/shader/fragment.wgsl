struct MaterialUniform {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: MaterialUniform;

struct FragmentInput {
    @location(0) normal: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@fragment
fn fs_main(input: FragmentInput) -> @location(0) vec4<f32> {
    return material.color;
}
