import { describe, expect, it } from "vitest";
import { checkObjective } from "./MissionDefinition";
import type { SimulationFrame } from "../types";

const baseFrame: SimulationFrame = {
  tick: 100,
  paused: false,
  speed_multiplier: 1,
  energy: { kinetic: 0, potential: 0, total: 0 },
  bodies: [
    {
      id: 1,
      name: "Sun",
      body_type: "star",
      mass: 1000,
      radius: 30,
      color: "#ffff00",
      is_fixed: true,
      position: { x: 0, y: 0, z: 0 },
      velocity: { x: 0, y: 0, z: 0 },
      acceleration: { x: 0, y: 0, z: 0 },
      thrust: { x: 0, y: 0, z: 0 },
      fuel: 0,
      max_fuel: 0,
      trail: [],
    },
    {
      id: 2,
      name: "Jupiter",
      body_type: "planet",
      mass: 20,
      radius: 10,
      color: "#ffaa00",
      is_fixed: false,
      position: { x: 500, y: 0, z: 0 },
      velocity: { x: 0, y: 0, z: 0 },
      acceleration: { x: 0, y: 0, z: 0 },
      thrust: { x: 0, y: 0, z: 0 },
      fuel: 0,
      max_fuel: 0,
      trail: [],
    },
    {
      id: 3,
      name: "Ship",
      body_type: "spacecraft",
      mass: 1,
      radius: 3,
      color: "#00aaff",
      is_fixed: false,
      position: { x: 510, y: 0, z: 0 },
      velocity: { x: 30, y: 0, z: 0 },
      acceleration: { x: 0, y: 0, z: 0 },
      thrust: { x: 0, y: 0, z: 0 },
      fuel: 100,
      max_fuel: 100,
      trail: [],
    },
  ],
};

describe("checkObjective", () => {
  it("passes reach_orbit objective near target radius from central body", () => {
    const result = checkObjective(
      {
        type: "reach_orbit",
        description: "Reach orbit",
        targetRadius: 510,
        threshold: 20,
      },
      baseFrame,
      3,
      10,
    );

    expect(result).toBe(true);
  });

  it("passes reach_body objective when spacecraft is inside threshold", () => {
    const result = checkObjective(
      {
        type: "reach_body",
        description: "Approach Jupiter",
        targetBodyName: "Jupiter",
        threshold: 20,
      },
      baseFrame,
      3,
      10,
    );

    expect(result).toBe(true);
  });

  it("passes achieve_speed objective when spacecraft speed meets minSpeed", () => {
    const result = checkObjective(
      { type: "achieve_speed", description: "Go fast", minSpeed: 25 },
      baseFrame,
      3,
      10,
    );

    expect(result).toBe(true);
  });

  it("passes survive_time objective when elapsed ticks meet requirement", () => {
    const result = checkObjective(
      { type: "survive_time", description: "Stay alive", requiredTicks: 50 },
      baseFrame,
      3,
      50,
    );

    expect(result).toBe(true);
  });

  it("returns false when spacecraft is not bound to the mission", () => {
    const result = checkObjective(
      { type: "achieve_speed", description: "Go fast", minSpeed: 25 },
      baseFrame,
      null,
      10,
    );

    expect(result).toBe(false);
  });
});
