import { Material } from "./material";
import shader from "./shaders/shaders.wgsl";
import TriangleMesh from "./triangle_mesh";
import QuadMesh from "./quad_mesh";
import { mat4 } from "gl-matrix";
import { object_types } from "../model/definitions";

// window.mat4 = mat4;
// console.log(mat4);
export class Renderer {
  constructor(canvas) {
    this.canvas = canvas;
  }

  async Initialize() {
    await this.setupDevice();

    await this.makeBindGroupLayouts();

    await this.createAssets();

    await this.makeDepthBufferResources();

    await this.makePipeline();

    await this.makeBindGroup();
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

  // 处理深度问题
  async makeDepthBufferResources() {
    this.depthStencilState = {
      format: "depth24plus-stencil8",
      depthWriteEnabled: true,
      depthCompare: "less-equal",
    };
    const size = {
      width: this.canvas.width,
      height: this.canvas.height,
      depthOrArrayLayers: 1,
    };

    const depthBufferDescriptor = {
      size: size,
      format: "depth24plus-stencil8",
      usage: GPUTextureUsage.RENDER_ATTACHMENT,
    };

    this.depthStencilBuffer = this.device.createTexture(depthBufferDescriptor);
    const viewDescriptor = {
      format: "depth24plus-stencil8",
      dimension: "2d",
      aspect: "all",
    };
    this.depthStencilView = this.depthStencilBuffer.createView(viewDescriptor);

    this.depthStencilAttachment = {
      view: this.depthStencilView,
      depthClearValue: 1.0,
      depthLoadOp: "clear",
      depthStoreOp: "store",

      stencilLoadOp: "clear",
      stencilStoreOp: "discard",
    };
  }

  async makeBindGroupLayouts() {
    this.frameGroupLayout = this.device.createBindGroupLayout({
      entries: [
        {
          // 条目
          binding: 0, // 绑定点
          visibility: GPUShaderStage.VERTEX, // 可见性
          buffer: {},
        },
        {
          binding: 1,
          visibility: GPUShaderStage.VERTEX,
          buffer: {
            type: "read-only-storage",
            hasDynamicOffset: false,
          },
        },
      ],
    });
    this.materialGroupLayout = this.device.createBindGroupLayout({
      entries: [
        {
          // 条目
          binding: 0, // 绑定点
          visibility: GPUShaderStage.FRAGMENT, // 可见性
          texture: {},
        },
        {
          binding: 1,
          visibility: GPUShaderStage.FRAGMENT,
          sampler: {}
        },
      ],
    });
  }

  async makePipeline() {
    // 创建管线布局
    const pipelineLayout = this.device.createPipelineLayout({
      bindGroupLayouts: [this.frameGroupLayout, this.materialGroupLayout], // 绑定组布局
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
      depthStencil: this.depthStencilState,
    });
  }

  // 创建静态资源
  async createAssets() {
    this.triangleMesh = new TriangleMesh(this.device);
    this.triangleMaterial = new Material();
    this.quadMesh = new QuadMesh(this.device);
    this.quadMaterial = new Material();

    const modelBufferDescriptor = {
      size: 64 * 1024,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST,
    };
    this.objectBuffer = this.device.createBuffer(modelBufferDescriptor);

    this.uniformBuffer = this.device.createBuffer({
      size: 64 * 2,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
      // mappedAtCreation:
    });

    await this.triangleMaterial.initialize(this.device, "/bg.jpeg", this.materialGroupLayout);
    await this.quadMaterial.initialize(this.device, "/bj1.jpg", this.materialGroupLayout);
  }

  async makeBindGroup() {
    this.frameBindGroup = this.device.createBindGroup({
      layout: this.frameGroupLayout, // 绑定组布局
      entries: [
        {
          binding: 0, // 绑定点
          resource: {
            buffer: this.uniformBuffer,
          },
        },
        {
          binding: 1, // 绑定点
          resource: {
            buffer: this.objectBuffer,
          },
        },
      ],
    })
  }

  async render(renderables) {
    const projection = mat4.create();
    mat4.perspective(projection, Math.PI / 4, 800 / 600, 0.1, 10);

    const view = renderables.view_transform;

    this.device.queue.writeBuffer(
      this.objectBuffer,
      0,
      renderables.model_transform,
      0,
      renderables.model_transform.length
    );
    this.device.queue.writeBuffer(this.uniformBuffer, 0, view);
    this.device.queue.writeBuffer(this.uniformBuffer, 64, projection);

    const commandEncoder = this.device.createCommandEncoder(); // 创建命令编码器

    const renderPass = commandEncoder.beginRenderPass({
      // 渲染通道描述符
      colorAttachments: [
        {
          view: this.context.getCurrentTexture().createView(), // 获取纹理视图
          loadValue: { r: 0.5, g: 0.0, b: 0.25, a: 1.0 }, // 加载值
          storeOp: "store", // 存储操作
          loadOp: "clear", // 加载操作
        },
      ],
      depthStencilAttachment: this.depthStencilAttachment,
    }); // 开始渲染通道

    // `setPipeline`方法用于设置渲染通道中使用的管线。管线定义了渲染操作的状态和行为，包括着色器、颜色混合、剔除等¹。
    // `setBindGroup`方法用于设置渲染通道中使用的资源绑定组。资源绑定组定义了一组要绑定在一起的资源以及这些资源在着色器阶段中的使用方式¹。

    renderPass.setPipeline(this.pipeline); // 设置管线
    renderPass.setBindGroup(0, this.frameBindGroup);

    let object_drawn = 0;

    // Triangles
    renderPass.setVertexBuffer(0, this.triangleMesh.buffer);
    renderPass.setBindGroup(1, this.triangleMaterial.bindGroup); // 设置绑定组
    renderPass.draw(
      3,
      renderables.object_counts[object_types.TRIANGLE],
      0,
      object_drawn
    ); // 绘制
    object_drawn += renderables.object_counts[object_types.TRIANGLE];

    // Quads
    renderPass.setVertexBuffer(0, this.quadMesh.buffer);
    renderPass.setBindGroup(1, this.quadMaterial.bindGroup); // 设置绑定组
    renderPass.draw(
      6,
      renderables.object_counts[object_types.QUAD],
      0,
      object_drawn
    ); // 绘制
    object_drawn += renderables.object_counts[object_types.QUAD];

    // renderPass.draw(3, 1, 0); // 绘制

    renderPass.end(); // 结束
    this.device.queue.submit([commandEncoder.finish()]); // 提交
  }
}
