import shader from "./shaders.wgsl";
import TriangleMesh from "./triangle_mesh";
import { mat4 } from "gl-matrix";

export class Renderer {
  constructor(canvas) {
    this.canvas = canvas;
    this.t = 0.0;
  }

  async Initialize() {
    await this.setupDevice();

    this.createAssets();

    await this.makePipeline();

    this.render();
  }

  async setupDevice() {
    // 获取适配器
    this.adapter = await navigator.gpu?.requestAdapter();

    // 获取设备
    this.device = await this.adapter?.requestDevice();

    // 获取上下文
    this.context = this.canvas.getContext("webgpu");

    // 交换链格式
    this.format = "bgra8unorm";

    // 配置上下文
    this.context.configure({
      device: this.device, // 设备
      format: this.format, // 交换链格式
      alphaMode: "opaque", // 透明度模式
    });
  }

  async makePipeline() {
    this.uniformBuffer = this.device.createBuffer({
      size: 64 * 3,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
      // mappedAtCreation:
    });

    // 创建绑定组布局
    const bindGroupLayout = this.device.createBindGroupLayout({
      entries: [
        {
          // 条目
          binding: 0, // 绑定点
          visibility: GPUShaderStage.VERTEX, // 可见性
          buffer: {
            // 缓冲区
            type: "uniform", // 类型
          },
        },
      ],
    });

    // 创建绑定组,定义在着色器阶段的使用方式
    this.bindGroup = this.device.createBindGroup({
      layout: bindGroupLayout, // 绑定组布局
      entries: [
        {
          binding: 0, // 绑定点
          resource: {
            // 资源
            buffer: this.uniformBuffer,
          },
        },
      ],
    });

    // 创建管线布局
    const pipelineLayout = this.device.createPipelineLayout({
      bindGroupLayouts: [bindGroupLayout], // 绑定组布局
    });

    // 创建渲染管线
    this.pipeline = this.device.createRenderPipeline({
      // 创建渲染管线
      layout: pipelineLayout, // 管线布局
      vertex: {
        // 顶点
        module: this.device.createShaderModule({
          code: shader,
        }),
        entryPoint: "vs_main",
        buffers: [this.triangleMesh.bufferLayout],
      },
      fragment: {
        // 片段
        module: this.device.createShaderModule({
          code: shader,
        }),
        entryPoint: "fs_main",
        targets: [
          {
            format: this.format,
          },
        ],
      },
      primitive: {
        // 图元
        topology: "triangle-list", // 拓扑
      },
    });
  }

  // 创建静态资源
  createAssets() {
    this.triangleMesh = new TriangleMesh(this.device);
  }

  render = () => {
    this.t += 0.01;
    if (this.t > 2.0 * Math.PI) {
      this.t -= 2.0 * Math.PI;
    }

    const projection = mat4.create();
    mat4.perspective(projection, Math.PI / 4, 800 / 600, 0.1, 10);

    const view = mat4.create();
    mat4.lookAt(view, [-2, 0, 2], [0, 0, 0], [0, 0, 1]);

    const model = mat4.create();
    mat4.rotate(model, model, this.t, [0, 0, 1]);

    this.device.queue.writeBuffer(this.uniformBuffer, 0, model);
    this.device.queue.writeBuffer(this.uniformBuffer, 64, view);
    this.device.queue.writeBuffer(this.uniformBuffer, 128, projection);

    const commandEncoder = this.device.createCommandEncoder(); // 创建命令编码器

    const renderPass = commandEncoder.beginRenderPass({
      // 渲染通道描述符
      colorAttachments: [
        {
          view: this.context.getCurrentTexture().createView(), // 获取纹理视图
          loadValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, // 加载值
          storeOp: "store", // 存储操作
          loadOp: "clear", // 加载操作
        },
      ],
    }); // 开始渲染通道

    // `setPipeline`方法用于设置渲染通道中使用的管线。管线定义了渲染操作的状态和行为，包括着色器、颜色混合、剔除等¹。
    // `setBindGroup`方法用于设置渲染通道中使用的资源绑定组。资源绑定组定义了一组要绑定在一起的资源以及这些资源在着色器阶段中的使用方式¹。

    renderPass.setPipeline(this.pipeline); // 设置管线
    renderPass.setBindGroup(0, this.bindGroup); // 设置绑定组
    renderPass.setVertexBuffer(0, this.triangleMesh.buffer);
    // renderPass.draw(3, 1, 0); // 绘制
    renderPass.draw(3); // 绘制
    renderPass.end(); // 结束
    this.device.queue.submit([commandEncoder.finish()]); // 提交

    requestAnimationFrame(this.render)
  }
}
