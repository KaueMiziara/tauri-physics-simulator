import { Plane } from "@react-three/drei";

export const Ground = () => (
  <Plane
    args={[10, 10]}
    rotation={[-Math.PI / 2, 0, 0]}
    position={[0, -1, 0]}
    receiveShadow
  >
    <meshStandardMaterial color="#f0f0f0" />
  </Plane>
);
