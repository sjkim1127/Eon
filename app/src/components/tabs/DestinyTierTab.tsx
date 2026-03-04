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
import { computeTierResult } from "../../utils/tierScore";

// ── 상수 ────────────────────────────────────────────────────────────────────

const TIER_GRADES_UI = [
  { grade: "S", label: "천운", color: "from-amber-400 to-yellow-600", desc: "사주와 별운이 모두 유리한 극상의 조합" },
  { grade: "A", label: "대길", color: "from-emerald-400 to-green-600", desc: "전반적으로 아주 강한 기운의 조합" },
  { grade: "B", label: "길상", color: "from-celestial-cyan to-indigo-500", desc: "균형 잡힌 운세, 적극적으로 활용 가능" },
  { grade: "C", label: "중평", color: "from-slate-400 to-slate-600", desc: "일부 어려움이 있으나 극복 가능한 조합" },
  { grade: "D", label: "다다익선", color: "from-rose-400 to-pink-600", desc: "성장 여지가 많은 시기, 주의 시점 활용 권장" },
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

const TIER_SCORE_MAP: Record<string, number> = { S: 5, A: 4, B: 3, C: 2, D: 1 };

// ── 헬퍼 함수 ───────────────────────────────────────────────────────────────

/** profile / riskLevel / growthGap 기반 동적 인사이트 문장 생성 */
function buildInsightText(
  grade: string,
  profile: "stable" | "balanced" | "growth",
  riskLevel: "low" | "medium" | "high",
  growthGap: number,
): string {
  const baseMap: Record<string, string> = {
    S: "사주와 별운이 서로 보완하며 극상의 기운을 이룹니다.",
    A: "전반적으로 강한 차트입니다. 용신·대운·요가가 유리하게 작용하는 시기에는 더욱 적극적인 도전을 권합니다.",
    B: "균형이 잘 잡힌 운세입니다. 강점을 살리고 약점은 주의 시점 참고를 통해 보완하면 좋은 결과를 기대할 수 있습니다.",
    C: "일부 어려운 시기가 있으나 극복 가능합니다. 역량 및 기운 탭의 주의 시점을 참고하고, 골든타임·대운 흐름을 활용해 보세요.",
    D: "성장 여지가 많은 시기입니다. 주의가 필요한 시기를 피하고, 용신·요가가 도와주는 구간을 적극 활용하시길 권합니다.",
  };
  const sentences: string[] = [baseMap[grade] ?? ""];

  if (profile === "growth") {
    sentences.push("현재 성장 가속 구간에 있습니다. 용신 오행을 강화하고 골든타임을 집중 활용하세요.");
  } else if (profile === "stable") {
    sentences.push("안정적인 원국이 흐름을 뒷받침합니다. 리스크 관리와 꾸준한 실천에 집중하세요.");
  } else {
    sentences.push("원국과 현재 운세가 균형을 이루고 있습니다. 강점 분야를 적극 공략해 보세요.");
  }

  if (riskLevel === "high") {
    sentences.push("⚠️ 현재 리스크 요소가 집중된 구간입니다. 주의 시점 탭을 반드시 확인하고 중요한 결정을 신중히 내리세요.");
  } else if (riskLevel === "medium") {
    sentences.push("일부 주의가 필요한 구간이 있습니다. 역량 탭에서 부하 시점을 확인하세요.");
  }

  if (growthGap > 15) {
    sentences.push(`잠재력과의 격차(+${Math.round(growthGap)}pt)가 크게 열려 있습니다. 용신 강화와 요가 활성화로 빠른 상향이 가능합니다.`);
  } else if (growthGap > 5) {
    sentences.push(`잠재력 여지(+${Math.round(growthGap)}pt)를 꾸준히 좁혀가세요.`);
  } else {
    sentences.push("현재의 흐름을 잘 유지하고 있습니다.");
  }

  return sentences.join(" ");
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
  const deukSum = (sajuSt?.deuk_ryeong?.acquired ? 8 : 0)
    + (sajuSt?.deuk_ji?.acquired ? 8 : 0)
    + (sajuSt?.deuk_si?.acquired ? 6 : 0)
    + ((sajuSt?.deuk_se?.support_ratio ?? 0) * 15);
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
          <p className="text-xs text-white/40 mb-4">베딕 12하우스 강도 기반 (S=5 · A=4 · B=3 · C=2 · D=1)</p>
          <ResponsiveContainer width="100%" height={300}>
            <RadarChart data={radarData} margin={{ top: 10, right: 20, bottom: 10, left: 20 }}>
              <PolarGrid stroke="rgba(255,255,255,0.1)" />
              <PolarAngleAxis
                dataKey="subject"
                tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 11 }}
              />
              <PolarRadiusAxis
                angle={90}
                domain={[0, 5]}
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
          <div className="flex flex-wrap justify-center gap-2 mt-2">
            {[
              { t: "S", c: "bg-amber-500/30 text-amber-300 border-amber-500/40" },
              { t: "A", c: "bg-emerald-500/30 text-emerald-300 border-emerald-500/40" },
              { t: "B", c: "bg-celestial-cyan/20 text-celestial-cyan border-celestial-cyan/40" },
              { t: "C", c: "bg-slate-500/20 text-slate-300 border-slate-500/40" },
              { t: "D", c: "bg-rose-500/20 text-rose-400 border-rose-500/40" },
            ].map(({ t, c }) => (
              <span key={t} className={`px-2 py-0.5 rounded text-xs font-bold border ${c}`}>{t}</span>
            ))}
          </div>
          {/* 보조 배지 그리드 */}
          <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-2 mt-4">
            {domainTiers.map(({ house, domain, tier }) => {
              const tierColor = tier === "S" ? "bg-amber-500/15 text-amber-400 border-amber-500/30"
                : tier === "A" ? "bg-emerald-500/15 text-emerald-400 border-emerald-500/30"
                  : tier === "B" ? "bg-celestial-cyan/15 text-celestial-cyan border-celestial-cyan/30"
                    : tier === "C" ? "bg-slate-500/15 text-slate-300 border-slate-500/30"
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

      {/* ⑤ 종합 인사이트 — 동적 */}
      <div className="glass p-6 rounded-2xl">
        <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
          <Flame className="w-5 h-5 text-celestial-gold" />
          종합 인사이트
        </h5>
        <p className="text-white/80 leading-relaxed">
          {buildInsightText(destinyTier.grade, profile, riskLevel, growthGap)}
        </p>
      </div>
    </motion.div>
  );
}

