import { vec3, mat4 } from "gl-matrix";

export class Quad {
  // position: vec3;
  // model: mat4;

  constructor(position) {
    this.position = position;
  }

  update() {
    this.model = mat4.create();
    mat4.translate(this.model, this.model, this.position);
  }

  get_model() {
    return this.model;
  }
}
