import React, { useEffect } from 'react'
import { Renderer } from './renderer'
import { Scene } from "./scene";

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
    const scene = new Scene();
    const renderer = await new Renderer(canvas, scene).Initialize()
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
