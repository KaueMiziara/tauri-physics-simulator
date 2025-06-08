import { JSX } from "react";

export const Ground = (props: JSX.IntrinsicElements["mesh"]) => {
  return (
    <mesh
      {...props}
      rotation={[-Math.PI / 2, 0, 0]}
      position={[0, 0, 0]}
      receiveShadow
    >
      <planeGeometry args={[10, 10]} />
      <meshStandardMaterial color="#f0f0f0" />
    </mesh>
  );
};
