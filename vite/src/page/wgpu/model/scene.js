import { Triangle } from "./triangle";
import { Camera } from "./camera";
import { vec3, mat4 } from "gl-matrix";

export class Scene {
  // triangles: Triangle[];
  // player: Camera;
  // object_data: Float32Array;
  // triangle_count: number

  constructor() {
    this.triangles = [];
    this.object_data = new Float32Array(16 * 1024);
    this.triangle_count = 0;

    let i = 0;
    for (let y = -5; y < 5; y++) {
      this.triangles.push(new Triangle([2, y, 0], 0));
      
      let blank_matrix = mat4.create();
      for (let j = 0; j < 16; j++) {
        this.object_data[16 * i + j] = blank_matrix.at(j);
      }

      i++
      this.triangle_count++
    }

    this.player = new Camera([-2, 0, 0.5], 0, 0);
  }

  update() {
    let i = 0

    this.triangles.forEach((triangle) => {
      triangle.update()
      let model = triangle.get_model()
      for (let j = 0; j < 16; j++) {
        this.object_data[16 * i + j] = model.at(j);
      }
      i++;
    });
    this.player.update();
  }

  spin_player(dX, dY) {
    this.player.eulers[2] -= dX;
    this.player.eulers[2] %= 360;

    this.player.eulers[1] = Math.min(
      89,
      Math.max(-89, this.player.eulers[1] + dY)
    );
  }

  move_player(forwards_amount, right_amount) {
    vec3.scaleAndAdd(
      this.player.position,
      this.player.position,
      this.player.forwards,
      forwards_amount
    );

    vec3.scaleAndAdd(
      this.player.position,
      this.player.position,
      this.player.right,
      right_amount
    );
  }

  get_player() {
    return this.player;
  }

  get_triangles() {
    return this.object_data;
  }
}
