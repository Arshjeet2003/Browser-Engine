struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec2<f32>, @location(1) color: vec3<f32>) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(pos, 0.0, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(@location(0) in_color: vec3<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(in_color, 1.0);
}


