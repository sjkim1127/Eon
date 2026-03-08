import { Activity, Star, Sparkles, Target } from "lucide-react";
import { ScoreBar } from "./ScoreBar";

interface Props {
  hasSaju: boolean;
  hasVedic: boolean;
  sajuResult: any;
  vedicResult: any;
  strengthNorm: number;
  deukSum: number;
  throughput: number;
  goldenTimePt: number;
  stabilityPt: number;
  planetStrengthNorm: number;
  highYogaCount: number;
  yogaPt: number;
  strongHouses: number;
  housePt: number;
  satiPt: number;
  dashaPt: number;
}

export function ScoreBreakdown({
  hasSaju, hasVedic, sajuResult, vedicResult,
  strengthNorm, deukSum, throughput, goldenTimePt, stabilityPt,
  planetStrengthNorm, highYogaCount, yogaPt, strongHouses, housePt, satiPt, dashaPt
}: Props) {
  return (
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
              {sajuResult.highlights.map((h: string, i: number) => (
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
              {vedicResult.highlights.map((h: string, i: number) => (
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
  );
}