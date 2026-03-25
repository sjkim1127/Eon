/**
 * 운명/잠재력/분야별 티어 계산 v3
 * ─────────────────────────────────────────────────────────
 * v3 변경 사항:
 *   A. 서브스코어 상한 재정규화 → 이론 max 100점
 *   B. 12하우스 → 6개 도메인 그룹 + 핵심 하우스 가중
 *   C. 4축 다차원 리스크 인덱스 (취약점/대운교체/베딕압박/현재운)
 *      + riskLevel에 "critical" 추가
 */
import type { SajuAnalysisResult, TransitResult } from "../types";
import type { VedicAnalysisResult } from "../types";
import type { TierResult, TierGrade } from "../types/analysis";
export type { TierResult, TierGrade };

// ── 티어 등급표 ────────────────────────────────────────────
export const TIER_GRADES = [
  { grade: "S+", label: "천기",    desc: "사주와 별운이 완전히 일치하는 극희귀 최상의 조합" },
  { grade: "S",  label: "천운",    desc: "사주와 별운이 모두 유리한 극상의 조합" },
  { grade: "A+", label: "대길상",  desc: "용신·대운·요가가 거의 완벽하게 지원하는 강운" },
  { grade: "A",  label: "대길",    desc: "전반적으로 아주 강한 기운의 조합" },
  { grade: "B+", label: "길상",    desc: "균형이 잡히고 강점이 뚜렷하게 빛나는 운세" },
  { grade: "B",  label: "길",      desc: "전반적으로 안정적이고 활용 가능한 운세" },
  { grade: "C+", label: "중상",    desc: "보통 이상의 기운, 노력으로 충분히 도약 가능" },
  { grade: "C",  label: "중평",    desc: "일부 어려움이 있으나 극복 가능한 조합" },
  { grade: "D+", label: "성장예비", desc: "성장 여지가 많으며 조건이 갖춰지면 빠른 상향 가능" },
  { grade: "D",  label: "다다익선", desc: "성장 여지가 많은 시기, 주의 시점 활용 권장" },
] as const;

// S+: 93+, S: 85+, A+: 77+, A: 69+, B+: 61+, B: 53+, C+: 45+, C: 37+, D+: 29+, D: <29
export function getTierFromScore(score: number): (typeof TIER_GRADES)[number] {
  if (score >= 93) return TIER_GRADES[0];
  if (score >= 85) return TIER_GRADES[1];
  if (score >= 77) return TIER_GRADES[2];
  if (score >= 69) return TIER_GRADES[3];
  if (score >= 61) return TIER_GRADES[4];
  if (score >= 53) return TIER_GRADES[5];
  if (score >= 45) return TIER_GRADES[6];
  if (score >= 37) return TIER_GRADES[7];
  if (score >= 29) return TIER_GRADES[8];
  return TIER_GRADES[9];
}

// ── B. 도메인 그룹 정의 ────────────────────────────────────
export const DOMAIN_GROUPS: { name: string; houses: number[]; weight: number }[] = [
  { name: "자아·건강",  houses: [1, 6],   weight: 1.5 },
  { name: "재물·수입",  houses: [2, 11],  weight: 1.5 },
  { name: "직업·명예",  houses: [10, 3],  weight: 1.4 },
  { name: "관계·결혼",  houses: [7, 5],   weight: 1.2 },
  { name: "학문·행운",  houses: [4, 9],   weight: 1.2 },
  { name: "변화·영성",  houses: [8, 12],  weight: 0.8 },
];

const RATING_TO_SCORE: Record<string, number> = {
  Excellent: 100,
  Strong: 75,
  Average: 50,
  Weak: 25,
};
const SCORE_TO_TIER = (s: number) => s >= 80 ? "S" : s >= 65 ? "A" : s >= 45 ? "B" : s >= 25 ? "C" : "D";

// ── 공통 유틸 ──────────────────────────────────────────────
const clampScore = (score: number): number => Math.min(100, Math.max(0, score));

// 상위 구간만 압축 (50 이하는 그대로, 50 초과는 x^1.4 압축)
const softNormalize = (score: number): number => {
  const c = clampScore(score);
  if (c <= 50) return c;
  const excess = (c - 50) / 50;
  return Math.round(50 + Math.pow(excess, 1.4) * 35);
};

const BENEFIC_PLANETS = ["Jupiter", "Venus", "Mercury", "Moon"];
const GOOD_12_STAGES = new Set(["장생", "건록", "제왕", "관대", "목욕"]);
const BAD_12_STAGES  = new Set(["절", "묘", "태"]);
const GOOD_RELATIONS_RE = /합|록|귀인|삼합|육합/;
const BAD_RELATIONS_RE  = /충|형|해|파/;
const WEAK_STRUCTURES   = new Set(["Follower", "FollowerResource", "FollowerStrength"]);

// ── A. 사주 점수 (이론 max ~100) ─────────────────────────────
function computeSajuScore(saju: SajuAnalysisResult | null): { score: number; highlights: string[] } {
  if (!saju?.report) return { score: 0, highlights: [] };
  const r = saju.report;
  const st = r.strength;
  const highlights: string[] = [];
  let s = 0;

  // 1. 신강·신약 (max 25)
  s += clampScore((st.strength_score ?? 25) * 2) * 0.25;

  // 2. 득령·득지·득시 (max 10)
  const acquired = [st.deuk_ryeong?.acquired, st.deuk_ji?.acquired, st.deuk_si?.acquired].filter(Boolean).length;
  s += acquired * (10 / 3);
  if (acquired === 3) highlights.push("삼득(득령·득지·득시) 완성");
  else if (acquired >= 2) highlights.push(`득령·득지·득시 ${acquired}개 달성`);

  // 3. 득세 지지비율 (max 8)
  const rawRatio = st.deuk_se?.support_ratio ?? 0;
  const supportPct = rawRatio > 1 ? rawRatio : rawRatio * 100;
  s += (supportPct / 100) * 8;
  if (supportPct > 60) highlights.push(`득세 지지비율 ${supportPct.toFixed(0)}% (우세)`);

  // 4. 오행 흐름 (max 10)
  const throughput = saju.qi_topology?.throughput ?? 0;
  s += throughput * 10;
  if (throughput > 0.75) highlights.push(`오행 흐름 ${(throughput * 100).toFixed(0)}% (원활)`);
  if (saju.qi_topology?.bottleneck) s -= 2; // 병목 패널티

  // 5. 격국 (±3)
  const structure = r.structure?.structure ?? "";
  if (WEAK_STRUCTURES.has(structure)) s -= 3;
  else if (structure) s += 2;

  // 6. 신살 길/흉 (max ±8)
  const auspicious   = r.spirit_markers?.auspicious?.length ?? 0;
  const inauspicious = r.spirit_markers?.inauspicious?.length ?? 0;
  s += Math.min(5, auspicious * 1);
  s -= Math.min(4, inauspicious * 0.8);
  if (auspicious > 0) highlights.push(`길신 ${auspicious}개`);
  if (inauspicious >= 3) highlights.push(`흉신 ${inauspicious}개 주의`);

  // 7. 골든타임 (max 6)
  const gt = r.golden_time;
  if (gt) {
    const len = gt.end_age - gt.start_age;
    s += Math.min(6, len * 0.4);
    highlights.push(`골든타임 ${gt.start_age}~${gt.end_age}세 (${len}년)`);
  }

  // 8. 시뮬레이션 (max 12)
  const frames = r.simulation_frames ?? [];
  if (frames.length > 0) {
    const avg = frames.reduce((acc, f) => acc + (f.score ?? 50), 0) / frames.length;
    const goodPct = frames.filter(f => (f.score ?? 0) >= 65).length / frames.length;
    s += Math.min(7, (avg - 50) * 0.14);
    s += goodPct * 5;
    if (avg >= 70) highlights.push(`시뮬레이션 평균 ${avg.toFixed(0)}점 (우수)`);
  }

  // 9. 취약점 (max ±5)
  const vulnTotal = saju.vulnerability_report?.total_crashes ?? 0;
  if (vulnTotal === 0 && saju.vulnerability_report) { s += 4; highlights.push("충돌 주의 시점 없음"); }
  else if (vulnTotal > 40) { s -= 5; highlights.push(`주의 시점 ${vulnTotal}개`); }
  else if (vulnTotal > 20) s -= 2;

  // 10. 안정성 등급 (±4)
  const stability = saju.complexity?.stability_grade ?? "";
  const entropy   = saju.entropy?.score ?? 1.0;
  if (/^A/.test(stability)) { s += 4; highlights.push("안정성 A등급"); }
  else if (/^B/.test(stability)) s += 2;
  else if (/^D/.test(stability)) s -= (entropy > 1.5 ? 1.5 : 3);

  // 11. 린트 (±5)
  const lints = saju.lints ?? [];
  const errCnt  = lints.filter(l => l.severity === "Error").length;
  const warnCnt = lints.filter(l => l.severity === "Warning").length;
  if (errCnt === 0 && warnCnt === 0) { s += 2; highlights.push("사주 구조 클린"); }
  s -= Math.min(5, errCnt * 1.5 + warnCnt * 0.4);

  // 12. 엔트로피 (±2)
  if (entropy < 1.0) { s += 2; highlights.push("운명 패턴 안정"); }
  else if (entropy > 2.0) s -= 2;

  // 13. 용신 명확도 (+2)
  if ((r.yongshin?.recommendations?.length ?? 0) >= 2) s += 2;

  return { score: clampScore(Math.round(s)), highlights: highlights.slice(0, 6) };
}

// ── A. 베딕 점수 (이론 max ~100) ─────────────────────────────
function computeVedicScore(report: VedicAnalysisResult | null): { score: number; highlights: string[] } {
  if (!report?.report) return { score: 0, highlights: [] };
  const r = report.report;
  const chart = report.chart;
  const highlights: string[] = [];
  let s = 0;

  // 1. 전체 강도 (max 35)
  s += clampScore((r.overall_strength_score ?? 0) / 6) * 0.35;

  // 2. 요가 (max 12)
  let veryHigh = 0, high = 0;
  for (const y of (r.yogas ?? []) as { quality: string | object }[]) {
    const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
    if (q === "VeryHigh") veryHigh++;
    else if (q === "High") high++;
  }
  s += Math.min(12, veryHigh * 4 + high * 2);
  if (veryHigh > 0) highlights.push(`최상급 요가 ${veryHigh}개`);
  else if (high > 0) highlights.push(`우수 요가 ${high}개`);

  // 3. 하우스 강도 (max 10)
  const summary = (r.house_summary ?? []) as { rating: string }[];
  const excellentH = summary.filter(h => h.rating === "Excellent").length;
  const strongH    = summary.filter(h => h.rating === "Strong").length;
  const weakH      = summary.filter(h => h.rating === "Weak").length;
  s += Math.min(10, excellentH * 2 + strongH * 0.8);
  s -= Math.min(6, weakH * 1.2);
  if (excellentH >= 4) highlights.push(`최강 하우스 ${excellentH}개`);
  else if (excellentH + strongH >= 6) highlights.push(`강한 하우스 ${excellentH + strongH}개`);

  // 4. 핵심 하우스 1·5·9·10 (max 6)
  const bhava = (chart?.bhava_strengths ?? []) as { house: number; total_score: number }[];
  let keyBonus = 0;
  for (const h of [1, 5, 9, 10]) {
    const b = bhava.find(b => b.house === h);
    if ((b?.total_score ?? 0) >= 60) keyBonus += 1.5;
    else if ((b?.total_score ?? 0) >= 40) keyBonus += 0.5;
  }
  s += Math.min(6, keyBonus);
  if (keyBonus >= 4) highlights.push("핵심 하우스(1·5·9·10) 강화");

  // 5. 사데사티 (±6)
  if (r.sade_sati === "None")    { s += 5; highlights.push("사데사티 비해당"); }
  else if (r.sade_sati === "Peak")    { s -= 6; highlights.push("사데사티 절정 진행 중"); }
  else if (r.sade_sati === "Rising")  { s -= 3; highlights.push("사데사티 상승 진행 중"); }
  else if (r.sade_sati === "Setting") s -= 1;

  // 6. 현재 다샤 (max 4)
  const dasha = r.dasha_focus ?? "";
  if (BENEFIC_PLANETS.some(p => dasha.includes(p))) { s += 4; highlights.push(`길성 다샤 (${dasha.split(" ")[0]})`); }

  // 7. SAV (max 5)
  const savPts = chart?.sav?.points ?? [];
  if (Array.isArray(savPts) && savPts.length === 12) {
    let strong = 0, weak = 0;
    for (const p of savPts as number[]) {
      if (p >= 28) strong++;
      else if (p <= 22) weak++;
    }
    s += Math.min(5, strong * 0.8);
    s -= Math.min(4, weak * 0.8);
    if (strong >= 6) highlights.push(`SAV 강점 하우스 ${strong}개`);
  }

  // 8. 빔쇼파카 (max 6)
  const vim = chart?.vimshopaka_scores ?? [];
  if (vim.length > 0) {
    const avg = vim.reduce((acc: number, [, v]: [string, { shadvarga_score: number; shodashavarga_score: number }]) =>
      acc + (v.shadvarga_score + v.shodashavarga_score) / 2, 0) / vim.length;
    s += Math.min(6, (avg / 20) * 6);
    if (avg >= 14) highlights.push(`빔쇼파카 평균 ${avg.toFixed(1)} (강함)`);
  }

  // 9. 역행·연소 (±4)
  const planets = (chart?.planets ?? []) as { is_retrograde: boolean; is_combust: boolean }[];
  const retroCnt  = planets.filter(p => p.is_retrograde).length;
  const combustCnt= planets.filter(p => p.is_combust).length;
  s -= Math.min(4, retroCnt * 0.8);
  s -= Math.min(3, combustCnt * 0.8);
  if (retroCnt + combustCnt >= 3) highlights.push(`역행 ${retroCnt}+연소 ${combustCnt}개 (약화)`);

  // 10. 아바스타 (±3)
  const avasthas = (chart?.avasthas ?? []) as { baladi: string }[];
  if (avasthas.length > 0) {
    const bala = avasthas.filter(a => a.baladi === "Bala" || a.baladi === "Yuva").length;
    const mrta = avasthas.filter(a => a.baladi === "Mrita" || a.baladi === "Vridha").length;
    s += Math.min(3, bala * 0.6);
    s -= Math.min(3, mrta * 0.7);
    if (bala >= 4) highlights.push(`활성 행성 ${bala}개 (Bala·Yuva)`);
  }

  return { score: clampScore(Math.round(s)), highlights: highlights.slice(0, 6) };
}

// ── A. 트랜짓 점수 (댐핑 적용, 변동폭 ±20) ───────────────────
function computeTransitScore(transit: TransitResult | null | undefined): { score: number; highlights: string[] } {
  if (!transit) return { score: 0, highlights: [] };
  const highlights: string[] = [];
  const frame = transit.current_frame;
  const nearby = (transit.nearby_diagnostics ?? []) as { status: string }[];

  // v3: 원국 중심값(50) ± 20 범위로 댐핑
  const rawFrame = frame?.score != null ? clampScore(frame.score) : 50;
  const delta = rawFrame - 50;
  const dampedDelta = Math.sign(delta) * Math.min(Math.abs(delta), 20);
  let s = 50 + dampedDelta;

  // 근처 부하 진단
  let badCnt = 0, downCnt = 0;
  for (const d of nearby) {
    if (d.status === "SystemDown") { badCnt++; downCnt++; }
    else if (d.status === "Overloaded") badCnt++;
  }
  if (badCnt > 0) { s -= Math.min(12, badCnt * 4); highlights.push(`근처 부하 구간 ${badCnt}개`); }
  if (downCnt > 0) s -= downCnt * 2;

  // 현재 점수 라벨
  if (s >= 65) highlights.push("현재 운세 긍정적 (맑음)");
  else if (s >= 50) highlights.push("현재 운세 보통 (구름)");
  else if (s < 40)  highlights.push("현재 운세 주의 필요");

  // 세운 12운성
  const yStage = transit.yearly_luck?.twelve_stage ?? "";
  if (GOOD_12_STAGES.has(yStage)) { s += 5; highlights.push(`세운 12운성: ${yStage} (길)`); }
  else if (BAD_12_STAGES.has(yStage)) { s -= 5; highlights.push(`세운 12운성: ${yStage} (흉)`); }

  // 월운 12운성
  const mStage = transit.monthly_luck?.twelve_stage ?? "";
  if (GOOD_12_STAGES.has(mStage)) s += 3;
  else if (BAD_12_STAGES.has(mStage)) s -= 3;

  // 세운 특이 이벤트
  const events = transit.yearly_luck?.special_events ?? [];
  if (events.length > 0) { s += Math.min(4, events.length * 1.5); highlights.push(`길조 이벤트 ${events.length}개`); }

  // 원국 관계
  const relations = (transit.yearly_luck?.influence?.relations_with_natal ?? []) as string[];
  let goodRel = 0, badRel = 0;
  for (const rel of relations) {
    if (GOOD_RELATIONS_RE.test(rel)) goodRel++;
    if (BAD_RELATIONS_RE.test(rel)) badRel++;
  }
  s += Math.min(4, goodRel * 1.5);
  s -= Math.min(4, badRel * 1.5);

  return { score: clampScore(Math.round(s)), highlights: highlights.slice(0, 4) };
}

// ── 잠재력 점수 ───────────────────────────────────────────────
function computePotentialScore(
  saju: SajuAnalysisResult | null,
  report: VedicAnalysisResult | null,
): { score: number; highlights: string[] } {
  const highlights: string[] = [];
  let s = 50;

  if (saju?.report) {
    const throughput = saju.qi_topology?.throughput ?? 0;
    s += throughput * 12;
    if (throughput > 0.7) highlights.push("오행 흐름 원활 (잠재력 발현 유리)");

    const stability = saju.complexity?.stability_grade ?? "";
    if (/^A/.test(stability)) { s += 8; highlights.push("안정성 A등급 (성장 기반 탄탄)"); }
    else if (/^B/.test(stability)) s += 4;
    else if (/^D/.test(stability)) s -= 4;

    const gt = saju.report.golden_time;
    if (gt) {
      const len = gt.end_age - gt.start_age;
      s += Math.min(12, len * 0.5);
      highlights.push(`골든타임 ${gt.start_age}~${gt.end_age}세`);
    }

    if ((saju.report.yongshin?.recommendations?.length ?? 0) >= 2) { s += 4; highlights.push("용신 방향 명확"); }

    const vulnTotal = saju.vulnerability_report?.total_crashes ?? 99;
    if (vulnTotal === 0) s += 5;
    else if (vulnTotal < 10) s += 3;
    else if (vulnTotal > 40) s -= 5;

    const auspicious = saju.report.spirit_markers?.auspicious?.length ?? 0;
    s += Math.min(5, auspicious * 1.2);
    if (auspicious >= 3) highlights.push(`길신 ${auspicious}개 (잠재력 촉매)`);

    const errCnt = (saju.lints ?? []).filter(l => l.severity === "Error").length;
    if (errCnt === 0) s += 3;
    else s -= errCnt * 2;

    const frames = saju.report.simulation_frames ?? [];
    if (frames.length > 0) {
      const goodPct = frames.filter(f => (f.score ?? 0) >= 70).length / frames.length;
      s += goodPct * 8;
      if (goodPct >= 0.5) highlights.push(`우호 구간 ${(goodPct * 100).toFixed(0)}% (절반 이상 순풍)`);
    }
  }

  if (report?.report) {
    const yogas = (report.report.yogas ?? []).filter((y: { quality: string | object }) => {
      const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
      return q === "VeryHigh" || q === "High";
    }).length;
    s += Math.min(10, yogas * 3);
    if (yogas > 0) highlights.push(`상급 요가 ${yogas}개 (잠재력 극대화)`);

    const excH = ((report.report.house_summary ?? []) as { rating: string }[]).filter(h => h.rating === "Excellent").length;
    s += Math.min(8, excH * 2.5);
    if (excH >= 3) highlights.push(`최강 하우스 ${excH}개`);

    if (report.report.sade_sati === "None") s += 3;
    else if (report.report.sade_sati === "Peak") s -= 5;
  }

  return { score: clampScore(Math.round(s)), highlights: highlights.slice(0, 5) };
}

// ── B. 도메인 티어 (6개 생활 영역 그룹) ─────────────────────
function getDomainTiers(report: VedicAnalysisResult | null): { house: number; domain: string; tier: string }[] {
  if (!report?.report?.house_summary?.length) return [];

  const summaryMap = new Map<number, string>();
  for (const h of (report.report.house_summary as { house: number; rating: string }[])) {
    summaryMap.set(h.house, h.rating);
  }

  return DOMAIN_GROUPS.map(group => {
    // 그룹 내 하우스 점수 평균 산출 (weight 반영)
    const scores = group.houses.map(h => RATING_TO_SCORE[summaryMap.get(h) ?? "Average"] ?? 50);
    const avg = scores.reduce((a, b) => a + b, 0) / scores.length;
    const weightedScore = Math.round(avg * group.weight);
    return {
      house: group.houses[0], // 대표 하우스
      domain: group.name,
      tier: SCORE_TO_TIER(Math.min(100, weightedScore)),
    };
  });
}

// ── C. 4축 다차원 리스크 인덱스 ──────────────────────────────
function computeRiskLevel(
  saju: SajuAnalysisResult | null,
  report: VedicAnalysisResult | null,
  transit: TransitResult | null | undefined,
  transitScore: number,
): "low" | "medium" | "high" | "critical" {
  let risk = 0;

  // 축 1: 취약점 — 치명적(0점) 벡터 비율
  if (saju?.vulnerability_report) {
    const total = saju.vulnerability_report.total_crashes ?? 0;
    const fatalVectors = (saju.vulnerability_report.critical_vectors ?? []).filter(v => v.crash_score <= 5).length;
    const fatalRatio = total > 0 ? fatalVectors / total : 0;
    if (fatalRatio > 0.5) risk += 3;       // 절반 이상 치명적
    else if (fatalRatio > 0.25) risk += 2;
    else if (fatalRatio > 0.1) risk += 1;
    if (total >= 30) risk += 1;
    if (total >= 50) risk += 1;
  }

  // 축 2: 대운 교체 — 현재 나이가 대운 시작 나이의 ±2년 이내
  if (transit && saju?.report?.major_luck) {
    const age = transit.current_age ?? -1;
    const majors = saju.report.major_luck.cycles ?? [];
    const isNearTransition = majors.some(m => Math.abs(age - m.start_age) <= 2);
    if (isNearTransition) risk += 2; // 대운 교체 직후는 불안정
  }

  // 축 3: 베딩 압박 — 사데사티 + 역행·연소
  if (report?.report) {
    if (report.report.sade_sati === "Peak") risk += 3;
    else if (report.report.sade_sati === "Rising") risk += 2;
    const planets = (report.chart?.planets ?? []) as { is_retrograde: boolean; is_combust: boolean }[];
    if (planets.filter(p => p.is_retrograde).length >= 3) risk += 1;
  }

  // 축 4: 현재운 — 트랜짓 점수 + 부하 진단
  if (transitScore < 35) risk += 2;
  else if (transitScore < 45) risk += 1;
  const nearbyDown = (transit?.nearby_diagnostics ?? [] as { status: string }[])
    .filter((d: { status: string }) => d.status === "SystemDown").length;
  if (nearbyDown >= 2) risk += 2;
  else if (nearbyDown >= 1) risk += 1;

  if (risk >= 8) return "critical";
  if (risk >= 5) return "high";
  if (risk >= 3) return "medium";
  return "low";
}

// ── 메인 함수 ──────────────────────────────────────────────────
export function computeTierResult(
  sajuReport: SajuAnalysisResult | null,
  report: VedicAnalysisResult | null,
  transitReport?: TransitResult | null,
): TierResult | null {
  const hasSaju = !!sajuReport?.report;
  const hasVedic = !!report?.report;
  if (!hasSaju && !hasVedic) return null;

  const sajuResult    = computeSajuScore(sajuReport);
  const vedicResult   = computeVedicScore(report);
  const transitResult = computeTransitScore(transitReport);
  const potentialResult = computePotentialScore(sajuReport, report);
  const domainTiers   = getDomainTiers(report);

  // 원국 점수 합산
  const natalScoreRaw = hasSaju && hasVedic
    ? (sajuResult.score * 0.5 + vedicResult.score * 0.5)
    : hasSaju ? sajuResult.score : vedicResult.score;

  // 나이 프로파일 가중치
  let natalWeight = 0.7, currentWeight = 0.3;
  let profile: TierResult["profile"] = "balanced";
  const age = transitReport?.current_age;
  if (typeof age === "number") {
    if (age < 35)      { natalWeight = 0.6; currentWeight = 0.4; profile = "growth"; }
    else if (age > 55) { natalWeight = 0.8; currentWeight = 0.2; profile = "stable"; }
  }

  const hasTransit = !!transitReport?.current_frame;
  let currentScore = transitResult.score;
  // 극단값 완화 캡
  if (hasTransit) {
    currentScore = Math.max(25, Math.min(90, currentScore));
  }

  const natalNorm   = softNormalize(natalScoreRaw);
  const currentNorm = hasTransit ? softNormalize(currentScore) : 0;

  let destinyScore = hasTransit
    ? natalNorm * natalWeight + currentNorm * currentWeight
    : natalNorm;

  // B. 도메인 그룹 기반 보정 (핵심 도메인 약세 시 추가 패널티)
  if (domainTiers.length > 0) {
    let adj = 0;
    for (const d of domainTiers) {
      const group = DOMAIN_GROUPS.find(g => g.name === d.domain);
      const w = group?.weight ?? 1.0;
      if (d.tier === "S") adj += 0.5 * w;
      else if (d.tier === "A") adj += 0.2 * w;
      else if (d.tier === "C") adj -= 0.5 * w;
      else if (d.tier === "D") adj -= 1.0 * w;
    }
    destinyScore += Math.max(-5, Math.min(6, adj));
  }

  destinyScore = clampScore(destinyScore);
  const destinyTier   = getTierFromScore(destinyScore);
  const potentialTier = getTierFromScore(potentialResult.score);

  // 강점 / 약점
  const strengths = [
    ...sajuResult.highlights.slice(0, 2),
    ...vedicResult.highlights.slice(0, 2),
    ...transitResult.highlights.filter(h => !h.includes("주의") && !h.includes("부하")).slice(0, 1),
  ].filter(Boolean).slice(0, 5);

  const weaknesses: string[] = [];
  const vulnTotal = sajuReport?.vulnerability_report?.total_crashes ?? 0;
  if (vulnTotal > 30) weaknesses.push(`주의 시점 ${vulnTotal}개 (위험 구간 다수)`);
  else if (vulnTotal > 0) weaknesses.push(`주의 시점 ${vulnTotal}개`);
  if (report?.report?.sade_sati === "Peak")  weaknesses.push("사데사티 절정 — 토성 압박 최고조");
  else if (report?.report?.sade_sati === "Rising") weaknesses.push("사데사티 상승 진행 중");
  const inauspicious = sajuReport?.report?.spirit_markers?.inauspicious?.length ?? 0;
  if (inauspicious >= 3) weaknesses.push(`흉신 ${inauspicious}개 (겁살·망신살 등)`);
  const errLints = (sajuReport?.lints ?? []).filter(l => l.severity === "Error");
  if (errLints.length > 0) weaknesses.push(`사주 구조 오류 ${errLints.length}개`);
  const badTransit = transitResult.highlights.filter(h => h.includes("주의") || h.includes("부하") || h.includes("흉"));
  if (badTransit.length > 0) weaknesses.push(badTransit[0]);

  // C. 4축 리스크 레벨
  const riskLevel = computeRiskLevel(sajuReport, report, transitReport, transitResult.score);

  // 갭 (잠재력 - 운명)
  const growthGap = clampScore(potentialResult.score) - destinyScore;

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
    version: "v3",
  };
}
