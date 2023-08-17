// import { vec2, vec3 } from "gl-matrix";

class ObjMesh {
  // buffer,
  // bufferLayout,
  // v
  // vt
  // vn
  // vertices
  // vertexCount
  constructor() {
    this.v = [];
    this.vt = [];
    this.vn = [];
  }

  async initialize(device, url) {
    await this.read_file(url);
    this.vertexCount = this.vertices.length / 5;

    const usage = GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST;

    const descriptor = {
      size: this.vertices.byteLength,
      usage: usage,
      mappedAtCreation: true,
    };

    this.buffer = device.createBuffer(descriptor);

    new Float32Array(this.buffer.getMappedRange()).set(this.vertices);

    this.buffer.unmap();

    this.bufferLayout = {
      arrayStride: 20,
      attributes: [
        {
          shaderLocation: 0,
          format: "float32x3",
          offset: 0,
        },
        {
          shaderLocation: 1,
          format: "float32x2",
          offset: 12,
        },
      ],
    };
  }

  async read_file(url) {
    let result = [];
    const response = await fetch(url);
    const blob = await response.blob();
    // const imageData = await createImageBitmap(blob);
    const file_contents = await blob.text();
    const lines = file_contents.split("\n");

    lines.forEach((line) => {
      if (line[0] === "v" && line[1] === " ") {
        this.read_vertex_line(line);
      } else if (line[0] === "v" && line[1] === "t") {
        this.read_textcoord_line(line);
      } else if (line[0] === "v" && line[1] === "n") {
        this.read_normal_line(line);
      } else if (line[0] === "f") {
        this.read_face_line(line, result);
      }
    });

    this.vertices = new Float32Array(result);
  }

  read_vertex_line(line) {
    // 顶点
    // Loading OBJ Models 13:21
    const components = line.split(" ");
    // ["v", 'x', 'y', 'z']
    const new_vertex = [
      Number(components[1]).valueOf(),
      Number(components[2]).valueOf(),
      Number(components[3]).valueOf(),
    ];
    this.v.push(new_vertex);
  }

  read_textcoord_line(line) {
    const components = line.split(" ");
    // ["vt", 'u', 'v']
    const new_textcoord = [
      Number(components[1]).valueOf(),
      Number(components[2]).valueOf(),
    ];
    this.vt.push(new_textcoord);
  }

  read_normal_line(line) {
    const components = line.split(" ");
    // ["vn", 'nx', 'ny', 'nz']
    const new_normal = [
      Number(components[1]).valueOf(),
      Number(components[2]).valueOf(),
      Number(components[3]).valueOf(),
    ];
    this.vn.push(new_normal);
  }

  read_face_line(line, result) {
    line = line.replace("\n", "");
    const vertex_descriptions = line.split(" ");
    // ["f", 'v1', 'v2', ...]
    const triangle_count = vertex_descriptions.length - 3;
    for (let i = 0; i < triangle_count; i++) {
      this.read_corner(vertex_descriptions[1], result);
      this.read_corner(vertex_descriptions[2 + i], result);
      this.read_corner(vertex_descriptions[3 + i], result);
    }
  }

  read_corner(vertex_descriptions, result) {
    const v_vt_vn = vertex_descriptions.split("/");
    const v = this.v[Number(v_vt_vn[0]).valueOf() - 1];
    const vt = this.vt[Number(v_vt_vn[1]).valueOf() - 1];
    result.push(v[0]);
    result.push(v[1]);
    result.push(v[2]);
    result.push(vt[0]);
    result.push(vt[1]);
  }
}

export default ObjMesh;
