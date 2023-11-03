import raytracer_kernel from "./shaders/raytracer_kernel.wgsl";
import screen_shader from "./shaders/screen_shader.wgsl";
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
    const ray_tracing_bind_group_layout = this.device.createBindGroupLayout(
      {
        entries: [
          {
            binding: 0,
            visibility: GPUShaderStage.COMPUTE,
            storageTexture: {
              access: 'write-only',
              format: 'rgba8unorm',
              viewDimension: '2d'
            }
          }
        ]
      }
    )

    this.ray_tracing_bind_group = this.device.createBindGroup(
      {
        layout: ray_tracing_bind_group_layout,
        entries: [
          {
            binding: 0,
            resource: this.color_buffer_view
          }
        ]
      }
    )


    const ray_tracing_pipeline_layout = this.device.createPipelineLayout(
      {
        bindGroupLayouts: [ray_tracing_bind_group_layout]
      }
    )

    this.ray_tracing_pipeline = this.device.createComputePipeline(
      {
        layout: ray_tracing_pipeline_layout,

        compute: {
          module: this.device.createShaderModule(
            {
              code: raytracer_kernel,
            }
          ),
          entryPoint: 'main',
        }
      }
    )

    // screen pipeline
    const screen_bind_group_layout = this.device.createBindGroupLayout(
      {
        entries: [
          {
            binding: 0,
            visibility: GPUShaderStage.FRAGMENT,
            sampler: {}
          },
          {
            binding: 1,
            visibility: GPUShaderStage.FRAGMENT,
            texture: {}
          }
        ]
      }
    )

    this.screen_bind_group = this.device.createBindGroup(
      {
        layout: screen_bind_group_layout,
        entries: [
          {
            binding: 0,
            resource: this.sampler
          },
          {
            binding: 1,
            resource: this.color_buffer_view
          }
        ]
      }
    )

    const screen_pipeline_layout = this.device.createPipelineLayout(
      {
        bindGroupLayouts: [screen_bind_group_layout]
      }
    )

    this.screen_pipeline = this.device.createRenderPipeline(
      {
        layout: screen_pipeline_layout,

        vertex: {
          module: this.device.createShaderModule(
            {
              code: screen_shader,
            }
          ),
          entryPoint: 'vert_main',
        },
        fragment: {
          module: this.device.createShaderModule(
            {
              code: screen_shader,
            }
          ),
          entryPoint: 'frag_main',
          targets: [
            {
              format: 'bgra8unorm'
            }
          ]
        },
        primitive: {
          topology: 'triangle-list'
        }
      }
    )
  }



  // 创建静态资源
  createAssets() {
    this.color_buffer = this.device.createTexture(
      {
        size: {
          width: this.canvas.width,
          height: this.canvas.height
        },
        format: 'rgba8unorm',
        usage: GPUTextureUsage.COPY_DST | GPUTextureUsage.STORAGE_BINDING | GPUTextureUsage.TEXTURE_BINDING
      }
    )

    this.color_buffer_view = this.color_buffer.createView();

    const samplerDescriptor = {
      addressModeU: "repeat",
      addressModeV: "repeat",
      magFilter: "linear",
      minFilter: "nearest",
      mipmapFilter: "nearest",
      maxAnisotropy: 1
    }

    this.sampler = this.device.createSampler(samplerDescriptor);
  }

  render = () => {
    const commandEncoder = this.device.createCommandEncoder(); // 创建命令编码器

    const ray_trace_pass = commandEncoder.beginComputePass()
    ray_trace_pass.setPipeline(this.ray_tracing_pipeline)
    ray_trace_pass.setBindGroup(0, this.ray_tracing_bind_group)
    ray_trace_pass.dispatchWorkgroups(this.canvas.width, this.canvas.height, 1)
    ray_trace_pass.end()


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

    renderPass.setPipeline(this.screen_pipeline); // 设置管线
    renderPass.setBindGroup(0, this.screen_bind_group); // 设置绑定组
    // renderPass.setVertexBuffer(0, this.triangleMesh.buffer);
    renderPass.draw(6, 1, 0, 0); // 绘制
    // renderPass.draw(3); // 绘制
    renderPass.end(); // 结束
    this.device.queue.submit([commandEncoder.finish()]); // 提交

    requestAnimationFrame(this.render)
  }
}
