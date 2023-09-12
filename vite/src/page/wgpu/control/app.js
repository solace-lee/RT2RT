import { Renderer } from "../view/renderer";
import { Scene } from "../model/scene";

export class App {
  // canvas: HTMLCanvasElement;
  // renderer: Renderer;
  // scene: Scene;

  constructor(canvas) {
    this.canvas = canvas;
    this.renderer = new Renderer(canvas);
    this.scene = new Scene();
    this.forwards_amount = 0;
    this.right_amount = 0;
  }

  async inititalize() {
    await this.renderer.Initialize();
  }

  mouse_move(movementX, movementY) {
    this.scene.spin_player(movementX / 10, movementY / 10);
  }

  run = () => {
    let running = true;

    this.scene.update();
    this.scene.move_player(this.forwards_amount, this.right_amount);

    this.renderer.render(
      this.scene.get_renderables(),
      this.scene.player
    );

    if (running) {
      requestAnimationFrame(this.run);
    }
  };
}
