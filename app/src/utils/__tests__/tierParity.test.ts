import { describe, it, expect } from "vitest";
import { getTierFromScore, spreadNormalize } from "../tierScore";

describe("Tier v3 Contract Parity", () => {
  it("should have the same tier cutoffs as backend", () => {
    // S+: 97, S: 90, A+: 83, A: 75, B+: 67, B: 59, C+: 51, C: 43, D+: 35, D: 27, E: 18, F: <18
    expect(getTierFromScore(97).grade).toBe("S+");
    expect(getTierFromScore(96).grade).toBe("S");
    expect(getTierFromScore(90).grade).toBe("S");
    expect(getTierFromScore(89).grade).toBe("A+");
    expect(getTierFromScore(83).grade).toBe("A+");
    expect(getTierFromScore(82).grade).toBe("A");
    expect(getTierFromScore(18).grade).toBe("E");
    expect(getTierFromScore(17).grade).toBe("F");
  });

  it("should have identical spreadNormalize logic", () => {
    // 50.0 * (s / 50.0).powf(1.4) for s < 50
    // 50.0 + 50.0 * ((s - 50.0) / 50.0).powf(0.7) for s >= 50
    
    // Check points
    expect(spreadNormalize(50)).toBe(50);
    expect(spreadNormalize(0)).toBe(0);
    expect(spreadNormalize(100)).toBe(100);
    
    // s = 25 -> 50 * (0.5 ^ 1.4) = 50 * 0.3789 = 18.94 -> 19
    expect(spreadNormalize(25)).toBe(19);
    
    // s = 75 -> 50 + 50 * (0.5 ^ 0.7) = 50 + 50 * 0.6155 = 50 + 30.77 = 80.77 -> 81
    expect(spreadNormalize(75)).toBe(81);
  });
});
