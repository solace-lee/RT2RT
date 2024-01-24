import raytracer_kernel from "./shaders/raytracer_kernel.wgsl";
import screen_shader from "./shaders/screen_shader.wgsl";
// import { Scene } from './scene'
// import * as matrix from "gl-matrix";

// window.matrix = matrix

// const a = matrix.mat4.create()
// // matrix.mat4.set(a,
// //   0.9999251365661621, 0.005850422196090221, -0.01074779313057661, 7.479354381561279,
// //   -0.0060952771455049515, 0.9997193813323975, -0.0228921789675951, -190.7123260498047,
// //   0.010610847733914852, 0.02295597456395626, 0.9996801614761353, -1.3054922819137573,
// //   0, 0, 0, 1)
// matrix.mat4.set(a,
//   0.9999251365661621,    -0.0060952771455049515, 0.010610847733914852, 0,
//   0.005850422196090221,  0.9997193813323975,     0.02295597456395626,  0,
//   -0.01074779313057661,  -0.0228921789675951,    0.9996801614761353,   0,
//   7.479354381561279,     -190.7123260498047,     -1.3054922819137573,  1
// )
// const b = matrix.vec3.create()
// matrix.mat4.getRotation(b, a)
// const c = matrix.vec3.create()
// matrix.mat4.getScaling(c, a)
// const d = matrix.vec3.create()
// matrix.mat4.getTranslation(d, a)
// console.log(a, '旋转：', b, '缩放：', c, '平移：', d, '哈哈哈');




export class Renderer {
  constructor(canvas, scene) {
    this.canvas = canvas;
    this.scene = scene
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
          },
          {
            binding: 1,
            visibility: GPUShaderStage.COMPUTE,
            buffer: {
              type: 'uniform'
            }
          },
          {
            binding: 2,
            visibility: GPUShaderStage.COMPUTE,
            buffer: {
              type: 'read-only-storage',
              hasDynamicOffset: false
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
          },
          {
            binding: 1,
            resource: {
              buffer: this.sceneParameters,
            }
          },
          {
            binding: 2,
            resource: {
              buffer: this.sphereBuffer
            }
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

    const parameterBufferDescriptor = {
      size: 64,
      usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
    }
    this.sceneParameters = this.device.createBuffer(parameterBufferDescriptor)

    const sphereBufferDescriptor = {
      size: 32 * this.scene.spheres.length,
      usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_DST
    }

    this.sphereBuffer = this.device.createBuffer(
      sphereBufferDescriptor
    )
  }

  prepareScene() {

    const sceneData = {
      cameraPos: this.scene.camera.position,
      cameraForwards: this.scene.camera.forwards,
      cameraRight: this.scene.camera.right,
      cameraUp: this.scene.camera.up,
      sphereCount: this.scene.spheres.length,
    }
    this.device.queue.writeBuffer(
      this.sceneParameters, 0,
      new Float32Array(
        [
          sceneData.cameraPos[0],
          sceneData.cameraPos[1],
          sceneData.cameraPos[2],
          0.0,
          sceneData.cameraForwards[0],
          sceneData.cameraForwards[1],
          sceneData.cameraForwards[2],
          0.0,
          sceneData.cameraRight[0],
          sceneData.cameraRight[1],
          sceneData.cameraRight[2],
          0.0,
          sceneData.cameraUp[0],
          sceneData.cameraUp[1],
          sceneData.cameraUp[2],
          sceneData.sphereCount
        ]
      ), 0, 16
    )

    const sphereData = new Float32Array(8 * this.scene.spheres.length);
    for (let i = 0; i < this.scene.spheres.length; i++) {
      sphereData[8 * i] = this.scene.spheres[i].center[0];
      sphereData[8 * i + 1] = this.scene.spheres[i].center[1];
      sphereData[8 * i + 2] = this.scene.spheres[i].center[2];
      sphereData[8 * i + 3] = 0.0;
      sphereData[8 * i + 4] = this.scene.spheres[i].color[0];
      sphereData[8 * i + 5] = this.scene.spheres[i].color[1];
      sphereData[8 * i + 6] = this.scene.spheres[i].color[2];
      sphereData[8 * i + 7] = this.scene.spheres[i].radius;
    }

    this.device.queue.writeBuffer(this.sphereBuffer, 0, sphereData, 0, 8 * this.scene.spheres.length);
  }

  render = () => {
    this.prepareScene();

    const commandEncoder = this.device.createCommandEncoder();

    const ray_trace_pass = commandEncoder.beginComputePass();
    ray_trace_pass.setPipeline(this.ray_tracing_pipeline);
    ray_trace_pass.setBindGroup(0, this.ray_tracing_bind_group);
    ray_trace_pass.dispatchWorkgroups(
      Math.ceil(this.canvas.width / 8),
      Math.ceil(this.canvas.height / 8), 1);
    ray_trace_pass.end();

    const textureView = this.context.getCurrentTexture().createView();
    const renderpass = commandEncoder.beginRenderPass({
      colorAttachments: [{
        view: textureView,
        clearValue: { r: 0.5, g: 0.0, b: 0.25, a: 1.0 },
        loadOp: "clear",
        storeOp: "store"
      }]
    });

    renderpass.setPipeline(this.screen_pipeline);
    renderpass.setBindGroup(0, this.screen_bind_group);
    renderpass.draw(6, 1, 0, 0);

    renderpass.end();

    this.device.queue.submit([commandEncoder.finish()]);

    requestAnimationFrame(this.render);
  }
}
