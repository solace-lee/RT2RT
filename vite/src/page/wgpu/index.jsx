import React, { useEffect } from 'react'
import shader from './shaders.wgsl'
import TriangleMesh from './triangle_mesh'

function Wgpu() {
  const gpuIsSupported = navigator.gpu ? true : false;

  useEffect(() => {
    if (navigator.gpu) {
      console.log('WebGPU is supported');
      Initialize()
    } else {
      console.log('WebGPU is not supported');
    }
  }, [])

  // 初始化
  async function Initialize() {
    const canvas = document.getElementById('gfx-main'); // 获取canvas
    const adapter = await navigator.gpu?.requestAdapter(); // 获取适配器
    const device = await adapter?.requestDevice(); // 获取设备
    const context = canvas.getContext('webgpu'); // 获取上下文
    const swapChainFormat = 'bgra8unorm'; // 交换链格式
    context.configure({ // 配置上下文
      device, // 设备
      format: swapChainFormat, // 交换链格式
      alphaMode: 'opaque', // 透明度模式
    });
    console.log(device);

    const triangleMesh = new TriangleMesh(device)

    const bindGroupLayout = device.createBindGroupLayout({ // 创建绑定组布局
      entries: [{ // 条目
        binding: 0, // 绑定点
        visibility: GPUShaderStage.VERTEX, // 可见性
        buffer: { // 缓冲区
          type: 'uniform', // 类型
        },
      }],
      // entries: []
    })

    const bindGroup = device.createBindGroup({ // 创建绑定组,定义在着色器阶段的使用方式
      layout: bindGroupLayout, // 绑定组布局
      // entries: []
      entries: [{
        binding: 0, // 绑定点
        resource: { // 资源
          buffer: device.createBuffer({ // 创建缓冲区
            size: 4 * 4, // 大小
            usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST, // 用途
          }),
        },
      }],
    });

    const pipelineLayout = device.createPipelineLayout({ // 创建管线布局
      bindGroupLayouts: [bindGroupLayout], // 绑定组布局
    });

    const pipeline = device.createRenderPipeline({ // 创建渲染管线
      layout: pipelineLayout, // 管线布局
      vertex: { // 顶点
        module: device.createShaderModule({
          code: shader
        }),
        entryPoint: 'vs_main',
        buffers: [triangleMesh.bufferLayout]
      },
      fragment: { // 片段
        module: device.createShaderModule({
          code: shader
        }),
        entryPoint: 'fs_main',
        targets: [{
          format: swapChainFormat,
        }],
      },
      primitive: { // 图元
        topology: 'triangle-list', // 拓扑
      },
    });

    const commandEncoder = device.createCommandEncoder(); // 创建命令编码器

    const renderPass = commandEncoder.beginRenderPass({ // 渲染通道描述符
      colorAttachments: [{
        view: context.getCurrentTexture().createView(), // 获取纹理视图
        loadValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, // 加载值
        storeOp: 'store', // 存储操作
        loadOp: 'clear', // 加载操作
      }]
    }); // 开始渲染通道

    // `setPipeline`方法用于设置渲染通道中使用的管线。管线定义了渲染操作的状态和行为，包括着色器、颜色混合、剔除等¹。
    // `setBindGroup`方法用于设置渲染通道中使用的资源绑定组。资源绑定组定义了一组要绑定在一起的资源以及这些资源在着色器阶段中的使用方式¹。

    renderPass.setPipeline(pipeline); // 设置管线
    renderPass.setBindGroup(0, bindGroup); // 设置绑定组
    renderPass.setVertexBuffer(0, triangleMesh.buffer)
    // renderPass.draw(3, 1, 0); // 绘制
    renderPass.draw(3); // 绘制
    renderPass.end(); // 结束
    device.queue.submit([commandEncoder.finish()]); // 提交
  }


  return (
    <div className="Wgpu">
      {/* {gpuIsSupported && <h2>WGPU已被支持</h2>} */}
      <div style={{ border: '1px solid red' }}>
        <canvas id='gfx-main' width='800' height='600'></canvas>
      </div>
    </div>
  )
}

export default Wgpu
