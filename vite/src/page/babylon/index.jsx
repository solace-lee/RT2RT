// import { Engine3D } from '@orillusion/core';
import { useEffect } from 'react';
import {
  WebGPUEngine,
  Engine,
  Scene,
  FreeCamera,
  Vector3,
  HemisphericLight,
  MeshBuilder,
} from '@babylonjs/core';

import "@babylonjs/core/Debug/debugLayer";
import "@babylonjs/inspector";


function BabylonPage(params) {

  useEffect(() => {
    init();
  }, [])

  async function createEngine() {
    const webGPUSupported = await WebGPUEngine.IsSupportedAsync;
    let canvas = document.getElementById('canvas')

    if (webGPUSupported) {
      const engine = new WebGPUEngine(canvas);
      await engine.initAsync();
      return engine;
    }
    return new Engine(canvas, true);
  }

  async function init() {
    // Load the 3D engine
    var engine = await createEngine();
    // CreateScene function that creates and return the scene
    var createScene = function () {
      // 创建基本的BJS场景对象
      var scene = new Scene(engine);
      // 创建一个FreeCamera，并将其位置设置为｛x:0，y:5，z:-10｝
      var camera = new FreeCamera('camera1', new Vector3(0, 5, -10), scene);
      // 将摄影机定位到场景原点
      camera.setTarget(Vector3.Zero());
      // 将相机连接到画布
      camera.attachControl(canvas, true);
      // 创建一个基本灯光，目标为0，1，0，意思是指向天空
      var light = new HemisphericLight('light1', new Vector3(0, 1, 0), scene);
      // 默认强度为1。让我们把灯调暗一点
      light.intensity = 0.7;
      // 使用SphereBuilder创建内置的“球体”形状
      var sphere = MeshBuilder.CreateSphere('sphere1', { segments: 16, diameter: 2 }, scene);
      // 将球体向上移动其高度的1/2
      sphere.position.y = 1;
      // 创建一个内置的“地面”形状；
      var ground = MeshBuilder.CreateGround("ground1", { width: 6, height: 6 }, scene);
      // 返回创建的场景
      return scene;
    }
    // call the createScene function
    var scene = createScene();
    // scene.debugLayer.show();
    // run the render loop
    engine.runRenderLoop(function () {
      scene.render();
    });
    // the canvas/window resize event handler
    window.addEventListener('resize', function () {
      engine.resize();
    });

  }

  return (
    <div>
      <div style={{ width: '100vw', height: '50vh' }}>
        <canvas width='800' height='500' id='canvas'></canvas>
      </div>
      <h1>BabylonPage</h1>
    </div>
  )
}

export default BabylonPage;