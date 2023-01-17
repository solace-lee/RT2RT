// import * as wasm from "../pkg/rt2rt.js";
// import { fib } from "../pkg/rt2rtjs";
import fs from 'fs'
import path from 'path'
import { fileURLToPath } from 'url'

// 调用wasm处理

// const fs = require('fs');
// const path = require('path');

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


const __filenameNew = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filenameNew)

fs.readFile(path.join(__dirname, "../pkg/rt2rt_bg.wasm"), (err, data) => {
  WebAssembly.instantiate(data).then((module) => {
    console.time('wasm一百万次耗时');
    console.log(module.instance.exports.fib(45), '4444');
    console.timeEnd('wasm一百万次耗时');
  }).catch(err => {
    console.log(err);
  })
})

// wasm-pack build