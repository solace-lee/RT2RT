import { createBrowserRouter } from "react-router-dom";
import Home from "./Home";
import Webgpu from "./Webgpu";

const router = [
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/webgpu",
    element: <Webgpu />,
  },
];

export default router;
