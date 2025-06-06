// Vertex input layout matching: pos: [f32; 2], color: [f32; 3]
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

// Output to fragment shader
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4<f32>(input.position, 0.0, 1.0);
    output.color = input.color;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.color, 1.0);
}
