import { beforeEach, describe, expect, it } from "vitest";
import { useSimStore } from "./store";
import { MISSIONS } from "./missions/missions";

describe("mission progress initialization", () => {
  beforeEach(() => {
    useSimStore.setState({ activeMission: null, missionProgress: null });
  });

  it("matches objectiveStatus length to the selected mission", () => {
    for (const mission of MISSIONS) {
      useSimStore.getState().setActiveMission(mission.id);
      const progress = useSimStore.getState().missionProgress;
      expect(progress).not.toBeNull();
      expect(progress?.objectiveStatus.length).toBe(mission.objectives.length);
      expect(progress?.objectiveStatus.every((status) => status === false)).toBe(true);
    }
  });

  it("clears mission progress when mission is aborted", () => {
    useSimStore.getState().setActiveMission("reach_mars");
    expect(useSimStore.getState().missionProgress).not.toBeNull();

    useSimStore.getState().setActiveMission(null);
    expect(useSimStore.getState().missionProgress).toBeNull();
  });
});
