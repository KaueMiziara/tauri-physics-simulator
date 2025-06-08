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
    let running = true;
    let lastTime = performance.now();

    const runPhysics = async () => {
      while (running) {
        const time = performance.now();
        const dt = Math.min((time - lastTime) / 1000, 0.1);
        lastTime = time;

        await invoke("physics_step", { dt });
        const position = await invoke<BallPosition>("get_ball_position");

        if (ballRef.current) {
          ballRef.current.position.set(position.x, position.y, position.z);
        }

        await new Promise((resolve) => requestAnimationFrame(resolve));
      }
    };

    invoke("reset_simulation");
    runPhysics();

    return () => {
      running = false;
    };
  }, [ballRef]);
};
