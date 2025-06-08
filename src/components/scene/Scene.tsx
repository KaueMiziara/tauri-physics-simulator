import { OrbitControls, PerspectiveCamera } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import { Ball } from "./Ball";
import { Ground } from "./Ground";

export const Scene = () => (
  <Canvas style={{ width: "100vw", height: "100vh" }} shadows>
    <PerspectiveCamera makeDefault position={[0, 2, 5]} />
    <OrbitControls target={[0, 0, 0]} />

    <ambientLight intensity={0.5} />
    <pointLight position={[10, 10, 10]} intensity={1} castShadow />

    <Ball position={[0, 2, 0]} />
    <Ground />
  </Canvas>
);
