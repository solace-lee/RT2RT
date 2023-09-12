import { mat4 } from "gl-matrix";

export const object_types = {
  TRIANGLE: 0,
  QUAD: 1,
};

export const pipeline_types = {
  SKY: 0,
  STANDARD: 1,
};

// export const RenderData = {
//   view_transform: mat4,
//   model_transforms: Float32Array,
//   object_counts: {
//     [obj in object_types] : Number
//   }
// }
