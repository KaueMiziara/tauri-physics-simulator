import { Canvas } from "@react-three/fiber";
import { Ball } from "./Ball";
import { Ground } from "./Ground";
import { usePhysics } from "../../hooks/usePhysics";
import { useRef } from "react";

export const Scene = () => {
  const ballRef = useRef(null);
  usePhysics(ballRef);

  return (
    <Canvas style={{ width: "100vw", height: "100vh" }}>
      <ambientLight intensity={0.5} />
      <pointLight position={[10, 10, 10]} intensity={1} />

      <Ball ref={ballRef} position={[0, 5, 0]} />
      <Ground />
    </Canvas>
  );
};
