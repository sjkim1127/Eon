/**
 * 운명/잠재력/분야별 티어 계산 — exportMarkdown 및 DestinyTierTab 공용
 */
import type { SajuAnalysisResult } from "../types";
import type { VedicAnalysisResult } from "../types";
import type { TransitResult } from "../types";

export const TIER_GRADES = [
  { grade: "S", label: "천운", desc: "사주와 별운이 모두 유리한 극상의 조합" },
  { grade: "A", label: "대길", desc: "전반적으로 아주 강한 기운의 조합" },
  { grade: "B", label: "길상", desc: "균형 잡힌 운세, 적극적으로 활용 가능" },
  { grade: "C", label: "중평", desc: "일부 어려움이 있으나 극복 가능한 조합" },
  { grade: "D", label: "다다익선", desc: "성장 여지가 많은 시기, 주의 시점 활용 권장" },
] as const;

export function getTierFromScore(score: number): (typeof TIER_GRADES)[number] {
  if (score >= 88) return TIER_GRADES[0];
  if (score >= 72) return TIER_GRADES[1];
  if (score >= 55) return TIER_GRADES[2];
  if (score >= 38) return TIER_GRADES[3];
  return TIER_GRADES[4];
}

const RATING_TO_TIER: Record<string, string> = {
  Excellent: "S",
  Strong: "A",
  Average: "B",
  Weak: "C",
};

const HOUSE_DOMAIN_LABELS: Record<number, string> = {
  1: "자아·건강", 2: "재물", 3: "형제·용기", 4: "가정·학업", 5: "자녀·창작", 6: "건강·노동",
  7: "관계·결혼", 8: "변화·유산", 9: "학문·행운", 10: "직업·명예", 11: "소원·수입", 12: "영성·은둔",
};

const BENEFIC_PLANETS = ["Jupiter", "Venus", "Mercury", "Moon"];

function computeSajuScore(saju: SajuAnalysisResult | null): { score: number; highlights: string[] } {
  if (!saju?.report) return { score: 0, highlights: [] };
  const r = saju.report;
  const st = r.strength;
  const highlights: string[] = [];
  const strengthNorm = Math.max(0, Math.min(100, (st.strength_score ?? 50) * 2));
  let sajuScore = strengthNorm * 0.35;
  if (st.deuk_ryeong?.acquired) { sajuScore += 8; highlights.push("득령: 계절의 도움"); }
  if (st.deuk_ji?.acquired) { sajuScore += 8; highlights.push("득지: 뿌리의 도움"); }
  if (st.deuk_si?.acquired) { sajuScore += 6; highlights.push("득시: 시간대의 도움"); }
  const supportRatio = st.deuk_se?.support_ratio ?? 0;
  sajuScore += supportRatio * 15;
  if (supportRatio > 0.5) highlights.push(`득세 지지비율 ${(supportRatio * 100).toFixed(0)}%`);
  const throughput = saju.qi_topology?.throughput ?? 0;
  sajuScore += throughput * 25;
  if (throughput > 0.7) highlights.push(`오행 흐름 원활 ${(throughput * 100).toFixed(0)}%`);
  if (r.golden_time) { sajuScore += 10; highlights.push(`골든타임 ${r.golden_time.start_age}~${r.golden_time.end_age}세`); }
  const vulnTotal = saju.vulnerability_report?.total_crashes ?? 0;
  if (vulnTotal > 40) sajuScore -= 5; else if (vulnTotal === 0 && saju.vulnerability_report) sajuScore += 5;
  const stabilityGrade = saju.complexity?.stability_grade ?? "";
  if (/^[AB]$/.test(stabilityGrade) || stabilityGrade.includes("A") || stabilityGrade.includes("B")) sajuScore += 4;
  else if (stabilityGrade.includes("D") || /High Entropy|Unstable/i.test(stabilityGrade)) sajuScore -= 3;
  if (r.yongshin?.primary) sajuScore += 2;
  return { score: Math.min(100, Math.max(0, Math.round(sajuScore))), highlights: highlights.slice(0, 5) };
}

function computeVedicScore(report: VedicAnalysisResult | null): { score: number; highlights: string[] } {
  if (!report?.report) return { score: 0, highlights: [] };
  const r = report.report;
  const chart = report.chart;
  const strengthNorm = Math.min(100, (r.overall_strength_score ?? 0) / 6);
  let vedicScore = strengthNorm * 0.5;
  const highlights: string[] = [];
  const yogas = r.yogas ?? [];
  const highYogas = yogas.filter((y: { quality: string | object }) => {
    const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
    return q === "VeryHigh" || q === "High";
  });
  vedicScore += Math.min(20, highYogas.length * 5);
  if (highYogas.length > 0) highlights.push(`우수 요가 ${highYogas.length}개`);
  const houseSummary = r.house_summary ?? [];
  const strongHouses = houseSummary.filter((h: { rating: string }) => h.rating === "Excellent" || h.rating === "Strong").length;
  vedicScore += (strongHouses / 12) * 20;
  if (strongHouses >= 6) highlights.push(`강한 하우스 ${strongHouses}개`);
  if (r.sade_sati === "None") { vedicScore += 5; highlights.push("사데사티 비해당"); }
  else if (r.sade_sati === "Peak" || r.sade_sati === "Rising") vedicScore -= 5;
  const dashaFocus = r.dasha_focus ?? "";
  if (BENEFIC_PLANETS.some((p) => dashaFocus.includes(p))) vedicScore += 4;
  const bhava = chart?.bhava_strengths ?? [];
  const h1 = bhava.find((b: { house: number }) => b.house === 1);
  const h10 = bhava.find((b: { house: number }) => b.house === 10);
  if ((h1?.total_score ?? 0) > 50) vedicScore += 1;
  if ((h10?.total_score ?? 0) > 50) vedicScore += 1;
  return { score: Math.min(100, Math.max(0, Math.round(vedicScore))), highlights: highlights.slice(0, 5) };
}

function computeTransitScore(transit: TransitResult | null | undefined): { score: number; highlights: string[] } {
  if (!transit) return { score: 0, highlights: [] };
  const highlights: string[] = [];
  const frame = transit.current_frame;
  const nearby = transit.nearby_diagnostics ?? [];
  let score = frame?.score != null ? Math.min(100, Math.max(0, frame.score)) : 50;
  if (score >= 70) highlights.push("현재 운세 긍정적");
  else if (score < 40) highlights.push("현재 운세 주의 필요");
  const badCount = nearby.filter((d: { status: string }) => d.status === "Overloaded" || d.status === "SystemDown").length;
  if (badCount > 0) { score -= Math.min(15, badCount * 5); highlights.push(`부하 구간 ${badCount}개`); }
  return { score: Math.min(100, Math.max(0, Math.round(score))), highlights: highlights.slice(0, 3) };
}

function computePotentialScore(saju: SajuAnalysisResult | null, report: VedicAnalysisResult | null): { score: number; highlights: string[] } {
  const highlights: string[] = [];
  let score = 50;
  if (saju?.report) {
    const throughput = saju.qi_topology?.throughput ?? 0;
    score += throughput * 15;
    if (throughput > 0.7) highlights.push("오행 흐름 원활");
    const stability = saju.complexity?.stability_grade ?? "";
    if (/^[AB]$/.test(stability) || /A|B/.test(stability)) { score += 8; highlights.push("안정적 성장 가능"); }
    if (saju.report.golden_time) { score += 10; highlights.push("골든타임 구간 있음"); }
    const recCount = saju.report.yongshin?.recommendations?.length ?? 0;
    if (recCount >= 2) { score += 5; highlights.push("용신 방향 명확"); }
    const vulnTotal = saju.vulnerability_report?.total_crashes ?? 99;
    if (vulnTotal < 10) score += 5; else if (vulnTotal > 40) score -= 5;
  }
  if (report?.report) {
    const yogas = report.report.yogas ?? [];
    const highYogas = yogas.filter((y: { quality: string | object }) => {
      const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
      return q === "VeryHigh" || q === "High";
    }).length;
    score += Math.min(12, highYogas * 4);
    if (highYogas > 0) highlights.push(`우수 요가 ${highYogas}개`);
    const excellentHouses = (report.report.house_summary ?? []).filter((h: { rating: string }) => h.rating === "Excellent").length;
    score += Math.min(10, excellentHouses * 3);
    if (excellentHouses >= 3) highlights.push(`강한 하우스 ${excellentHouses}개`);
  }
  return { score: Math.min(100, Math.max(0, Math.round(score))), highlights: highlights.slice(0, 5) };
}

function getDomainTiers(report: VedicAnalysisResult | null): { house: number; domain: string; tier: string }[] {
  if (!report?.report?.house_summary?.length) return [];
  return report.report.house_summary.map((h: { house: number; rating: string }) => ({
    house: h.house,
    domain: HOUSE_DOMAIN_LABELS[h.house] ?? `H${h.house}`,
    tier: RATING_TO_TIER[h.rating] ?? "D",
  }));
}

export interface TierResult {
  natalScore: number;
  currentScore: number;
  destinyScore: number;
  destinyTier: (typeof TIER_GRADES)[number];
  potentialScore: number;
  potentialTier: (typeof TIER_GRADES)[number];
  domainTiers: { house: number; domain: string; tier: string }[];
  sajuResult: { score: number; highlights: string[] };
  vedicResult: { score: number; highlights: string[] };
  transitResult: { score: number; highlights: string[] };
  strengths: string[];
  weaknesses: string[];
}

export function computeTierResult(
  sajuReport: SajuAnalysisResult | null,
  report: VedicAnalysisResult | null,
  transitReport?: TransitResult | null
): TierResult | null {
  const hasSaju = !!sajuReport?.report;
  const hasVedic = !!report?.report;
  if (!hasSaju && !hasVedic) return null;
  const sajuResult = computeSajuScore(sajuReport);
  const vedicResult = computeVedicScore(report);
  const transitResult = computeTransitScore(transitReport);
  const potentialResult = computePotentialScore(sajuReport, report);
  const domainTiers = getDomainTiers(report);
  const natalScore = hasSaju && hasVedic ? (sajuResult.score * 0.5 + vedicResult.score * 0.5) : hasSaju ? sajuResult.score : vedicResult.score;
  const hasTransit = !!transitReport?.current_frame;
  const destinyScore = hasTransit ? natalScore * 0.7 + transitResult.score * 0.3 : natalScore;
  const destinyTier = getTierFromScore(destinyScore);
  const potentialTier = getTierFromScore(potentialResult.score);
  const strengths = [...sajuResult.highlights.slice(0, 2), ...vedicResult.highlights.slice(0, 2)].filter(Boolean).slice(0, 3);
  if (sajuReport?.report?.golden_time) strengths.push(`골든타임 ${sajuReport.report.golden_time.start_age}~${sajuReport.report.golden_time.end_age}세`);
  const weaknesses: string[] = [];
  const vulnTotal = sajuReport?.vulnerability_report?.total_crashes ?? 0;
  if (vulnTotal > 0) weaknesses.push(`주의 시점 ${vulnTotal}개`);
  if (report?.report?.sade_sati === "Peak" || report?.report?.sade_sati === "Rising") weaknesses.push("사데사티 진행 중");
  return {
    natalScore,
    currentScore: transitResult.score,
    destinyScore,
    destinyTier,
    potentialScore: potentialResult.score,
    potentialTier,
    domainTiers,
    sajuResult,
    vedicResult,
    transitResult,
    strengths: strengths.slice(0, 3),
    weaknesses: weaknesses.slice(0, 2),
  };
}
