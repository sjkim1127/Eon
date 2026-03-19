import { describe, it, expect } from "vitest";

describe("Tab Availability Logic", () => {
  it("should calculate availability correctly for exact birth time", () => {
    const vedicData = { report: {} };
    const sajuData = { report: {} };
    const transitData = { current_frame: {} };
    const tierData = { destinyTier: {} };
    const birthData = { unknown_time: false };
    const aiAuditData = { meta: {} };

    const availability = {
      overview: !!vedicData,
      saju: !!sajuData,
      vedic_charts: !!vedicData && !birthData.unknown_time,
      strength: !!sajuData,
      transit: !!transitData,
      destiny_tier: !!tierData,
      ai_audit: !!aiAuditData,
    };

    expect(availability.overview).toBe(true);
    expect(availability.saju).toBe(true);
    expect(availability.vedic_charts).toBe(true);
    expect(availability.strength).toBe(true);
    expect(availability.transit).toBe(true);
    expect(availability.destiny_tier).toBe(true);
    expect(availability.ai_audit).toBe(true);
  });

  it("should disable vedic_charts if birth time is unknown", () => {
    const vedicData = { report: {} };
    const birthData = { unknown_time: true };

    const availability = {
      vedic_charts: !!vedicData && !birthData.unknown_time,
    };

    expect(availability.vedic_charts).toBe(false);
  });
});
