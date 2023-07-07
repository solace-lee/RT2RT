import { Triangle } from "./triangle";
import { Camera } from "./camera";
import { vec3 } from "gl-matrix";

export class Scene {
  // triangles: Triangle[];
  // player: Camera;

  constructor() {
    this.triangles = [];
    this.triangles.push(new Triangle([2, 0, 0], 0));
    this.player = new Camera([-2, 0, 0.5], 0, 0)
  }

  update() {
    this.triangles.forEach(
      (triangle) => triangle.update()
    )
    this.player.update()
  }

  spin_player(dX, dY) {
    this.player.eulers[2] -= dX;
    this.player.eulers[2] %= 360;

    this.player.eulers[1] = Math.min(
      89, Math.max(
        -89,
        this.player.eulers[1] + dY
      )
    )
  }

  move_player(forwards_amount, right_amount) {
    vec3.scaleAndAdd(
      this.player.position, this.player.position,
      this.player.forwards, forwards_amount
    )

    vec3.scaleAndAdd(
      this.player.position, this.player.position,
      this.player.right, right_amount
    )
  }

  get_player() {
    return this.player;
  }

  get_triangles() {
    return this.triangles;
  }
}
