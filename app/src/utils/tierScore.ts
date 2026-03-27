/**
 * 운명/잠재력/분야별 티어 계산 v3 (Thin Fallback)
 * ─────────────────────────────────────────────────────────
 * v3 변경 사항:
 *   - 백엔드 SSOT 중심 아키텍처로 전환하여 프런트엔드 계산 로직 대폭 축소
 *   - 등급표 및 정규화 함수만 유지하여 일관성 보장
 */
import type { SajuAnalysisResult, TransitResult } from "../types";
import type { VedicAnalysisResult } from "../types";
import type { TierResult, TierGrade } from "../types/analysis";
export type { TierResult, TierGrade };

// ── 티어 등급표 (백엔드와 동기화) ────────────────────────────
export const TIER_GRADES = [
  { grade: "S+", label: "천기",    desc: "하늘의 기틀이 잡힌 극귀(極貴)의 운명" },
  { grade: "S",  label: "천운",    desc: "하늘이 돕고 땅이 비추는 대귀(大貴)의 운명" },
  { grade: "A+", label: "대길상",  desc: "복이 넘치며 만인이 부러워할 기세의 운명" },
  { grade: "A",  label: "대길",    desc: "크게 길하며 성취가 확실한 운명" },
  { grade: "B+", label: "길상",    desc: "순한 기세 속에서 재능을 펼치는 운명" },
  { grade: "B",  label: "길",      desc: "안정적이고 무난하게 번영할 운명" },
  { grade: "C+", label: "중상",    desc: "보통 이상의 저력이 있으며 노력이 빛을 발함" },
  { grade: "C",  label: "중평",    desc: "굴곡이 있으나 능히 헤쳐 나갈 수 있는 운명" },
  { grade: "D+", label: "중하",    desc: "고비가 잦으나 인내로써 길을 열어야 함" },
  { grade: "D",  label: "하평",    desc: "많은 주의를 요하며 신중한 처세가 필요한 시기" },
  { grade: "E",  label: "하하",    desc: "크나큰 인고와 역경 뒤에 겨우 싹을 틔울 운명" },
  { grade: "F",  label: "난국",    desc: "길이 험난하니 수양과 지혜로 화를 피해야 함" },
] as const;

export function getTierFromScore(score: number): (typeof TIER_GRADES)[number] {
  if (score >= 97) return TIER_GRADES[0];
  if (score >= 90) return TIER_GRADES[1];
  if (score >= 83) return TIER_GRADES[2];
  if (score >= 75) return TIER_GRADES[3];
  if (score >= 67) return TIER_GRADES[4];
  if (score >= 59) return TIER_GRADES[5];
  if (score >= 51) return TIER_GRADES[6];
  if (score >= 43) return TIER_GRADES[7];
  if (score >= 35) return TIER_GRADES[8];
  if (score >= 27) return TIER_GRADES[9];
  if (score >= 18) return TIER_GRADES[10];
  return TIER_GRADES[11];
}

// ── 정규화 유틸 ──────────────────────────────────────────────
export function spreadNormalize(score: number): number {
  const s = Math.min(100, Math.max(0, score));
  const normalized = s < 50
    ? 50 * Math.pow(s / 50, 1.4) 
    : 50 + 50 * Math.pow((s - 50) / 50, 0.7);
  return Math.round(normalized);
}

/**
 * [Thin Fallback] computeTierResult
 * 프런트엔드에서 최소한의 정보로 티어 구조를 생성합니다.
 * 실제 정밀 분석은 백엔드 응답(tierReport)을 사용하는 것이 원칙입니다.
 */
export function computeTierResult(
  _sajuReport: SajuAnalysisResult | null,
  _report: VedicAnalysisResult | null,
  _transitReport?: TransitResult | null,
): TierResult | null {
  // 기본적으로 fallback 결과는 생성하지 않고 null을 반환하여 
  // UI에서 "백엔드 결과 대기" 상태를 유도합니다.
  // 로컬 테스트나 특수 상황에서만 최소 구조를 반환할 수 있습니다.
  return null;
}
