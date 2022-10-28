// import * as wasm from "./rt2rt_bg.wasm";
// export * from "./rt2rt_bg.js";


const fs = require('fs');
const path = require('path');

// let src = new Uint8Array(fs.readFileSync(path.join(__dirname, "./rt2rt_bg.wasm")))
// const env = {
//   memoryBase: 0,
//   tableBase: 0,
//   memory: new WebAssembly.Memory({
//     initial: 256
//   }),
//   table: new WebAssembly.Table({
//     initial: 2,
//     element: 'anyfunc'
//   }),
//   abort: () => {
//     throw 'abort'
//   }
// }

// WebAssembly.instantiate(src, {
//     env
//   })
//   .then(result => {
//     console.log(result.instance.exports.fib(4), "hhh");
//   }).catch(err => {
//     console.log(err);
//   })

fs.readFile(path.join(__dirname, "./pkg/rt2rt_bg.wasm"), (err, data) => {
  WebAssembly.instantiate(data).then((module) => {
    console.log(module.instance.exports.fib(6), '4444');
  }).catch(err => {
    console.log(err);
  })
})

// wasm-pack build