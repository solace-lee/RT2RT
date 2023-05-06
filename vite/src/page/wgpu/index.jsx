import React, { useEffect } from 'react'
import shader from './shaders.wgsl'

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

  async function Initialize() {
    const canvas = document.getElementById('gfx-main');
    const adapter = await navigator.gpu?.requestAdapter();
    const device = await adapter?.requestDevice();
    const context = canvas.getContext('webgpu');
    const swapChainFormat = 'bgra8unorm';
    context.configure({
      device,
      format: swapChainFormat
    });
    console.log(device);
    const pipeline = device.createRenderPipeline({
      layout: device.createPipelineLayout({ bindGroupLayouts: [] }),
      vertex: {
        module: device.createShaderModule({
          code: shader
        }),
        entryPoint: 'vs_main',
      },
      fragment: {
        module: device.createShaderModule({
          code: shader
        }),
        entryPoint: 'fs_main',
        targets: [{
          format: swapChainFormat,
        }],
      },
      primitive: {
        topology: 'triangle-list',
      },
    });
    const commandEncoder = device.createCommandEncoder();
    const textureView = context.getCurrentTexture().createView();
    const renderPassDescriptor = {
      colorAttachments: [{
        view: textureView,
        loadValue: { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
        storeOp: 'store',
        loadOp: 'clear',
      }]
    };
    const renderPass = commandEncoder.beginRenderPass(renderPassDescriptor);
    renderPass.setPipeline(pipeline);
    renderPass.draw(3, 1, 0, 0);
    renderPass.end();
    device.queue.submit([commandEncoder.finish()]);
  }


  return (
    <div className="Wgpu">
      {gpuIsSupported && <h2>WGPU已被支持</h2>}
      <div style={{ border: '1px solid red'}}>
        <canvas id='gfx-main' width='800' height='600'></canvas>
      </div>
    </div>
  )
}

export default Wgpu
