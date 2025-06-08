import { Canvas } from "@react-three/fiber";
import { Ball } from "./Ball";
import { Ground } from "./Ground";
import { usePhysics } from "../../hooks/usePhysics";
import { useRef } from "react";
import { OrbitControls } from "@react-three/drei";

export const Scene = () => {
  const ballRef = useRef(null);
  usePhysics(ballRef);

  return (
    <Canvas
      style={{ width: "100vw", height: "100vh" }}
      camera={{ position: [0, 2, 5], fov: 75 }}
      shadows
    >
      <ambientLight intensity={0.5} />
      <pointLight position={[1, 2, 1]} intensity={2} castShadow />

      <OrbitControls
        makeDefault
        enablePan={true}
        enableZoom={true}
        enableRotate={true}
      />

      <Ball ref={ballRef} position={[0, 5, 0]} />
      <Ground />
    </Canvas>
  );
};
