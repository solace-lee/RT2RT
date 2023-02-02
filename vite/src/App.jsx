import reactLogo from './assets/react.svg'
import { rt2rt, handleData } from '../pkg/rt2rt'
import { json } from './json'
import './App.css'
// import { ImageInfo } from '../../rust/pkg/rt2rt_bg'

function App() {

  function clickTest(params) {
    console.time('start')
    console.log(handleData && handleData(json))
    console.timeEnd('start')
    // const obj = {
    //   column: 12,
    //   row: 14,
    //   lay_num: 15,
    //   row_pixel_spacing: 15.0,
    //   column_pixel_spacing: 16.0,
    //   thickness: 3.0,
    //   zm_obj_array: {
    //     data: [4]
    //   },
    //   zm_data: [],
    //   zm_data_obj: []
    // }
    // console.log(getObj && getObj(obj));
    // console.log(rt2rt && rt2rt(obj));
    // console.log(rt2rt && rt2rt([45, 99, 4466, 99, 88]));
  }

  return (
    <div className="App">
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
