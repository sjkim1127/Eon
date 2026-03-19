import type { SajuAnalysisResult, TransitResult, VedicAnalysisResult } from "../types";
import type { AiAuditReport, TierResult } from "../types/analysis";

export interface TabAvailability {
  overview: boolean;
  saju: boolean;
  vedic_charts: boolean;
  strength: boolean;
  transit: boolean;
  destiny_tier: boolean;
  ai_audit: boolean;
}

export interface AvailabilitySource {
  sajuData: SajuAnalysisResult | null;
  vedicData: VedicAnalysisResult | null;
  transitData: TransitResult | null;
  aiAuditData: AiAuditReport| null;
  tierData: TierResult | null;
  unknownTime: boolean;
}

/**
 * 탭별 활성화 상태를 계산합니다.
 */
export function getTabAvailability(source: AvailabilitySource): TabAvailability {
  const { sajuData, vedicData, transitData, aiAuditData, tierData, unknownTime } = source;
  
  return {
    overview: !!vedicData,
    saju: !!sajuData,
    vedic_charts: !!vedicData && !unknownTime,
    strength: !!sajuData,
    transit: !!transitData,
    destiny_tier: !!tierData,
    ai_audit: !!aiAuditData,
  };
}
