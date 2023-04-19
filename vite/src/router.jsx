import { createBrowserRouter } from "react-router-dom";
import Home from "./Home";
import Wgpu from "./wgpu";

const router = [
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/webgpu",
    element: <Wgpu />,
  },
];

export default router;
