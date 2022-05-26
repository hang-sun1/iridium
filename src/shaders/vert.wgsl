struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]]  color: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

struct MatrixData {
    mvp: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> uni_data: MatrixData;

[[stage(vertex)]]
fn main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.pos = uni_data.mvp * vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color;
    return out;
}