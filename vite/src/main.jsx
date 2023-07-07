import React from 'react'
import ReactDOM from 'react-dom/client'
import './index.css'
import { createBrowserRouter, RouterProvider, Route, Outlet } from 'react-router-dom'
import router from './router'

// import { registerSW } from 'virtual:pwa-register'
// registerSW({
//   // 每小时检查一次
//   onRegistered: r => r && setInterval(async () => await r.update(), 3600000),
//   // 注册失败则报错到 console
//   onRegisterError: error => console.error(error)
// })

if ('serviceWorker' in navigator) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('/sw.js')
      .then(registration => {
        console.log('Service Worker registered: ', registration);
      })
      .catch(error => {
        console.log('Service Worker registration failed: ', error);
      });
  });
}

const Router = createBrowserRouter(router)

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <RouterProvider router={Router} />
  </React.StrictMode>)
