/**
 * 운명/잠재력/분야별 티어 계산 — exportMarkdown 및 DestinyTierTab 공용
 */
import type { SajuAnalysisResult } from "../types";
import type { VedicAnalysisResult } from "../types";
import type { TransitResult } from "../types";

export const TIER_GRADES = [
  { grade: "S+", label: "천기",   desc: "사주와 별운이 완전히 일치하는 극희귀 최상의 조합" },
  { grade: "S",  label: "천운",   desc: "사주와 별운이 모두 유리한 극상의 조합" },
  { grade: "A+", label: "대길상", desc: "용신·대운·요가가 거의 완벽하게 지원하는 강운" },
  { grade: "A",  label: "대길",   desc: "전반적으로 아주 강한 기운의 조합" },
  { grade: "B+", label: "길상",   desc: "균형이 잡히고 강점이 뚜렷하게 빛나는 운세" },
  { grade: "B",  label: "길",     desc: "전반적으로 안정적이고 활용 가능한 운세" },
  { grade: "C+", label: "중상",   desc: "보통 이상의 기운, 노력으로 충분히 도약 가능" },
  { grade: "C",  label: "중평",   desc: "일부 어려움이 있으나 극복 가능한 조합" },
  { grade: "D+", label: "성장예비", desc: "성장 여지가 많으며 조건이 갖춰지면 빠른 상향 가능" },
  { grade: "D",  label: "다다익선", desc: "성장 여지가 많은 시기, 주의 시점 활용 권장" },
] as const;

// 10단계 티어 기준 (score → grade)
// 사용자 점수 34 → D+ 로 인식되도록 설계
// S+: 93+, S: 85+, A+: 77+, A: 69+, B+: 61+, B: 53+, C+: 45+, C: 37+, D+: 29+, D: <29
export function getTierFromScore(score: number): (typeof TIER_GRADES)[number] {
  if (score >= 93) return TIER_GRADES[0]; // S+
  if (score >= 85) return TIER_GRADES[1]; // S
  if (score >= 77) return TIER_GRADES[2]; // A+
  if (score >= 69) return TIER_GRADES[3]; // A
  if (score >= 61) return TIER_GRADES[4]; // B+
  if (score >= 53) return TIER_GRADES[5]; // B
  if (score >= 45) return TIER_GRADES[6]; // C+
  if (score >= 37) return TIER_GRADES[7]; // C
  if (score >= 29) return TIER_GRADES[8]; // D+
  return TIER_GRADES[9];                  // D
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

const clampScore = (score: number): number => Math.min(100, Math.max(0, score));

// 고득점 구간만 압축하는 정규화
// ▸ 50 이하: 그대로 통과 (하방 압축 금지 — x^1.3 곡선이 45→35로 왜곡하던 버그 수정)
// ▸ 50 초과: 상단 35점 범위에서 x^1.4 압축 적용 (80→67, 90→76, 100→85)
const softNormalizeForDestiny = (score: number): number => {
  const clamped = clampScore(score);
  if (clamped <= 50) return clamped;
  const excess = (clamped - 50) / 50; // 0~1
  const compressed = Math.pow(excess, 1.4) * 35; // 최대 35점 추가
  return Math.round(50 + compressed);
};

// 흉격 계열 격국
const WEAK_STRUCTURES = new Set(["Follower", "FollowerResource", "FollowerStrength"]);
// 길한 12운성
const GOOD_12_STAGES = new Set(["장생", "건록", "제왕", "관대", "목욕"]);
// 흉한 12운성
const BAD_12_STAGES = new Set(["절", "묘", "태"]);

function computeSajuScore(saju: SajuAnalysisResult | null): { score: number; highlights: string[] } {
  if (!saju?.report) return { score: 0, highlights: [] };
  const r = saju.report;
  const st = r.strength;
  const highlights: string[] = [];
  let sajuScore = 0;

  // ── 1. 신강/신약 (strength_score 0~50 → 정규화) ──
  const strengthNorm = clampScore((st.strength_score ?? 25) * 2);
  sajuScore += strengthNorm * 0.28; // 최대 28점

  // ── 2. 득령/득지/득시 ──
  const acquired = [st.deuk_ryeong?.acquired, st.deuk_ji?.acquired, st.deuk_si?.acquired].filter(Boolean).length;
  sajuScore += acquired * 4; // 최대 12점
  if (acquired === 3) highlights.push("삼득(득령·득지·득시) 완성");
  else if (acquired >= 2) highlights.push(`득령·득지·득시 ${acquired}개 달성`);

  // ── 3. 득세 지지비율 ──
  const rawSupportRatio = st.deuk_se?.support_ratio ?? 0;
  const supportPct = rawSupportRatio > 1 ? rawSupportRatio : rawSupportRatio * 100;
  sajuScore += (supportPct / 100) * 10; // 최대 10점
  if (supportPct > 60) highlights.push(`득세 지지비율 ${supportPct.toFixed(0)}% (우세)`);
  else if (supportPct < 30) { /* 약세 — 감점 없음, 이미 낮은 값 */ }

  // ── 4. 오행 에너지 흐름 ──
  const throughput = saju.qi_topology?.throughput ?? 0;
  sajuScore += throughput * 12; // 최대 12점
  if (throughput > 0.75) highlights.push(`오행 흐름 ${(throughput * 100).toFixed(0)}% (원활)`);
  // 병목 페널티
  if (saju.qi_topology?.bottleneck) sajuScore -= 2;

  // ── 5. 격국 유형 ──
  const structure = r.structure?.structure ?? "";
  if (WEAK_STRUCTURES.has(structure)) sajuScore -= 3; // 종격류
  else if (structure) sajuScore += 2; // 일반 내격

  // ── 6. 신살 — 길신·흉신 비율 ──
  const auspicious = r.spirit_markers?.auspicious?.length ?? 0;
  const inauspicious = r.spirit_markers?.inauspicious?.length ?? 0;
  sajuScore += Math.min(5, auspicious * 1); // 길신 최대 +5
  sajuScore -= Math.min(4, inauspicious * 0.8); // 흉신 최대 -4
  if (auspicious > 0) highlights.push(`길신 ${auspicious}개 (천을귀인 등)`);
  if (inauspicious >= 3) highlights.push(`흉신 ${inauspicious}개 주의`);

  // ── 7. 골든타임 ──
  if (r.golden_time) {
    const gtLength = (r.golden_time.end_age - r.golden_time.start_age);
    sajuScore += Math.min(6, gtLength * 0.4); // 길이에 비례 최대 6점
    highlights.push(`골든타임 ${r.golden_time.start_age}~${r.golden_time.end_age}세 (${gtLength}년)`);
  }

  // ── 8. 시뮬레이션 프레임 — 평균 점수 + 고득점 구간 비율 ──
  const frames = r.simulation_frames ?? [];
  if (frames.length > 0) {
    const avg = frames.reduce((s, f) => s + (f.score ?? 50), 0) / frames.length;
    const goodPct = frames.filter(f => (f.score ?? 0) >= 65).length / frames.length;
    sajuScore += Math.min(8, (avg - 50) * 0.16); // 평균 50 기준 최대 ±8
    sajuScore += goodPct * 5; // 좋은 구간 비율 최대 5점
    if (avg >= 70) highlights.push(`시뮬레이션 평균 ${avg.toFixed(0)}점 (우수)`);
  }

  // ── 9. 주의 시점 (vulnerability) ──
  const vulnTotal = saju.vulnerability_report?.total_crashes ?? 0;
  if (vulnTotal === 0 && saju.vulnerability_report) { sajuScore += 4; highlights.push("충돌 주의 시점 없음"); }
  else if (vulnTotal > 40) { sajuScore -= 5; highlights.push(`주의 시점 ${vulnTotal}개`); }
  else if (vulnTotal > 20) sajuScore -= 2;

  // ── 10. 안정성 등급 ──
  const stabilityGrade = saju.complexity?.stability_grade ?? "";
  const entropyScore = saju.entropy?.score ?? 1.0;
  // 엔트로피가 이미 높으면(>1.5) stability D 패널티를 절반으로 줄여 이중 패널티 방지
  const isHighEntropy = entropyScore > 1.5;
  if (/^A/.test(stabilityGrade)) { sajuScore += 4; highlights.push("안정성 A등급"); }
  else if (/^B/.test(stabilityGrade)) sajuScore += 2;
  else if (/^D/.test(stabilityGrade) || /Unstable|High Entropy/i.test(stabilityGrade)) {
    sajuScore -= isHighEntropy ? 1.5 : 3; // 엔트로피 중복 패널티 완화
  }

  // ── 11. 린트 — Error 페널티, 클린 보너스 ──
  const lints = saju.lints ?? [];
  const errorCount = lints.filter(l => l.severity === "Error").length;
  const warnCount = lints.filter(l => l.severity === "Warning").length;
  if (errorCount === 0 && warnCount === 0) { sajuScore += 2; highlights.push("사주 구조 클린"); }
  sajuScore -= Math.min(5, errorCount * 1.5 + warnCount * 0.4);

  // ── 12. 운명 복잡도 엔트로피 ──
  // entropy.score는 0~3 범위 (백엔드 실제 스케일) — 과거 >0.7 기준은 버그
  if (entropyScore < 1.0) { sajuScore += 2; highlights.push("운명 패턴 안정 (예측 가능)"); }
  else if (entropyScore > 2.0) { sajuScore -= 2; highlights.push("운명 변수 많음"); }

  // ── 13. 용신 추천 명확도 ──
  const recCount = r.yongshin?.recommendations?.length ?? 0;
  if (recCount >= 2) sajuScore += 2; // 용신 방향이 여럿으로 명확

  return { score: clampScore(Math.round(sajuScore)), highlights: highlights.slice(0, 6) };
}

function computeVedicScore(report: VedicAnalysisResult | null): { score: number; highlights: string[] } {
  if (!report?.report) return { score: 0, highlights: [] };
  const r = report.report;
  const chart = report.chart;
  const highlights: string[] = [];
  let vedicScore = 0;

  // ── 1. 전체 차트 강도 (0~600 → 0~100 정규화) ──
  const strengthNorm = clampScore((r.overall_strength_score ?? 0) / 6);
  vedicScore += strengthNorm * 0.35; // 최대 35점

  // ── 2. 요가 품질 ──
  const yogas = r.yogas ?? [];
  const veryHighYogas = yogas.filter((y: { quality: string | object }) => {
    const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
    return q === "VeryHigh";
  });
  const highYogas = yogas.filter((y: { quality: string | object }) => {
    const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
    return q === "High";
  });
  vedicScore += Math.min(12, veryHighYogas.length * 4 + highYogas.length * 2);
  if (veryHighYogas.length > 0) highlights.push(`최상급 요가 ${veryHighYogas.length}개`);
  else if (highYogas.length > 0) highlights.push(`우수 요가 ${highYogas.length}개`);

  // ── 3. 하우스 강도 분류 ──
  const houseSummary = r.house_summary ?? [];
  const excellentHouses = houseSummary.filter((h: { rating: string }) => h.rating === "Excellent").length;
  const strongHouses = houseSummary.filter((h: { rating: string }) => h.rating === "Strong").length;
  const weakHouses = houseSummary.filter((h: { rating: string }) => h.rating === "Weak").length;
  vedicScore += Math.min(10, excellentHouses * 2 + strongHouses * 0.8);
  vedicScore -= Math.min(6, weakHouses * 1.2);
  if (excellentHouses >= 4) highlights.push(`최강 하우스 ${excellentHouses}개`);
  else if (strongHouses + excellentHouses >= 6) highlights.push(`강한 하우스 ${strongHouses + excellentHouses}개`);

  // ── 4. 핵심 하우스 강도 (1·5·9·10—자아·운·지혜·명예) ──
  const bhava = chart?.bhava_strengths ?? [];
  const KEY_HOUSES = [1, 5, 9, 10];
  let keyHouseBonus = 0;
  for (const h of KEY_HOUSES) {
    const bh = bhava.find((b: { house: number }) => b.house === h);
    if ((bh?.total_score ?? 0) >= 60) keyHouseBonus += 1.5;
    else if ((bh?.total_score ?? 0) >= 40) keyHouseBonus += 0.5;
  }
  vedicScore += Math.min(6, keyHouseBonus);
  if (keyHouseBonus >= 4) highlights.push("핵심 하우스(1·5·9·10) 강화");

  // ── 5. 사데사티 ──
  if (r.sade_sati === "None") { vedicScore += 5; highlights.push("사데사티 비해당"); }
  else if (r.sade_sati === "Peak") { vedicScore -= 6; highlights.push("사데사티 절정 진행 중"); }
  else if (r.sade_sati === "Rising") { vedicScore -= 3; highlights.push("사데사티 상승 진행 중"); }
  else if (r.sade_sati === "Setting") vedicScore -= 1; // 하강은 일부 회복

  // ── 6. 현재 다샤 기준 길성 여부 ──
  const dashaFocus = r.dasha_focus ?? "";
  if (BENEFIC_PLANETS.some((p) => dashaFocus.includes(p))) { vedicScore += 4; highlights.push(`길성 다샤 (${dashaFocus.split(" ")[0]})`); }

  // ── 7. SAV 포인트 — 28+ 하우스가 많을수록 강함 ──
  const savPoints = chart?.sav?.points ?? [];
  if (Array.isArray(savPoints) && savPoints.length === 12) {
    const strongSav = savPoints.filter((p: number) => p >= 28).length;
    const weakSav = savPoints.filter((p: number) => p <= 22).length;
    vedicScore += Math.min(5, strongSav * 0.8);
    vedicScore -= Math.min(4, weakSav * 0.8);
    if (strongSav >= 6) highlights.push(`SAV 강점 하우스 ${strongSav}개`);
  }

  // ── 8. 빔쇼파카 발라 평균 (0~20 → ×5 환산) ──
  const vimshopaka = chart?.vimshopaka_scores ?? [];
  if (vimshopaka.length > 0) {
    const avgVim = vimshopaka.reduce((sum: number, [, v]: [string, { shadvarga_score: number; shodashavarga_score: number }]) => {
      return sum + ((v.shadvarga_score + v.shodashavarga_score) / 2);
    }, 0) / vimshopaka.length;
    vedicScore += Math.min(6, (avgVim / 20) * 6); // 최대 6점
    if (avgVim >= 14) highlights.push(`빔쇼파카 발라 평균 ${avgVim.toFixed(1)} (강함)`);
  }

  // ── 9. 역행·연소 행성 페널티 ──
  const planets = chart?.planets ?? [];
  const retroCount = planets.filter((p: { is_retrograde: boolean }) => p.is_retrograde).length;
  const combustCount = planets.filter((p: { is_combust: boolean }) => p.is_combust).length;
  vedicScore -= Math.min(4, retroCount * 0.8);
  vedicScore -= Math.min(3, combustCount * 0.8);
  if (retroCount + combustCount >= 3) highlights.push(`역행 ${retroCount}+연소 ${combustCount}개 (약화)`);

  // ── 10. 아바스타 (행성 상태) — Bala=활성, Mrita=사망 ──
  const avasthas = chart?.avasthas ?? [];
  if (avasthas.length > 0) {
    const balaCount = avasthas.filter((a: { baladi: string }) => a.baladi === "Bala" || a.baladi === "Yuva").length;
    const mrtaCount = avasthas.filter((a: { baladi: string }) => a.baladi === "Mrita" || a.baladi === "Vridha").length;
    vedicScore += Math.min(3, balaCount * 0.6);
    vedicScore -= Math.min(3, mrtaCount * 0.7);
    if (balaCount >= 4) highlights.push(`활성 행성 ${balaCount}개 (Bala·Yuva)`);
  }

  return { score: clampScore(Math.round(vedicScore)), highlights: highlights.slice(0, 6) };
}

function computeTransitScore(transit: TransitResult | null | undefined): { score: number; highlights: string[] } {
  if (!transit) return { score: 0, highlights: [] };
  const highlights: string[] = [];
  const frame = transit.current_frame;
  const nearby = transit.nearby_diagnostics ?? [];
  let score = frame?.score != null ? clampScore(frame.score) : 50;

  // ── 1. 현재 프레임 점수 기반 라벨 ──
  if (score >= 75) highlights.push("현재 운세 긍정적 (맑음)");
  else if (score >= 55) highlights.push("현재 운세 보통 (구름)");
  else if (score < 40) highlights.push("현재 운세 주의 필요");

  // ── 2. 근처 부하 진단 ──
  const badCount = nearby.filter((d: { status: string }) => d.status === "Overloaded" || d.status === "SystemDown").length;
  if (badCount > 0) { score -= Math.min(15, badCount * 5); highlights.push(`근처 부하 구간 ${badCount}개`); }
  const downCount = nearby.filter((d: { status: string }) => d.status === "SystemDown").length;
  if (downCount > 0) score -= downCount * 3; // SystemDown 추가 페널티

  // ── 3. 세운 12운성 ──
  const yearlyStage = transit.yearly_luck?.twelve_stage ?? "";
  if (GOOD_12_STAGES.has(yearlyStage)) { score += 5; highlights.push(`세운 12운성: ${yearlyStage} (길)`); }
  else if (BAD_12_STAGES.has(yearlyStage)) { score -= 5; highlights.push(`세운 12운성: ${yearlyStage} (흉)`); }

  // ── 4. 월운 12운성 ──
  const monthlyStage = transit.monthly_luck?.twelve_stage ?? "";
  if (GOOD_12_STAGES.has(monthlyStage)) { score += 3; highlights.push(`월운 12운성: ${monthlyStage} (길)`); }
  else if (BAD_12_STAGES.has(monthlyStage)) score -= 3;

  // ── 5. 세운 특이 이벤트 ──
  const events = transit.yearly_luck?.special_events ?? [];
  if (events.length > 0) {
    score += Math.min(4, events.length * 1.5);
    highlights.push(`길조 이벤트 ${events.length}개`);
  }

  // ── 6. 세운·월운 원국 관계 (합이 있을수록 활성화) ──
  const yearRelations = transit.yearly_luck?.influence?.relations_with_natal ?? [];
  const goodRelations = yearRelations.filter((rel: string) => /합|록|귀인|삼합|육합/.test(rel)).length;
  const badRelations = yearRelations.filter((rel: string) => /충|형|해|파/.test(rel)).length;
  score += Math.min(4, goodRelations * 1.5);
  score -= Math.min(4, badRelations * 1.5);

  return { score: clampScore(Math.round(score)), highlights: highlights.slice(0, 4) };
}

function computePotentialScore(saju: SajuAnalysisResult | null, report: VedicAnalysisResult | null): { score: number; highlights: string[] } {
  const highlights: string[] = [];
  let score = 50;

  if (saju?.report) {
    // 오행 흐름
    const throughput = saju.qi_topology?.throughput ?? 0;
    score += throughput * 12;
    if (throughput > 0.7) highlights.push("오행 흐름 원활 (잠재력 발현 유리)");

    // 안정성 등급
    const stability = saju.complexity?.stability_grade ?? "";
    if (/^A/.test(stability)) { score += 8; highlights.push("안정성 A등급 (성장 기반 탄탄)"); }
    else if (/^B/.test(stability)) score += 4;
    else if (/^D/.test(stability)) score -= 4;

    // 골든타임 — 길이에 비례
    const gt = saju.report.golden_time;
    if (gt) {
      const gtLen = gt.end_age - gt.start_age;
      score += Math.min(12, gtLen * 0.5); // 24년 골든타임 → +12
      highlights.push(`골든타임 ${gt.start_age}~${gt.end_age}세 (${gtLen}년)`);
    }

    // 용신 추천 방향
    const recCount = saju.report.yongshin?.recommendations?.length ?? 0;
    if (recCount >= 2) { score += 4; highlights.push("용신 방향 명확 (잠재력 활용 가능)"); }

    // 주의 시점 — 적을수록 성장 여지
    const vulnTotal = saju.vulnerability_report?.total_crashes ?? 99;
    if (vulnTotal === 0) { score += 5; } else if (vulnTotal < 10) score += 3;
    else if (vulnTotal > 40) score -= 5;

    // 길신(신살)
    const auspicious = saju.report.spirit_markers?.auspicious?.length ?? 0;
    score += Math.min(5, auspicious * 1.2);
    if (auspicious >= 3) highlights.push(`길신 ${auspicious}개 (잠재력 촉매)`);

    // 린트 클린
    const errorCount = (saju.lints ?? []).filter(l => l.severity === "Error").length;
    if (errorCount === 0) score += 3;
    else score -= errorCount * 2;

    // 시뮬레이션 고득점 구간 비율
    const frames = saju.report.simulation_frames ?? [];
    if (frames.length > 0) {
      const goodPct = frames.filter(f => (f.score ?? 0) >= 70).length / frames.length;
      score += goodPct * 8; // 최대 8점
      if (goodPct >= 0.5) highlights.push(`우호 구간 ${(goodPct * 100).toFixed(0)}% (절반 이상 순풍)`);
    }
  }

  if (report?.report) {
    // 베딕 요가
    const yogas = report.report.yogas ?? [];
    const highYogas = yogas.filter((y: { quality: string | object }) => {
      const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
      return q === "VeryHigh" || q === "High";
    }).length;
    score += Math.min(10, highYogas * 3);
    if (highYogas > 0) highlights.push(`상급 요가 ${highYogas}개 (잠재력 극대화)`);

    // Excellent 하우스
    const excellentHouses = (report.report.house_summary ?? []).filter((h: { rating: string }) => h.rating === "Excellent").length;
    score += Math.min(8, excellentHouses * 2.5);
    if (excellentHouses >= 3) highlights.push(`최강 하우스 ${excellentHouses}개`);

    // 사데사티 없으면 성장 가능성 ↑
    if (report.report.sade_sati === "None") score += 3;
    else if (report.report.sade_sati === "Peak") score -= 5;
  }

  return { score: clampScore(Math.round(score)), highlights: highlights.slice(0, 5) };
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
  growthGap: number;
  riskLevel: "low" | "medium" | "high";
  profile: "stable" | "balanced" | "growth";
  version: "v1" | "v2";
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
  const natalScoreRaw = hasSaju && hasVedic
    ? (sajuResult.score * 0.5 + vedicResult.score * 0.5)
    : hasSaju
      ? sajuResult.score
      : vedicResult.score;

  // 연령·프로필에 따른 원국/현재 가중치
  let natalWeight = 0.7;
  let currentWeight = 0.3;
  let profile: TierResult["profile"] = "balanced";
  const age = transitReport?.current_age;
  if (typeof age === "number") {
    if (age < 35) {
      natalWeight = 0.6;
      currentWeight = 0.4;
      profile = "growth";
    } else if (age > 55) {
      natalWeight = 0.8;
      currentWeight = 0.2;
      profile = "stable";
    }
  }

  const hasTransit = !!transitReport?.current_frame;
  let currentScoreForDestiny = transitResult.score;
  if (hasTransit) {
    // 현재 점수 캡핑 (극단값 완화)
    if (currentScoreForDestiny < 25) currentScoreForDestiny = 25;
    if (currentScoreForDestiny > 90) currentScoreForDestiny = 90;
  }

  const natalScoreNormalized = softNormalizeForDestiny(natalScoreRaw);
  const currentScoreNormalized = hasTransit ? softNormalizeForDestiny(currentScoreForDestiny) : 0;

  let destinyScore = hasTransit
    ? natalScoreNormalized * natalWeight + currentScoreNormalized * currentWeight
    : natalScoreNormalized;

  // 분야별 티어에 따른 소폭 보정
  if (domainTiers.length > 0) {
    let domainAdjustment = 0;
    let penaltyFocus = 0;
    for (const d of domainTiers) {
      if (d.tier === "S") domainAdjustment += 0.5;
      else if (d.tier === "A") domainAdjustment += 0.2;
      else if (d.tier === "C") domainAdjustment -= 0.5;
      else if (d.tier === "D") domainAdjustment -= 1;

      if ([1, 2, 6, 10, 11].includes(d.house)) {
        if (d.tier === "C") penaltyFocus -= 0.5;
        else if (d.tier === "D") penaltyFocus -= 1;
      }
    }
    domainAdjustment = Math.max(-4, Math.min(5, domainAdjustment + penaltyFocus));
    destinyScore += domainAdjustment;
  }

  destinyScore = clampScore(destinyScore);
  const destinyTier = getTierFromScore(destinyScore);
  const potentialTier = getTierFromScore(potentialResult.score);

  // 강점: 사주·베딕·운세 하이라이트에서 최대 5개
  const strengths = [
    ...sajuResult.highlights.slice(0, 2),
    ...vedicResult.highlights.slice(0, 2),
    ...transitResult.highlights.filter(h => !h.includes("주의") && !h.includes("부하")).slice(0, 1),
  ].filter(Boolean).slice(0, 5);

  // 약점: 주의 시점, 사데사티, 부하진단, 흉신, 린트
  const weaknesses: string[] = [];
  const vulnTotal = sajuReport?.vulnerability_report?.total_crashes ?? 0;
  if (vulnTotal > 30) weaknesses.push(`주의 시점 ${vulnTotal}개 (위험 구간 다수)`);
  else if (vulnTotal > 0) weaknesses.push(`주의 시점 ${vulnTotal}개`);
  if (report?.report?.sade_sati === "Peak") weaknesses.push("사데사티 절정 — 토성 압박 최고조");
  else if (report?.report?.sade_sati === "Rising") weaknesses.push("사데사티 상승 진행 중");
  const inauspicious = sajuReport?.report?.spirit_markers?.inauspicious?.length ?? 0;
  if (inauspicious >= 3) weaknesses.push(`흉신 ${inauspicious}개 (겁살·망신살 등)`);
  const errorLints = (sajuReport?.lints ?? []).filter(l => l.severity === "Error");
  if (errorLints.length > 0) weaknesses.push(`사주 구조 오류 ${errorLints.length}개 — ${errorLints[0].code}`);
  const badTransit = transitResult.highlights.filter(h => h.includes("주의") || h.includes("부하") || h.includes("흉"));
  if (badTransit.length > 0) weaknesses.push(badTransit[0]);

  // 잠재력-운명 갭 및 리스크 레벨
  const growthGap = clampScore(potentialResult.score) - destinyScore;
  let riskPoints = 0;
  if (vulnTotal >= 10) riskPoints += 1;
  if (vulnTotal >= 20) riskPoints += 1;
  if (vulnTotal >= 40) riskPoints += 1;
  if (transitResult.score < 40) riskPoints += 1;
  if (transitResult.score < 30) riskPoints += 1;
  if (report?.report?.sade_sati === "Peak" || report?.report?.sade_sati === "Rising") riskPoints += 2;
  const nearby = transitReport?.nearby_diagnostics ?? [];
  const badNearby = nearby.filter((d: { status: string }) => d.status === "Overloaded" || d.status === "SystemDown").length;
  if (badNearby >= 2) riskPoints += 1;
  const riskLevel: TierResult["riskLevel"] =
    riskPoints <= 1 ? "low" : riskPoints <= 3 ? "medium" : "high";

  return {
    natalScore: clampScore(Math.round(natalScoreRaw)),
    currentScore: transitResult.score,
    destinyScore,
    destinyTier,
    potentialScore: potentialResult.score,
    potentialTier,
    domainTiers,
    sajuResult,
    vedicResult,
    transitResult,
    strengths: strengths.slice(0, 5),
    weaknesses: weaknesses.slice(0, 4),
    growthGap,
    riskLevel,
    profile,
    version: "v2",
  };
}
