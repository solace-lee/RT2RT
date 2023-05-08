import { Engine3D } from '@orillusion/core';
import { useEffect } from 'react';




function OrillusionPage(params) {

  useEffect(() => {
    init();
  }, [])

  async function init() {
    let canvas = document.getElementById('canvas')
    await Engine3D.init({
      canvasConfig: { canvas }
    });
  }
  return (
    <div>
      <div style={{ width: 500, height: 500 }}>
        <canvas id='canvas'></canvas>
      </div>
      <h1>OrillusionPage</h1>
    </div>
  )
}

export default OrillusionPage;