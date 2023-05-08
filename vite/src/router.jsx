import { createBrowserRouter } from "react-router-dom";
import Home from "./Home";
import Wgpu from "./page/wgpu";
import OrillusionPage from "./page/orillusion";

const router = [
  {
    path: "/",
    element: <Home />,
  },
  {
    path: "/webgpu",
    element: <Wgpu />,
  },
  {
    path: "/orillusion",
    element: <OrillusionPage />,
  },
];

export default router;
