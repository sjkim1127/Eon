import { Trophy, Zap, Shield, Loader2 } from "lucide-react";
import { TierResult } from "../../utils/tierScore";
import { TIER_GRADES_UI, PROFILE_META, RISK_META } from "./destinyUtils";

interface Props {
  result: TierResult;
  hasTransit: boolean;
  transitResult: any;
}

export function TierCard({ result, hasTransit, transitResult }: Props) {
  const { destinyTier, potentialTier, profile, riskLevel, growthGap, natalScore, currentScore, destinyScore } = result;

  const tierUi = TIER_GRADES_UI.find((t) => t.grade === destinyTier.grade) ?? TIER_GRADES_UI[TIER_GRADES_UI.length - 1];
  const profileMeta = PROFILE_META[profile] ?? PROFILE_META["balanced"];
  const riskMeta = RISK_META[riskLevel] ?? RISK_META["low"];
  const potentialHigher = result.potentialScore > result.destinyScore;

  // 트랜싯 점수 색상
  const transitColor =
    transitResult?.score >= 60 ? "text-celestial-cyan" :
    transitResult?.score >= 40 ? "text-amber-300" : "text-rose-400";
  const transitBorder =
    transitResult?.score >= 60 ? "border-celestial-cyan/20" :
    transitResult?.score >= 40 ? "border-amber-500/20" : "border-rose-500/20";

  return (
    <div className="space-y-6">
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
          <div className="flex-1 space-y-4 w-full">
            <div className="flex items-center gap-2">
              <span className="text-white/60">종합 점수</span>
              <span className="text-2xl font-bold text-white">{Math.round(destinyScore)}</span>
              <span className="text-white/40">/ 100</span>
            </div>
            <div className="w-full bg-white/10 h-3 rounded-full overflow-hidden">
              <div
                className={`h-full rounded-full bg-gradient-to-r ${tierUi.color} transition-all duration-700`}
                style={{ width: `${destinyScore}%` }}
              />
            </div>
            <div className="flex flex-wrap gap-2 text-sm">
              <span className="text-white/50">원국 {Math.round(natalScore)}</span>
              {hasTransit && (
                <span className="text-white/50">· 현재 {Math.round(currentScore)}</span>
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
            {hasTransit ? `원국 ${Math.round(natalScore)} · 현재 ${Math.round(currentScore)} = 종합 ${Math.round(destinyScore)}` : `종합 점수 ${Math.round(destinyScore)}`}
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
      {hasTransit && transitResult && (
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
          {transitResult.highlights && transitResult.highlights.length > 0 && (
            <ul className="space-y-1.5">
              {transitResult.highlights.map((h: string, i: number) => (
                <li key={i} className={`text-sm flex items-center gap-2 ${transitColor}`}>
                  <span className="w-1.5 h-1.5 rounded-full bg-current shrink-0" />
                  {h}
                </li>
              ))}
            </ul>
          )}
        </div>
      )}
    </div>
  );
}
