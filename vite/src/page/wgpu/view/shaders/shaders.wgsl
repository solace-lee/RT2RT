struct TransformData {
  view: mat4x4<f32>,
  projection: mat4x4<f32>,
}

struct ObjectData {
  model: array<mat4x4<f32>>,

}
// 帧组
@binding(0) @group(0) var<uniform> tranformUBO: TransformData;
@binding(1) @group(0) var<storage, read> objects: ObjectData;
// 材质
@binding(0) @group(1) var myTexture: texture_2d<f32>;
@binding(1) @group(1) var mySampler: sampler;

struct Fragment {
  @builtin(position) Position : vec4<f32>,
  @location(0) TexCoord : vec2<f32>,
};

@vertex
fn vs_main(
  @builtin(instance_index) id: u32,
  @location(0) vertexPosition: vec3<f32>,
  @location(1) vertexTextCoord: vec2<f32>,
) -> Fragment {

  var output : Fragment;
  output.Position = tranformUBO.projection * tranformUBO.view * objects.model[id] * vec4<f32>(vertexPosition, 1.0);
  output.TexCoord = vertexTextCoord;
  return output;
};

@fragment
fn fs_main(@location(0) TexCoord: vec2<f32>) -> @location(0) vec4<f32> {
  return textureSample(myTexture, mySampler, TexCoord);
};