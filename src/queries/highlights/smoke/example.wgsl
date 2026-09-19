// café WGSL fixture
enable f16;
diagnostic(off, derivative_uniformity);
alias Samples = array /* comment */ <vec4 // misleading >
  <f32>, 4>;
struct Vertex {
  @location(0) position: vec4<f32>,
  weights: array<array<u32, 4>, 2>,
}
const count: u32 = 4u;
override scale: f32 = 1.5f;
@group(0) @binding(1) var<storage, read_write> data: array<Vertex>;
const_assert count > 0u;
/* nested /* inner */ tail **/
fn helper(value: u32, pointer: ptr<function, u32>) -> u32 {
  let shifted = (value << 1u) >> 1u;
  var café = 0x2au;
  café += shifted;
  café >>= 1u;
  if value >= 1u && value != 2u { café = café + 1u; }
  else { café = 0u; }
  for (var i = 0u; i < count; i++) { café ^= i; }
  while café > 4u { café--; }
  loop {
    café++;
    continuing { break if café == 4u; }
  }
  switch café { case 0u: { break; } default: { café = 3u; } }
  return café;
}
@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) invocation: vec3<u32>) {
  let index = invocation.x;
  var local = index;
  let result = helper(index, &local);
  let matrix = mat2x2<f32>(1.0, 0.0, 0.0, 1.0);
  let fraction = 0x1.fp+2f;
  if true { data[index].position.xy = vec2<f32>(fraction); }
}
@fragment fn fragment_main() { discard; }
fn after() -> u32 { return 7u; }
