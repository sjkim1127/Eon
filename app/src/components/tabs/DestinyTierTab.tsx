import { motion } from "framer-motion";
import { Trophy, Activity, Star, Sparkles, Target, TrendingUp, Zap, ChevronRight } from "lucide-react";
import type { SajuAnalysisResult } from "../../types";
import type { VedicAnalysisResult } from "../../types";
import type { TransitResult } from "../../types";
import { computeTierResult } from "../../utils/tierScore";

const TIER_GRADES_UI = [
  { grade: "S", label: "천운", color: "from-amber-400 to-yellow-600", desc: "사주와 별운이 모두 유리한 극상의 조합" },
  { grade: "A", label: "대길", color: "from-emerald-400 to-green-600", desc: "전반적으로 아주 강한 기운의 조합" },
  { grade: "B", label: "길상", color: "from-celestial-cyan to-indigo-500", desc: "균형 잡힌 운세, 적극적으로 활용 가능" },
  { grade: "C", label: "중평", color: "from-slate-400 to-slate-600", desc: "일부 어려움이 있으나 극복 가능한 조합" },
  { grade: "D", label: "다다익선", color: "from-rose-400 to-pink-600", desc: "성장 여지가 많은 시기, 주의 시점 활용 권장" },
] as const;

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

  const { destinyTier, potentialTier, domainTiers, sajuResult, vedicResult, strengths, weaknesses } = result;
  const hasSaju = !!sajuReport?.report;
  const hasVedic = !!report?.report;
  const hasTransit = !!transitReport?.current_frame;

  const potentialHigher = result.potentialScore > result.destinyScore;
  const tierUi = TIER_GRADES_UI.find((t) => t.grade === destinyTier.grade) ?? TIER_GRADES_UI[TIER_GRADES_UI.length - 1];

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

      {/* 메인 티어 카드 */}
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

      {/* 운명 / 잠재력 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass p-6 rounded-2xl border border-celestial-purple/20">
          <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
            <Trophy className="w-5 h-5 text-celestial-purple" />
            운명 티어
          </h5>
          <p className="text-2xl font-black text-celestial-purple mb-2">{destinyTier.grade} · {destinyTier.label}</p>
          <p className="text-xs text-white/50 mb-4">
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

      {/* 분야별 티어 */}
      {domainTiers.length > 0 && (
        <div className="glass p-6 rounded-2xl">
          <h5 className="text-lg font-bold text-white mb-4">분야별 티어</h5>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-3">
            {domainTiers.map(({ house, domain, tier }) => {
              const tierColor = tier === "S" ? "bg-amber-500/20 text-amber-400 border-amber-500/40"
                : tier === "A" ? "bg-emerald-500/20 text-emerald-400 border-emerald-500/40"
                  : tier === "B" ? "bg-celestial-cyan/20 text-celestial-cyan border-celestial-cyan/40"
                    : tier === "C" ? "bg-slate-500/20 text-slate-300 border-slate-500/40"
                      : "bg-rose-500/20 text-rose-400 border-rose-500/40";
              return (
                <div key={house} className={`p-3 rounded-xl border ${tierColor}`} title={`H${house} · ${domain}`}>
                  <p className="text-[10px] text-white/60 mb-1">{domain}</p>
                  <p className="text-lg font-black">{tier}</p>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* 사주 / 베딕 기여도 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {hasSaju && (
          <div className="glass p-6 rounded-2xl border border-celestial-purple/20">
            <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
              <Activity className="w-5 h-5 text-celestial-purple" />
              사주 기반
            </h5>
            <div className="flex items-baseline gap-2 mb-4">
              <span className="text-3xl font-bold text-celestial-purple">{sajuResult.score}</span>
              <span className="text-white/40">/ 100</span>
            </div>
            <ul className="space-y-1.5">
              {sajuResult.highlights.map((h, i) => (
                <li key={i} className="text-sm text-white/70 flex items-center gap-2">
                  <Sparkles className="w-3 h-3 text-celestial-purple shrink-0" />
                  {h}
                </li>
              ))}
            </ul>
          </div>
        )}
        {hasVedic && (
          <div className="glass p-6 rounded-2xl border border-celestial-cyan/20">
            <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
              <Star className="w-5 h-5 text-celestial-cyan" />
              베딕 기반
            </h5>
            <div className="flex items-baseline gap-2 mb-4">
              <span className="text-3xl font-bold text-celestial-cyan">{vedicResult.score}</span>
              <span className="text-white/40">/ 100</span>
            </div>
            <ul className="space-y-1.5">
              {vedicResult.highlights.map((h, i) => (
                <li key={i} className="text-sm text-white/70 flex items-center gap-2">
                  <Target className="w-3 h-3 text-celestial-cyan shrink-0" />
                  {h}
                </li>
              ))}
            </ul>
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
                  <span className="w-1.5 h-1.5 rounded-full bg-emerald-400" />
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
                  <span className="w-1.5 h-1.5 rounded-full bg-amber-400" />
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

      {/* 종합 인사이트 */}
      <div className="glass p-6 rounded-2xl">
        <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
          <TrendingUp className="w-5 h-5 text-celestial-gold" />
          종합 인사이트
        </h5>
        <p className="text-white/80 leading-relaxed">
          {destinyTier.grade === "S" && "사주와 별운이 서로 보완하며 극상의 기운을 이룹니다. 현재의 흐름을 잘 활용하시고, 기회가 오면 적극적으로 도전해 보세요."}
          {destinyTier.grade === "A" && "전반적으로 강한 차트입니다. 용신·대운·요가가 유리하게 작용하는 시기에는 더욱 적극적인 도전을 권합니다."}
          {destinyTier.grade === "B" && "균형이 잘 잡힌 운세입니다. 강점을 살리고 약점은 주의 시점 참고를 통해 보완하면 좋은 결과를 기대할 수 있습니다."}
          {destinyTier.grade === "C" && "일부 어려운 시기가 있으나 극복 가능합니다. 역량 및 기운 탭의 주의 시점을 참고하고, 골든타임·대운 흐름을 활용해 보세요."}
          {destinyTier.grade === "D" && "성장 여지가 많은 시기입니다. 주의가 필요한 시기를 피하고, 용신·요가가 도와주는 구간을 적극 활용하시길 권합니다."}
        </p>
      </div>
    </motion.div>
  );
}
