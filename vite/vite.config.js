import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
// import vitePluginString from 'vite-plugin-string'
import glsl from 'vite-plugin-glsl'
import { VitePWA } from 'vite-plugin-pwa'


// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait(), glsl({defaultExtension: 'wgsl'}), VitePWA()],
})
