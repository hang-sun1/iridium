struct VertexInput {
    [[location(0)]] position: vec4<f32>;
    [[location(1)]]  color: vec4<f32>;
};

struct InstanceInput {
    [[location(2)]] mvp_0: vec4<f32>;
    [[location(3)]] mvp_1: vec4<f32>;
    [[location(4)]] mvp_2: vec4<f32>;
    [[location(5)]] mvp_3: vec4<f32>;
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
fn main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    let mvp = mat4x4<f32>(
        instance.mvp_0,
        instance.mvp_1,
        instance.mvp_2,
        instance.mvp_3,
    );
    var out: VertexOutput;
    let identity = mat4x4<f32>(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );
    out.pos = mvp * vec4<f32>(vertex.position.xyz, 1.0);
    // out.pos = vec4<f32>(vertex.position, 1.0);
    out.color = vertex.color.xzy;
    return out;
}