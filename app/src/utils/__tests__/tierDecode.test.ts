import { describe, it, expect } from "vitest";
import { TierResult } from "../../types/analysis";

describe("TierResult Decoding", () => {
  const sampleJson = {
    "natalScore": 74.0,
    "currentScore": 69.0,
    "destinyScore": 74.0,
    "destinyTier": {
      "grade": "B+",
      "label": "길상",
      "desc": "순한 기세 속에서 재능을 펼치는 운명"
    },
    "potentialScore": 76.0,
    "potentialTier": {
      "grade": "A",
      "label": "대길",
      "desc": "크게 길하며 성취가 확실한 운명"
    },
    "domainTiers": [
      { "house": 1, "domain": "자아·건강", "tier": "A" }
    ],
    "sajuResult": { "score": 55.0, "highlights": [] },
    "vedicResult": { "score": 38.0, "highlights": [] },
    "transitResult": { "score": 69.0, "highlights": [] },
    "strengths": [],
    "weaknesses": [],
    "growthGap": 2.0,
    "riskLevel": "low",
    "profile": "balanced",
    "version": "v3_spread_model",
    "destinyRawScore": 67.13736,
    "destinyTierScore": 74.0,
    "detailedComponents": [
      {
        "key": "saju_strength",
        "label": "사주 원국 강점",
        "score": 100.0,
        "weight": 0.12,
        "reasons": ["신강약 지수: 56.8"]
      }
    ],
    "tierModelVersion": "3.0.0"
  };

  it("should correctly map to TierResult interface", () => {
    const result: TierResult = sampleJson as any;
    
    expect(result.destinyTier.grade).toBe("B+");
    expect(result.potentialTier.grade).toBe("A");
    expect(result.detailedComponents).toHaveLength(1);
    expect(result.detailedComponents[0].key).toBe("saju_strength");
    expect(result.tierModelVersion).toBe("3.0.0");
    expect(result.destinyRawScore).toBeCloseTo(67.13736);
  });
});
