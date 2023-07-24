import React, { useEffect, useState } from 'react'
import { App } from './control/app';

function Wgpu() {
  const gpuIsSupported = navigator.gpu ? true : false;
  const [move, setMove] = useState({})
  const [key, setKey] = useState('')
  const [app, setApp] = useState(null)

  useEffect(() => {
    if (navigator.gpu) {
      console.log('WebGPU is supported');
      Initialize()
    } else {
      console.log('WebGPU is not supported');
    }
  }, [])

  useEffect(() => {
    document.addEventListener('keydown', keyDown)
    document.addEventListener('keyup', keyUp)
    return () => {
      document.removeEventListener('keydown', keyDown)
      document.removeEventListener('keyup', keyUp)
    }
  })

  // 初始化
  async function Initialize() {
    const canvas = document.getElementById('gfx-main'); // 获取canvas
    canvas.onclick = () => {
      canvas.requestPointerLock()
    }
    const app = new App(canvas)
    await app.inititalize()
    app.run()
    setApp(app)
  }

  function mouseMoveHandle(event) {
    setMove({ x: event.clientX, y: event.clientY })
    if (app) {
      app.mouse_move(event.movementX, event.movementY)
    }
  }
  function keyDown(event) {
    setKey(event.code)

    if (app) {
      if (event.code === 'KeyW') {
        app.forwards_amount = 0.02
      }
      if (event.code === 'KeyS') {
        app.forwards_amount = -0.02
      }
      if (event.code === 'KeyA') {
        app.right_amount = -0.02
      }
      if (event.code === 'KeyD') {
        app.right_amount = 0.02
      }
    }
  }

  function keyUp(event) {
    setKey(event.code)

    if (app) {
      if (event.code === 'KeyW') {
        app.forwards_amount = 0
      }
      if (event.code === 'KeyS') {
        app.forwards_amount = 0
      }
      if (event.code === 'KeyA') {
        app.right_amount = 0
      }
      if (event.code === 'KeyD') {
        app.right_amount = 0
      }
    }
  }


  return (
    <div className="Wgpu">
      {/* {gpuIsSupported && <h2>WGPU已被支持</h2>} */}
      <div
        style={{ border: '1px solid red' }}
        onKeyDown={keyDown}
        onKeyUp={keyUp}
      >
        <canvas onMouseMove={mouseMoveHandle} id='gfx-main' width='800' height='600'></canvas>
      </div>
      <h2>Current Key: {key}</h2>
      <hr />
      <h2>Mouse X: {move.x}</h2>
      <hr />
      <h2>Mouse Y: {move.y}</h2>
      <hr />
    </div>
  )
}

export default Wgpu
