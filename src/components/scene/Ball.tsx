import { JSX } from "react";

export const Ball = (props: JSX.IntrinsicElements["mesh"]) => (
  <mesh {...props} castShadow>
    <sphereGeometry args={[0.5, 32, 32]} />
    <meshStandardMaterial color="hotpink" />
  </mesh>
);
