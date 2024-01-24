export class Sphere {
  constructor(center, radius, color) {
    this.center = new Float32Array(center)
    this.radius = radius
    this.color = new Float32Array(color)
  }
}