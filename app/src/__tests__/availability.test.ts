import { describe, it, expect } from "vitest";
import { getTabAvailability } from "../utils/analysis";

describe("Tab Availability Logic", () => {
  it("should calculate availability correctly for exact birth time", () => {
    const vedicData = { report: {} } as any;
    const sajuData = { report: {} } as any;
    const transitData = { current_frame: {} } as any;
    const tierData = { destinyTier: {} } as any;
    const birthData = { unknown_time: false };

    const availability = getTabAvailability({
      sajuData,
      vedicData,
      transitData,
      tierData,
      unknownTime: birthData.unknown_time,
    });

    expect(availability.overview).toBe(true);
    expect(availability.saju).toBe(true);
    expect(availability.vedic_charts).toBe(true);
    expect(availability.strength).toBe(true);
    expect(availability.transit).toBe(true);
    expect(availability.simulation).toBe(true);
    expect(availability.destiny_tier).toBe(true);
  });

  it("should disable vedic_charts if birth time is unknown", () => {
    const vedicData = { report: {} } as any;
    const birthData = { unknown_time: true };

    const availability = getTabAvailability({
      sajuData: null,
      vedicData,
      transitData: null,
      tierData: null,
      unknownTime: !!birthData.unknown_time,
    });

    expect(availability.vedic_charts).toBe(false);
  });
});
