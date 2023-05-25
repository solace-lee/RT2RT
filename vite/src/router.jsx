import { createBrowserRouter } from "react-router-dom";
import Home from "./Home";
import Wgpu from "./page/wgpu";
import OrillusionPage from "./page/orillusion";
import BabylonPage from "./page/babylon";
import ThreePage from "./page/three"

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
  {
    path: "/babylon",
    element: <BabylonPage />,
  },
  {
    path: "/three",
    element: <ThreePage />,
  },
];

export default router;
