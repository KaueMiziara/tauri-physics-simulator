import { invoke } from "@tauri-apps/api/core";
import { useEffect } from "react";

type BallPosition = {
  x: number;
  y: number;
  z: number;
};

type PositionableObject = {
  position: {
    x: number;
    y: number;
    z: number;
    set: (x: number, y: number, z: number) => void;
  };
};

export const usePhysics = (
  ballRef: React.RefObject<PositionableObject | null>,
) => {
  useEffect(() => {
    let animationFrameId: number;
    let lastTime = performance.now();

    const animate = async (time: number) => {
      const dt = (time - lastTime) / 1000;
      lastTime = time;

      await invoke("physics_step", { dt });

      const postition = await invoke<BallPosition>("get_ball_position");

      if (ballRef.current) {
        ballRef.current.position.set(postition.x, postition.y, postition.z);
      }

      animationFrameId = requestAnimationFrame(animate);
    };
    animationFrameId = requestAnimationFrame(animate);

    return () => {
      cancelAnimationFrame(animationFrameId);
    };
  }, [ballRef]);
};
