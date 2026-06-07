import { describe, expect, it } from "vitest";
import { decodeSharedState, encodeSharedState } from "../src/utils/sharing";

describe("sharing helpers", () => {
  it("round-trips exported state payloads", async () => {
    const json = JSON.stringify({
      bodies: [
        { id: 1, name: "Sun", body_type: "star" },
        { id: 2, name: "Earth", body_type: "planet" },
      ],
      paused: false,
      tick: 42,
    });

    const encoded = await encodeSharedState(json);
    const decoded = await decodeSharedState(encoded);

    expect(encoded).not.toBe(json);
    expect(decoded).toBe(json);
  });

  it("rejects invalid shared payloads", async () => {
    await expect(decodeSharedState("not-valid-base64")).rejects.toBeTruthy();
  });
});
