struct FragInput {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};


[[stage(fragment)]]
fn main(in: FragInput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}