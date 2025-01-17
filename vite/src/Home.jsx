import reactLogo from './assets/react.svg'
// import { rt2rt, handleData } from '../pkg/rt2rt'
import { json } from './json'
import './App.css'
import data from './RT_fmt.json'
// import { ImageInfo } from '../../rust/pkg/rt2rt_bg'

function App() {

  function clickTest(params) {
    // console.log(rt2rt && rt2rt(obj));
    // console.log(rt2rt && rt2rt([45, 99, 4466, 99, 88]));
    console.time('耗时')
    const x = rt2rt(data)
    console.timeEnd('耗时')
    console.log(x);
  }

  return (
    <div className="App">
      <canvas className="glslCanvas" data-fragment-url="yourShader.frag" data-textures="yourInputImage.png" width="500" height="500"></canvas>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo" alt="Vite logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={clickTest}>
          执行
        </button>
        <p>
          Edit <code>src/App.jsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </div>
  )
}

export default App
