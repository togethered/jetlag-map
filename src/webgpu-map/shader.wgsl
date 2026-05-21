struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
};

@vertex
fn vertex_main(
    @builtin(vertex_index) index: u32,
) -> VertexOutput {
    const pos = array(
        vec2(0.0, 0), vec2(1, 0), vec2(1, 1),
    );
    var result: VertexOutput;
    result.position = vec4(pos[index] * 0.5 - vec2(1), 0, 1);
    result.tex_coord = pos[index];
    return result;
}

@fragment
fn fragment_main(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(vertex.tex_coord, 0, 1);
}
