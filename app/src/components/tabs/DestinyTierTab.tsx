import { motion } from "framer-motion";
import {
  Trophy, Activity, Star, Sparkles, Target, TrendingUp, Zap,
  ChevronRight, AlertTriangle, Shield, Flame, Loader2
} from "lucide-react";
import {
  RadarChart, Radar, PolarGrid, PolarAngleAxis, PolarRadiusAxis,
  ResponsiveContainer, Tooltip
} from "recharts";
import type { SajuAnalysisResult } from "../../types";
import type { VedicAnalysisResult } from "../../types";
import type { TransitResult } from "../../types";
import { computeTierResult, type TierResult } from "../../utils/tierScore";

// ── 상수 ────────────────────────────────────────────────────────────────────

const TIER_GRADES_UI = [
  { grade: "S+", label: "천기",    color: "from-orange-300 to-amber-500",    desc: "극희귀 최상의 조합" },
  { grade: "S",  label: "천운",    color: "from-amber-400 to-yellow-600",    desc: "사주와 별운이 모두 유리한 극상의 조합" },
  { grade: "A+", label: "대길상",  color: "from-lime-400 to-emerald-500",    desc: "용신·대운·요가가 거의 완벽하게 지원하는 강운" },
  { grade: "A",  label: "대길",    color: "from-emerald-400 to-green-600",   desc: "전반적으로 아주 강한 기운의 조합" },
  { grade: "B+", label: "길상",    color: "from-sky-400 to-celestial-cyan",  desc: "균형이 잡히고 강점이 뚜렷하게 빛나는 운세" },
  { grade: "B",  label: "길",      color: "from-celestial-cyan to-indigo-500", desc: "전반적으로 안정적이고 활용 가능한 운세" },
  { grade: "C+", label: "중상",    color: "from-violet-400 to-purple-500",   desc: "보통 이상의 기운, 노력으로 충분히 도약 가능" },
  { grade: "C",  label: "중평",    color: "from-slate-400 to-slate-600",     desc: "일부 어려움이 있으나 극복 가능한 조합" },
  { grade: "D+", label: "성장예비", color: "from-orange-400 to-rose-500",    desc: "성장 여지가 많으며 빠른 상향 가능" },
  { grade: "D",  label: "다다익선", color: "from-rose-400 to-pink-600",      desc: "성장 여지가 많은 시기, 주의 시점 활용 권장" },
] as const;

const PROFILE_META: Record<string, { icon: string; label: string; color: string }> = {
  growth:   { icon: "🌱", label: "성장형",  color: "bg-emerald-500/20 text-emerald-300 border-emerald-500/40" },
  balanced: { icon: "⚖️", label: "균형형",  color: "bg-celestial-cyan/20 text-celestial-cyan border-celestial-cyan/40" },
  stable:   { icon: "🏛️", label: "안정형",  color: "bg-slate-500/20 text-slate-300 border-slate-500/40" },
};

const RISK_META: Record<string, { icon: string; label: string; color: string; bg: string }> = {
  low:    { icon: "🟢", label: "리스크 낮음", color: "text-emerald-300", bg: "bg-emerald-500/15 border-emerald-500/30" },
  medium: { icon: "🟡", label: "리스크 보통", color: "text-amber-300",   bg: "bg-amber-500/15 border-amber-500/30"   },
  high:   { icon: "🔴", label: "리스크 높음", color: "text-rose-300",    bg: "bg-rose-500/15 border-rose-500/30"     },
};

const TIER_SCORE_MAP: Record<string, number> = {
  "S+": 10, S: 9, "A+": 8, A: 7, "B+": 6, B: 5, "C+": 4, C: 3, "D+": 2, D: 1,
};

// ── 헬퍼 함수 ───────────────────────────────────────────────────────────────

/** TierResult 전체를 받아 다층 인사이트 문단 배열을 생성 */
function buildInsightBlocks(result: TierResult): { title: string; icon: string; text: string; color: string }[] {
  const { destinyTier, potentialTier, profile, riskLevel, growthGap,
    strengths, weaknesses, domainTiers, sajuResult, vedicResult, transitResult,
    natalScore, currentScore, destinyScore } = result;
  const grade = destinyTier.grade;
  const blocks: { title: string; icon: string; text: string; color: string }[] = [];

  // ── 1. 종합 판정 ──────────────────────────────────
  // S+/S는 같은 계열, + 접두 제거로 base 키 참조
  const base = grade.replace("+", "") as string;
  const baseMap: Record<string, string> = {
    "S+": `사주와 별운이 완전히 일치하는 극희귀 최상의 조합입니다(${Math.round(destinyScore)}점). 모든 조건이 이상적으로 결합된 천기(天機) 수준의 운세입니다.`,
    S:  `사주와 별운이 서로 보완하며 극상의 기운을 이룹니다(${Math.round(destinyScore)}점). 대부분의 조건이 이상적으로 결합된 희귀한 조합입니다.`,
    "A+": `용신·대운·요가가 거의 완벽하게 지원하는 강한 차트입니다(${Math.round(destinyScore)}점). 적극적인 도전과 확장이 결실을 맺기 매우 좋은 환경입니다.`,
    A:  `전반적으로 매우 강한 차트입니다(${Math.round(destinyScore)}점). 용신·대운·요가가 유리하게 맞물리는 시기에 적극적인 도전이 빛납니다.`,
    "B+": `균형이 잡혀 있고 강점이 뚜렷하게 빛나는 운세입니다(${Math.round(destinyScore)}점). 강점 분야를 주력으로 삼으면 기대 이상의 결과를 낼 수 있습니다.`,
    B:  `전반적으로 안정적이고 활용 가능한 운세입니다(${Math.round(destinyScore)}점). 강점을 살리고 주의 시점을 사전에 파악해 보완하면 좋은 결과를 기대할 수 있습니다.`,
    "C+": `보통 이상의 기운으로 노력에 따라 충분히 도약 가능합니다(${Math.round(destinyScore)}점). 골든타임·용신 방향을 정확하게 파악하고 실행하는 것이 키포인트입니다.`,
    C:  `일부 어려운 구간이 있으나 충분히 극복 가능합니다(${Math.round(destinyScore)}점). 주의 시점과 골든타임·대운 흐름을 함께 참고하세요.`,
    "D+": `성장 여지가 많으며 조건이 갖춰지면 빠른 상향이 가능합니다(${Math.round(destinyScore)}점). 지금은 기반을 다지고 골든타임을 기다리는 준비 단계입니다.`,
    D:  `성장 여지가 많은 시기입니다(${Math.round(destinyScore)}점). 주의 구간을 피하고 용신·요가가 도와주는 구간을 집중 활용하면 큰 변화를 만들 수 있습니다.`,
  };
  blocks.push({ title: "종합 판정", icon: "🏆", text: baseMap[grade] ?? baseMap[base] ?? "", color: "text-celestial-gold" });

  // ── 2. 원국 vs 현재 운세 비교 ────────────────────
  const diffText = (() => {
    const diff = currentScore - natalScore;
    if (Math.abs(diff) < 5) return `원국(${Math.round(natalScore)}점)과 현재 운세(${Math.round(currentScore)}점)가 거의 일치합니다. 타고난 흐름 그대로 안정적으로 진행 중입니다.`;
    if (diff > 15) return `현재 운세(${Math.round(currentScore)}점)가 원국(${Math.round(natalScore)}점)보다 현저히 높습니다. 지금이 바로 행동해야 할 최적의 타이밍입니다.`;
    if (diff > 0) return `현재 운세(${Math.round(currentScore)}점)가 원국(${Math.round(natalScore)}점)보다 소폭 우세합니다. 적극적인 실행이 효과적인 시기입니다.`;
    if (diff < -15) return `현재 운세(${Math.round(currentScore)}점)가 원국(${Math.round(natalScore)}점)보다 눈에 띄게 낮습니다. 리스크를 줄이고 내실을 다지는 수성(守城) 전략을 권합니다.`;
    return `현재 운세(${Math.round(currentScore)}점)가 원국(${Math.round(natalScore)}점)보다 소폭 낮습니다. 과도한 확장보다 준비와 기반 강화에 집중하세요.`;
  })();
  blocks.push({ title: "원국 vs 현재 운세", icon: "⚖️", text: diffText, color: "text-celestial-cyan" });

  // ── 3. 프로필 기반 조언 ──────────────────────────
  const profileMap: Record<string, string> = {
    growth: "현재 성장 가속 구간(35세 미만)에 있습니다. 용신 오행과 연계된 색상·방위·직업군을 환경에 적극 반영하고, 골든타임 내 학습·도전·투자를 집중하세요.",
    stable: "안정형 원국(56세 이상)으로 타고난 흐름이 흔들리지 않습니다. 새로운 모험보다 리스크 관리와 꾸준한 실천, 후진 양성에 집중하면 더욱 탄탄한 기반을 만들 수 있습니다.",
    balanced: "원국과 현재 운세가 균형을 이루고 있습니다. 강점 분야를 적극 공략하면서도 약점 구간의 방어를 소홀히 하지 마세요.",
  };
  blocks.push({ title: "운세 프로필", icon: profile === "growth" ? "🌱" : profile === "stable" ? "🏛️" : "⚖️", text: profileMap[profile], color: "text-emerald-300" });

  // ── 4. 강점 요약 ──────────────────────────────────
  if (strengths.length > 0) {
    const strengthText = strengths.map(s => `· ${s}`).join("  ") +
      "  이 요소들이 운명 티어를 지탱하는 핵심 기둥입니다.";
    blocks.push({ title: "핵심 강점", icon: "✨", text: strengthText, color: "text-amber-300" });
  }

  // ── 5. 약점 & 리스크 ─────────────────────────────
  const riskMap: Record<string, string> = {
    high: "⚠️ 현재 리스크 요소가 복합적으로 집중된 구간입니다. 주의 시점 탭의 대운·세운 충돌 지점을 반드시 사전에 파악하고, 중요한 계약·투자·이직 결정을 신중히 내리세요.",
    medium: "일부 주의가 필요구간이 있습니다. 역량 탭의 부하 시점을 확인하고, 체력 관리와 재정 안전망을 점검하세요.",
    low: "리스크 요인이 적어 안정적인 환경입니다. 이 여유를 기반 강화와 장기 포트폴리오 구축에 활용하세요.",
  };
  const weaknessPart = weaknesses.length > 0
    ? `  주요 약점: ${weaknesses.join(", ")}.`
    : "";
  blocks.push({ title: "리스크 & 약점", icon: riskLevel === "high" ? "🔴" : riskLevel === "medium" ? "🟡" : "🟢", text: riskMap[riskLevel] + weaknessPart, color: riskLevel === "high" ? "text-rose-300" : riskLevel === "medium" ? "text-amber-300" : "text-emerald-300" });

  // ── 6. 잠재력 격차 조언 ───────────────────────────
  const potGrade = potentialTier.grade;
  const potText = (() => {
    if (growthGap > 20) return `잠재력 티어(${potGrade})가 운명 티어(${grade})보다 +${Math.round(growthGap)}pt 크게 앞섭니다. 용신 오행 강화, 우수 요가 활성화, 골든타임 집중 활용으로 단기간에 큰 상향이 가능합니다.`;
    if (growthGap > 10) return `잠재력(${potGrade})이 운명 티어(${grade})보다 +${Math.round(growthGap)}pt 앞서 있습니다. 일관된 노력과 주의 시점 회피로 꾸준히 격차를 좁혀가세요.`;
    if (growthGap > 3) return `잠재력(+${Math.round(growthGap)}pt 여유)을 점진적으로 발현 중입니다. 현재 방향을 유지하면 자연스럽게 상향됩니다.`;
    if (growthGap < -5) return `운명 티어(${grade})가 잠재력(${potGrade})보다 앞서 있습니다. 현재의 흐름이 매우 효율적임을 의미합니다.`;
    return "잠재력과 운명 티어가 거의 일치합니다. 현재의 흐름을 잘 유지하고 있습니다.";
  })();
  blocks.push({ title: "잠재력 격차 분석", icon: "🚀", text: potText, color: "text-purple-300" });

  // ── 7. 분야별 집중 공략 ───────────────────────────
  const topDomains = [...domainTiers].filter(d => d.tier === "S" || d.tier === "A").slice(0, 3);
  const weakDomains = [...domainTiers].filter(d => d.tier === "D" || d.tier === "C").slice(0, 3);
  if (topDomains.length > 0 || weakDomains.length > 0) {
    const domainParts: string[] = [];
    if (topDomains.length > 0) domainParts.push(`강점 분야: ${topDomains.map(d => `${d.domain}(${d.tier})`).join(", ")} — 이 영역을 주력 무대로 삼으세요.`);
    if (weakDomains.length > 0) domainParts.push(`보완 분야: ${weakDomains.map(d => `${d.domain}(${d.tier})`).join(", ")} — 과도한 집중보다 방어적 관리를 권합니다.`);
    blocks.push({ title: "분야별 집중 전략", icon: "🎯", text: domainParts.join("  "), color: "text-sky-300" });
  }

  // ── 8. 사주 vs 베딕 점수 균형 ────────────────────
  const sajuS = sajuResult.score;
  const vedicS = vedicResult.score;
  const transitS = transitResult.score;
  const balanceParts: string[] = [];
  if (Math.abs(sajuS - vedicS) > 20) {
    if (sajuS > vedicS) balanceParts.push(`사주 원국(${sajuS}점)이 베딕 차트(${vedicS}점)보다 강합니다. 동양 명리 기반의 판단이 더 정확할 수 있습니다.`);
    else balanceParts.push(`베딕 차트(${vedicS}점)가 사주 원국(${sajuS}점)보다 강합니다. 베딕 요가와 다샤 흐름을 우선 참고하세요.`);
  } else {
    balanceParts.push(`사주(${sajuS}점)·베딕(${vedicS}점) 두 체계가 균형 있게 같은 방향을 가리킵니다. 신뢰도가 높은 분석 결과입니다.`);
  }
  if (transitS >= 70) balanceParts.push(`현재 운세 점수(${transitS}점)가 높아 지금이 행동하기 좋은 시기입니다.`);
  else if (transitS < 40) balanceParts.push(`현재 운세 점수(${transitS}점)가 낮습니다. 중요 결정은 운세 점수가 회복된 후 미루는 것을 권합니다.`);
  blocks.push({ title: "분석 체계 균형", icon: "🔬", text: balanceParts.join("  "), color: "text-indigo-300" });

  return blocks;
}

// ── 점수 바 컴포넌트 ─────────────────────────────────────────────────────────

function ScoreBar({ label, value, max, color }: { label: string; value: number; max: number; color: string }) {
  const pct = Math.min(100, (value / max) * 100);
  return (
    <div className="flex items-center gap-3">
      <span className="text-xs text-white/50 w-36 shrink-0">{label}</span>
      <div className="flex-1 bg-white/10 h-2 rounded-full overflow-hidden">
        <div className={`h-full rounded-full ${color} transition-all duration-500`} style={{ width: `${pct}%` }} />
      </div>
      <span className="text-xs text-white/70 w-8 text-right shrink-0">{Math.round(value)}</span>
    </div>
  );
}

// ── 메인 컴포넌트 ────────────────────────────────────────────────────────────

export interface DestinyTierTabProps {
  sajuReport: SajuAnalysisResult | null;
  report: VedicAnalysisResult | null;
  transitReport?: TransitResult | null;
  unknownTime?: boolean;
}

export { computeTierResult } from "../../utils/tierScore";
export type { TierResult } from "../../utils/tierScore";

export function DestinyTierTab({ sajuReport, report, transitReport, unknownTime }: DestinyTierTabProps) {
  const result = computeTierResult(sajuReport, report, transitReport);

  if (!result) {
    return (
      <motion.div
        key="destiny-tier-empty"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex flex-col items-center justify-center py-24 text-center"
      >
        <Trophy className="w-16 h-16 text-white/20 mb-4" />
        <h3 className="text-xl font-bold text-white mb-2">운명 티어 분석</h3>
        <p className="text-white/50 max-w-sm">
          출생 정보를 입력하고 통합 분석을 실행하면, 사주와 베딕을 종합한 운명·잠재력·분야별 티어를 확인할 수 있습니다.
        </p>
      </motion.div>
    );
  }

  const {
    destinyTier, potentialTier, domainTiers, sajuResult, vedicResult,
    transitResult, strengths, weaknesses, profile, riskLevel, growthGap,
  } = result;

  const hasSaju = !!sajuReport?.report;
  const hasVedic = !!report?.report;
  const hasTransit = !!transitReport?.current_frame;

  const potentialHigher = result.potentialScore > result.destinyScore;
  const tierUi = TIER_GRADES_UI.find((t) => t.grade === destinyTier.grade) ?? TIER_GRADES_UI[TIER_GRADES_UI.length - 1];
  const profileMeta = PROFILE_META[profile] ?? PROFILE_META["balanced"];
  const riskMeta = RISK_META[riskLevel] ?? RISK_META["low"];

  // ── 점수 분해: 사주 ──
  const sajuSt = sajuReport?.report?.strength;
  const strengthNorm = Math.min(100, (sajuSt?.strength_score ?? 0) * 2);
  const rawSupportRatioUi = sajuSt?.deuk_se?.support_ratio ?? 0;
  const supportPctUi = rawSupportRatioUi > 1 ? rawSupportRatioUi : rawSupportRatioUi * 100;
  const deukSum = (sajuSt?.deuk_ryeong?.acquired ? 8 : 0)
    + (sajuSt?.deuk_ji?.acquired ? 8 : 0)
    + (sajuSt?.deuk_si?.acquired ? 6 : 0)
    + (supportPctUi / 100) * 15;
  const throughput = sajuReport?.qi_topology?.throughput ?? 0;
  const goldenTimePt = sajuReport?.report?.golden_time ? 10 : 0;
  const stabilityGrade = sajuReport?.complexity?.stability_grade ?? "";
  const stabilityPt = /^[AB]$/.test(stabilityGrade) || /A|B/.test(stabilityGrade) ? 4 : 0;

  // ── 점수 분해: 베딕 ──
  const vr = report?.report;
  const planetStrengthNorm = Math.min(50, ((vr?.overall_strength_score ?? 0) / 6) * 0.5);
  const highYogaCount = (vr?.yogas ?? []).filter((y: { quality: string | object }) => {
    const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
    return q === "VeryHigh" || q === "High";
  }).length;
  const yogaPt = Math.min(20, highYogaCount * 5);
  const strongHouses = (vr?.house_summary ?? []).filter((h: { rating: string }) =>
    h.rating === "Excellent" || h.rating === "Strong"
  ).length;
  const housePt = (strongHouses / 12) * 20;
  const satiPt = vr?.sade_sati === "None" ? 5 : (vr?.sade_sati === "Peak" || vr?.sade_sati === "Rising" ? 0 : 2.5);
  const BENEFIC_PLANETS = ["Jupiter", "Venus", "Mercury", "Moon"];
  const dashaPt = BENEFIC_PLANETS.some((p) => (vr?.dasha_focus ?? "").includes(p)) ? 4 : 0;

  // ── 도메인 레이더 데이터 ──
  const radarData = domainTiers.map(({ domain, tier }) => ({
    subject: domain.split("·")[0],
    fullMark: 5,
    value: TIER_SCORE_MAP[tier] ?? 1,
  }));

  // ── 트랜싯 점수 색상 ──
  const transitColor =
    transitResult.score >= 60 ? "text-celestial-cyan" :
    transitResult.score >= 40 ? "text-amber-300" : "text-rose-400";
  const transitBorder =
    transitResult.score >= 60 ? "border-celestial-cyan/20" :
    transitResult.score >= 40 ? "border-amber-500/20" : "border-rose-500/20";

  return (
    <motion.div
      key="destiny-tier"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {unknownTime && (
        <div className="flex items-center gap-3 px-5 py-3 rounded-2xl bg-amber-500/10 border border-amber-500/25 text-amber-300 text-sm">
          <span>시주 미상 시 티어 산정은 시 기반 항목을 제외하고 계산됩니다.</span>
        </div>
      )}

      {/* ── 메인 티어 카드 ── */}
      <div className={`glass p-10 rounded-[2.5rem] bg-gradient-to-br ${tierUi.color} bg-opacity-10 border border-white/10`}>
        <h2 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
          <Trophy className="w-8 h-8" />
          당신의 운명의 티어는?
        </h2>
        <div className="flex flex-col md:flex-row items-center gap-8 mt-8">
          <div className="flex flex-col items-center">
            <span className="text-8xl md:text-9xl font-black text-black">
              {destinyTier.grade}
            </span>
            <span className="text-lg font-bold text-white mt-2">{destinyTier.label}</span>
            <p className="text-sm text-white/60 text-center mt-2 max-w-xs">{tierUi.desc}</p>
            {/* ① 프로필 · 리스크 · 성장갭 배지 */}
            <div className="flex flex-wrap justify-center gap-2 mt-4">
              <span className={`inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-semibold border ${profileMeta.color}`}>
                {profileMeta.icon} {profileMeta.label}
              </span>
              <span className={`inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-semibold border ${riskMeta.bg} ${riskMeta.color}`}>
                {riskMeta.icon} {riskMeta.label}
              </span>
              {growthGap > 5 ? (
                <span className="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-semibold border bg-violet-500/20 text-violet-300 border-violet-500/40">
                  <Zap className="w-3 h-3" /> +{Math.round(growthGap)}pt 성장 여지
                </span>
              ) : (
                <span className="inline-flex items-center gap-1 px-2.5 py-1 rounded-full text-xs font-semibold border bg-slate-500/20 text-slate-300 border-slate-500/40">
                  <Shield className="w-3 h-3" /> 최대 활용 중
                </span>
              )}
            </div>
          </div>
          <div className="flex-1 space-y-4">
            <div className="flex items-center gap-2">
              <span className="text-white/60">종합 점수</span>
              <span className="text-2xl font-bold text-white">{Math.round(result.destinyScore)}</span>
              <span className="text-white/40">/ 100</span>
            </div>
            <div className="w-full bg-white/10 h-3 rounded-full overflow-hidden">
              <div
                className={`h-full rounded-full bg-gradient-to-r ${tierUi.color} transition-all duration-700`}
                style={{ width: `${result.destinyScore}%` }}
              />
            </div>
            <div className="flex flex-wrap gap-2 text-sm">
              <span className="text-white/50">원국 {Math.round(result.natalScore)}</span>
              {hasTransit && (
                <span className="text-white/50">· 현재 {Math.round(result.currentScore)}</span>
              )}
              <span className="text-white/30">|</span>
              <span className="font-semibold text-celestial-cyan">
                잠재력 {potentialTier.grade}
              </span>
              {potentialHigher && (
                <span className="text-emerald-400/80 text-xs">(성장 여지 큼)</span>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* ── 운명 / 잠재력 카드 ── */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass p-6 rounded-2xl border border-celestial-purple/20">
          <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
            <Trophy className="w-5 h-5 text-celestial-purple" />
            운명 티어
          </h5>
          <p className="text-2xl font-black text-celestial-purple mb-2">{destinyTier.grade} · {destinyTier.label}</p>
          <p className="text-xs text-white/50">
            {hasTransit ? `원국 ${Math.round(result.natalScore)} · 현재 ${Math.round(result.currentScore)} = 종합 ${Math.round(result.destinyScore)}` : `종합 점수 ${Math.round(result.destinyScore)}`}
          </p>
        </div>
        <div className="glass p-6 rounded-2xl border border-celestial-cyan/20">
          <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
            <Zap className="w-5 h-5 text-celestial-cyan" />
            잠재력 티어
          </h5>
          <p className="text-2xl font-black text-celestial-cyan mb-2">{potentialTier.grade} · {potentialTier.label}</p>
          <p className="text-xs text-white/50">
            {potentialHigher ? "성장 여지가 큽니다. 용신·골든타임·우수 요가를 적극 활용하세요." : "현재 흐름을 잘 활용하고 있습니다."}
          </p>
        </div>
      </div>

      {/* ④ Transit 전용 카드 */}
      {hasTransit && (
        <div className={`glass p-6 rounded-2xl border ${transitBorder}`}>
          <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
            <Loader2 className={`w-5 h-5 ${transitColor}`} />
            현재 운세 흐름
          </h5>
          <div className="flex items-baseline gap-2 mb-3">
            <span className={`text-3xl font-bold ${transitColor}`}>{Math.round(transitResult.score)}</span>
            <span className="text-white/40">/ 100</span>
            <span className={`ml-2 text-xs font-semibold px-2 py-0.5 rounded-full ${
              transitResult.score >= 60 ? "bg-celestial-cyan/15 text-celestial-cyan" :
              transitResult.score >= 40 ? "bg-amber-500/15 text-amber-300" :
              "bg-rose-500/15 text-rose-400"
            }`}>
              {transitResult.score >= 60 ? "긍정적" : transitResult.score >= 40 ? "보통" : "주의"}
            </span>
          </div>
          <div className="w-full bg-white/10 h-2 rounded-full overflow-hidden mb-4">
            <div
              className={`h-full rounded-full transition-all duration-500 ${
                transitResult.score >= 60 ? "bg-celestial-cyan" :
                transitResult.score >= 40 ? "bg-amber-400" : "bg-rose-400"
              }`}
              style={{ width: `${transitResult.score}%` }}
            />
          </div>
          {transitResult.highlights.length > 0 && (
            <ul className="space-y-1.5">
              {transitResult.highlights.map((h, i) => (
                <li key={i} className={`text-sm flex items-center gap-2 ${transitColor}`}>
                  <span className="w-1.5 h-1.5 rounded-full bg-current shrink-0" />
                  {h}
                </li>
              ))}
            </ul>
          )}
        </div>
      )}

      {/* ③ 분야별 티어 — 레이더 차트 */}
      {domainTiers.length > 0 && (
        <div className="glass p-6 rounded-2xl">
          <h5 className="text-lg font-bold text-white mb-1">분야별 티어</h5>
          <p className="text-xs text-white/40 mb-4">베딕 12하우스 강도 기반 (S+=10 · S=9 · A+=8 · A=7 · B+=6 · B=5 · C+=4 · C=3 · D+=2 · D=1)</p>
          <ResponsiveContainer width="100%" height={300}>
            <RadarChart data={radarData} margin={{ top: 10, right: 20, bottom: 10, left: 20 }}>
              <PolarGrid stroke="rgba(255,255,255,0.1)" />
              <PolarAngleAxis
                dataKey="subject"
                tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 11 }}
              />
              <PolarRadiusAxis
                angle={90}
                domain={[0, 10]}
                tick={{ fill: "rgba(255,255,255,0.3)", fontSize: 9 }}
                tickCount={6}
              />
              <Radar
                name="티어"
                dataKey="value"
                stroke="#a78bfa"
                fill="#a78bfa"
                fillOpacity={0.35}
                strokeWidth={2}
              />
              <Tooltip
                contentStyle={{
                  background: "rgba(15,10,30,0.9)",
                  border: "1px solid rgba(167,139,250,0.3)",
                  borderRadius: 12,
                  fontSize: 12,
                }}
                formatter={(val: number) => {
                  const t = Object.entries(TIER_SCORE_MAP).find(([, v]) => v === val)?.[0] ?? "?";
                  return [`${t} (${val})`, "티어"];
                }}
              />
            </RadarChart>
          </ResponsiveContainer>
          {/* 티어 범례 */}
          <div className="flex flex-wrap justify-center gap-1.5 mt-2">
            {[
              { t: "S+", c: "bg-orange-500/30 text-orange-300 border-orange-500/40" },
              { t: "S",  c: "bg-amber-500/30 text-amber-300 border-amber-500/40" },
              { t: "A+", c: "bg-lime-500/30 text-lime-300 border-lime-500/40" },
              { t: "A",  c: "bg-emerald-500/30 text-emerald-300 border-emerald-500/40" },
              { t: "B+", c: "bg-sky-500/30 text-sky-300 border-sky-500/40" },
              { t: "B",  c: "bg-celestial-cyan/20 text-celestial-cyan border-celestial-cyan/40" },
              { t: "C+", c: "bg-violet-500/30 text-violet-300 border-violet-500/40" },
              { t: "C",  c: "bg-slate-500/20 text-slate-300 border-slate-500/40" },
              { t: "D+", c: "bg-orange-500/20 text-orange-400 border-orange-500/30" },
              { t: "D",  c: "bg-rose-500/20 text-rose-400 border-rose-500/40" },
            ].map(({ t, c }) => (
              <span key={t} className={`px-2 py-0.5 rounded text-xs font-bold border ${c}`}>{t}</span>
            ))}
          </div>
          {/* 보조 배지 그리드 */}
          <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-2 mt-4">
            {domainTiers.map(({ house, domain, tier }) => {
              const tierColor =
                tier === "S+" ? "bg-orange-500/15 text-orange-300 border-orange-500/30"
                : tier === "S"  ? "bg-amber-500/15 text-amber-400 border-amber-500/30"
                : tier === "A+" ? "bg-lime-500/15 text-lime-400 border-lime-500/30"
                : tier === "A"  ? "bg-emerald-500/15 text-emerald-400 border-emerald-500/30"
                : tier === "B+" ? "bg-sky-500/15 text-sky-400 border-sky-500/30"
                : tier === "B"  ? "bg-celestial-cyan/15 text-celestial-cyan border-celestial-cyan/30"
                : tier === "C+" ? "bg-violet-500/15 text-violet-400 border-violet-500/30"
                : tier === "C"  ? "bg-slate-500/15 text-slate-300 border-slate-500/30"
                : tier === "D+" ? "bg-orange-500/10 text-orange-400 border-orange-500/20"
                : "bg-rose-500/15 text-rose-400 border-rose-500/30";
              return (
                <div key={house} className={`p-2 rounded-lg border text-center ${tierColor}`}>
                  <p className="text-[9px] text-white/50 mb-0.5 leading-tight">{domain}</p>
                  <p className="text-sm font-black">{tier}</p>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* ② 점수 세부 분해 패널 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {hasSaju && (
          <div className="glass p-6 rounded-2xl border border-celestial-purple/20">
            <h5 className="text-lg font-bold text-white mb-1 flex items-center gap-2">
              <Activity className="w-5 h-5 text-celestial-purple" />
              사주 점수 분해
            </h5>
            <div className="flex items-baseline gap-2 mb-4">
              <span className="text-3xl font-bold text-celestial-purple">{sajuResult.score}</span>
              <span className="text-white/40">/ 100</span>
            </div>
            <div className="space-y-2.5">
              <ScoreBar label="일간 강도 (×35%)" value={strengthNorm * 0.35} max={35} color="bg-gradient-to-r from-violet-500 to-purple-400" />
              <ScoreBar label="4득 합산 (22pt)" value={Math.min(22, deukSum)} max={22} color="bg-gradient-to-r from-purple-400 to-fuchsia-400" />
              <ScoreBar label="오행 흐름 (×25%)" value={throughput * 25} max={25} color="bg-gradient-to-r from-fuchsia-400 to-pink-400" />
              <ScoreBar label="골든타임 (10pt)" value={goldenTimePt} max={10} color="bg-gradient-to-r from-amber-400 to-yellow-400" />
              <ScoreBar label="안정도 보정 (4pt)" value={stabilityPt} max={4} color="bg-gradient-to-r from-emerald-400 to-teal-400" />
            </div>
            {sajuResult.highlights.length > 0 && (
              <ul className="mt-4 space-y-1">
                {sajuResult.highlights.map((h, i) => (
                  <li key={i} className="text-xs text-white/60 flex items-center gap-2">
                    <Sparkles className="w-3 h-3 text-celestial-purple shrink-0" />
                    {h}
                  </li>
                ))}
              </ul>
            )}
          </div>
        )}
        {hasVedic && (
          <div className="glass p-6 rounded-2xl border border-celestial-cyan/20">
            <h5 className="text-lg font-bold text-white mb-1 flex items-center gap-2">
              <Star className="w-5 h-5 text-celestial-cyan" />
              베딕 점수 분해
            </h5>
            <div className="flex items-baseline gap-2 mb-4">
              <span className="text-3xl font-bold text-celestial-cyan">{vedicResult.score}</span>
              <span className="text-white/40">/ 100</span>
            </div>
            <div className="space-y-2.5">
              <ScoreBar label="행성 강도 (×50%)" value={planetStrengthNorm} max={50} color="bg-gradient-to-r from-cyan-500 to-sky-400" />
              <ScoreBar label={`우수 요가 ×5pt (${highYogaCount}개)`} value={yogaPt} max={20} color="bg-gradient-to-r from-sky-400 to-indigo-400" />
              <ScoreBar label={`강한 하우스 ${strongHouses}/12`} value={housePt} max={20} color="bg-gradient-to-r from-indigo-400 to-violet-400" />
              <ScoreBar label="사데사티 (±5pt)" value={satiPt} max={5} color="bg-gradient-to-r from-violet-400 to-purple-400" />
              <ScoreBar label="다사 보너스 (4pt)" value={dashaPt} max={4} color="bg-gradient-to-r from-emerald-400 to-teal-400" />
            </div>
            {vedicResult.highlights.length > 0 && (
              <ul className="mt-4 space-y-1">
                {vedicResult.highlights.map((h, i) => (
                  <li key={i} className="text-xs text-white/60 flex items-center gap-2">
                    <Target className="w-3 h-3 text-celestial-cyan shrink-0" />
                    {h}
                  </li>
                ))}
              </ul>
            )}
          </div>
        )}
      </div>

      {/* 강점·약점 */}
      <div className="glass p-6 rounded-2xl">
        <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
          <TrendingUp className="w-5 h-5 text-celestial-gold" />
          강점 · 약점
        </h5>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div>
            <p className="text-xs text-white/40 font-bold uppercase mb-2">강점</p>
            <ul className="space-y-1.5">
              {strengths.length > 0 ? strengths.map((s, i) => (
                <li key={i} className="text-sm text-emerald-300 flex items-center gap-2">
                  <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 shrink-0" />
                  {s}
                </li>
              )) : (
                <li className="text-sm text-white/40">—</li>
              )}
            </ul>
          </div>
          <div>
            <p className="text-xs text-white/40 font-bold uppercase mb-2">약점 (참고)</p>
            <ul className="space-y-1.5">
              {weaknesses.length > 0 ? weaknesses.map((w, i) => (
                <li key={i} className="text-sm text-amber-300 flex items-center gap-2">
                  <AlertTriangle className="w-3 h-3 text-amber-400 shrink-0" />
                  {w}
                </li>
              )) : (
                <li className="text-sm text-white/40">특이사항 없음</li>
              )}
            </ul>
          </div>
        </div>
        <p className="text-xs text-white/30 mt-4 flex items-center gap-1">
          <ChevronRight className="w-3 h-3" />
          역량 및 기운 탭에서 주의 시점, 현재 운세 탭에서 세운·월운을 확인하세요.
        </p>
      </div>

      {/* ⑤ 종합 인사이트 — 다층 블록 */}
      <div className="glass p-6 rounded-2xl">
        <h5 className="text-lg font-bold text-white mb-5 flex items-center gap-2">
          <Flame className="w-5 h-5 text-celestial-gold" />
          종합 인사이트
        </h5>
        <div className="space-y-4">
          {buildInsightBlocks(result).map((block, i) => (
            <div key={i} className="flex gap-3 p-3.5 rounded-xl bg-white/5 border border-white/8">
              <span className="text-xl shrink-0 mt-0.5">{block.icon}</span>
              <div>
                <p className={`text-xs font-semibold mb-1 ${block.color}`}>{block.title}</p>
                <p className="text-sm text-white/80 leading-relaxed">{block.text}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </motion.div>
  );
}

