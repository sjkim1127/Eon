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

// S+: 93+, S: 85+, A+: 77+, A: 69+, B+: 61+, B: 53+, C+: 45+, C: 37+, D+: 29+, D: <29
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

/**
 * Deterministic Spread Normalization
 * 양 끝단으로 점수를 더 밀어내어 변별력을 강화 (v3 핵심 알고리즘)
 */
export function spreadNormalize(score: number): number {
  const s = Math.min(100, Math.max(0, score));
  // 50점을 기준으로 Sigmoid-like stretch 적용
  const normalized = s < 50
    ? 50 * Math.pow(s / 50, 1.4) // 아래로 볼록 (C/D/E/F 분산)
    : 50 + 50 * Math.pow((s - 50) / 50, 0.7); // 위로 볼록 (A/S 견인)
  return Math.round(normalized);
}

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
function computeDetailedComponents(
  saju: SajuAnalysisResult | null,
  vedic: VedicAnalysisResult | null,
  _transit: TransitResult | null | undefined,
  transitScore: number
): TierResult["detailedComponents"] {
  const components: TierResult["detailedComponents"] = [];
  const r = saju?.report;

  // 1. 사주 원국 강점 (0.12)
  const sajuStrength = saju?.report?.strength?.strength_score ?? 0;
  const sajuStrengthScore = Math.min(100, Math.abs(sajuStrength) * 2);
  components.push({
    key: "saju_strength",
    label: "사주 원국 강점",
    score: sajuStrengthScore,
    weight: 0.12,
    reasons: [`신강약 지수: ${sajuStrength.toFixed(1)}`],
  });

  // 2. 오행 유통성 (0.08)
  const throughput = saju?.qi_topology?.throughput ?? 0;
  components.push({
    key: "element_flow",
    label: "오행 유통성",
    score: throughput * 100,
    weight: 0.08,
    reasons: [`오행 흐름 효율: ${(throughput * 100).toFixed(1)}%`],
  });

  // 3. 격국 및 용신 (0.10)
  const structure = r?.structure?.structure ?? "";
  const structureScore = structure.includes("Follower") ? 40 : 85;
  components.push({
    key: "structure",
    label: "격국 및 용신",
    score: structureScore,
    weight: 0.10,
    reasons: [`격국: ${structure || "일반격"}`],
  });

  // 4. 길흉신 분포 (0.06)
  const auspicious = r?.spirit_markers?.auspicious?.length ?? 0;
  const inauspicious = r?.spirit_markers?.inauspicious?.length ?? 0;
  const spiritScore = Math.max(0, Math.min(100, auspicious * 20 - inauspicious * 10));
  components.push({
    key: "spirit_markers",
    label: "길흉신 분포",
    score: spiritScore,
    weight: 0.06,
    reasons: [`길신 ${auspicious}개, 흉신 ${inauspicious}개`],
  });

  // 5. 베딕 하우스 역량 (0.12)
  const houseScore = (vedic?.report?.overall_strength_score ?? 0) / 6;
  components.push({
    key: "vedic_houses",
    label: "베딕 하우스 역량",
    score: Math.min(100, houseScore),
    weight: 0.12,
    reasons: [`전체 하우스 평균 강도: ${houseScore.toFixed(1)}`],
  });

  // 6. 행성 조합(요가) (0.10)
  const veryHighYogas = (vedic?.report?.yogas ?? []).filter((y: any) => {
    const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
    return q === "VeryHigh";
  }).length;
  const yogaScore = Math.min(100, veryHighYogas * 30);
  components.push({
    key: "vedic_yogas",
    label: "행성 조합(요가)",
    score: yogaScore,
    weight: 0.10,
    reasons: [`최상급 요가 ${veryHighYogas}개 감지`],
  });

  // 7. 행성 활성도 (0.07)
  const balaYuva = (vedic?.chart?.avasthas ?? []).filter((a: any) => a.baladi === "Bala" || a.baladi === "Yuva").length;
  const avasthaScore = (balaYuva / 7) * 100;
  components.push({
    key: "planet_status",
    label: "행성 활성도",
    score: Math.min(100, avasthaScore),
    weight: 0.07,
    reasons: [`활성 상태(Bala/Yuva) 행성 ${balaYuva}개`],
  });

  // 8. 현재 운 흐름 (0.12)
  components.push({
    key: "luck_cycle",
    label: "현재 운 흐름",
    score: transitScore,
    weight: 0.12,
    reasons: [`트랜짓 종합 점수: ${transitScore.toFixed(1)}`],
  });

  // 9. 인생 안정성 (0.08)
  const crashes = saju?.vulnerability_report?.total_crashes ?? 0;
  const stabilityScore = Math.max(0, 100 - crashes * 2);
  components.push({
    key: "stability",
    label: "인생 안정성",
    score: stabilityScore,
    weight: 0.08,
    reasons: [`취약점(Crash) 지수: ${crashes}`],
  });

  // 10. 발전 가능성 (0.05)
  const entropy = saju?.entropy?.score ?? 1.0;
  components.push({
    key: "potential",
    label: "발전 가능성",
    score: Math.min(100, entropy * 50),
    weight: 0.05,
    reasons: [`운명 엔트로피: ${entropy.toFixed(2)}`],
  });

  // 11. 인생 골든타임 (0.05)
  const goldenTime = r?.golden_time;
  const goldenScore = goldenTime?.average_score ?? 50;
  components.push({
    key: "golden_time",
    label: "인생 골든타임",
    score: goldenScore,
    weight: 0.05,
    reasons: [goldenTime ? `최상의 시기: ${goldenTime.start_age}~${goldenTime.end_age}세` : "골든타임 미정"],
  });

  // 12. 종합적 조화 (0.05)
  const lints = saju?.lints?.length ?? 0;
  const balanceScore = Math.max(0, 100 - lints * 5);
  components.push({
    key: "holistic_balance",
    label: "종합적 조화",
    score: balanceScore,
    weight: 0.05,
    reasons: [`사주 구조 분석 린트 ${lints}건`],
  });

  return components;
}

function computeRiskLevel(
  saju: SajuAnalysisResult | null,
  report: VedicAnalysisResult | null,
  transit: TransitResult | null | undefined,
  transitScore: number,
): "low" | "medium" | "high" | "critical" {
  let risk = 0;
  if (saju?.vulnerability_report) {
    const total = saju.vulnerability_report.total_crashes ?? 0;
    if (total >= 30) risk += 2;
    if (total >= 50) risk += 2;
  }
  if (transit && saju?.report?.major_luck) {
    const age = transit.current_age ?? -1;
    const majors = saju.report.major_luck.cycles ?? [];
    if (majors.some(m => Math.abs(age - m.start_age) <= 2)) risk += 2;
  }
  if (report?.report) {
    if (report.report.sade_sati === "Peak") risk += 3;
    const planets = (report.chart?.planets ?? []) as any[];
    if (planets.filter(p => p.is_retrograde).length >= 3) risk += 1;
  }
  if (transitScore < 35) risk += 2;
  return risk >= 8 ? "critical" : risk >= 5 ? "high" : risk >= 3 ? "medium" : "low";
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

  // 나이 프로파일 가중치
  let profile = "balanced";
  const age = transitReport?.current_age;
  if (typeof age === "number") {
    if (age < 35)      { profile = "growth"; }
    else if (age > 55) { profile = "stable"; }
  }

  // v3 정밀 분석 컴포넌트
  const detailedComponents = computeDetailedComponents(sajuReport, report, transitReport, transitResult.score);
  
  // v3 핵심 계산: 12개 컴포넌트 가중합
  const destinyRawScore = detailedComponents.reduce((acc, c) => acc + c.score * c.weight, 0);
  
  // B. 도메인 그룹 기반 보정 (핵심 도메인 약세 시 추가 패널티)
  let destinyScore = spreadNormalize(destinyRawScore);
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

  const potentialScore = spreadNormalize(potentialResult.score);
  const potentialTier = getTierFromScore(potentialScore);

  // 갭 (잠재력 - 운명)
  const growthGap = potentialScore - destinyScore;

  return {
    natalScore: Math.round(destinyScore), // natalScore now follows destiny tier score for consistency
    currentScore: transitResult.score,
    destinyScore,
    destinyTier,
    potentialScore,
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
    version: "v3_spread_model",
    destinyRawScore,
    destinyTierScore: destinyScore,
    detailedComponents,
    tierModelVersion: "3.0.0",
  };
}
