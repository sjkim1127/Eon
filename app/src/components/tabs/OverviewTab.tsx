import { motion } from "framer-motion";
import { Heart, Clock, Star, Shield, Sparkles } from "lucide-react";
import { cn } from "../../utils";
import type { VedicAnalysisResult } from "../../types";

interface OverviewTabProps {
  report: VedicAnalysisResult;
}

export function OverviewTab({ report }: OverviewTabProps) {
  if (!report || !report.report) return null;
  const r = report.report;

  return (
    <motion.div
      key="results"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* Hero Statistics – Karaka 트리오 + 차트 강도 */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
          <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
            <Heart className="w-32 h-32" />
          </div>
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            영혼의 지표 (Atmakaraka)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {r.primary_karakas.atmakaraka}
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            이번 생에서 영혼이 추구하는 가장 강력한 욕망과 핵심 과제를 나타냅니다.
          </p>
        </div>

        <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
          <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
            직업 지표 (Amatyakaraka)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {r.primary_karakas.amatyakaraka}
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            직업·사회적 역할에서 영혼을 보필하는 행성입니다.
          </p>
        </div>

        <div className="glass p-8 rounded-[2rem] border-celestial-cyan/20 bg-celestial-cyan/5">
          <p className="text-celestial-cyan/80 text-sm font-bold uppercase tracking-wider mb-2">
            파트너 지표 (Darakaraka)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {r.primary_karakas.darakaraka}
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            배우자·가까운 파트너와의 관계 패턴을 나타내는 행성입니다.
          </p>
        </div>
      </div>

      {/* Dasha + 전체 강도 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
          <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
            현재 대운 (Dasha)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {r.dasha_focus.replace("Current Major Period: ", "")}
          </h4>
          <div className="flex items-center gap-2 text-sm text-white/60">
            <Clock className="w-4 h-4" />
            <span>인생의 현재 단계에서 가장 강력한 영향을 미치는 기운입니다.</span>
          </div>
        </div>

        <div className="glass p-8 rounded-[2rem]">
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            전체 차트 강도
          </p>
          <div className="flex items-baseline gap-2 mb-4">
            <h4 className="text-5xl font-black text-gradient leading-none">
              {Math.round(r.overall_strength_score)}
            </h4>
            <span className="text-white/20 font-bold">/ 600</span>
          </div>
          <div className="w-full bg-white/5 h-2 rounded-full overflow-hidden">
            <div
              className="bg-celestial-purple h-full rounded-full transition-all duration-1000"
              style={{ width: `${(r.overall_strength_score / 600) * 100}%` }}
            />
          </div>
        </div>
      </div>

      {/* Secondary Info */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        <div className="glass p-10 rounded-[2.5rem]">
          <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
            <Star className="w-6 h-6 text-celestial-gold" />
            낙샤트라 청사진
          </h5>
          <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
            <p className="text-white text-lg font-medium leading-relaxed">
              {r.nakshatra_info}
            </p>
          </div>
        </div>

        <div className="glass p-10 rounded-[2.5rem]">
          <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
            <Shield className="w-6 h-6 text-celestial-cyan" />
            현재 운세 경고 (사데사티)
          </h5>
          <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
            <p className="text-white text-lg font-medium leading-relaxed">
              {r.sade_sati}
            </p>
          </div>
        </div>
      </div>

      {/* Yoga 섹션 */}
      {r.yogas && r.yogas.length > 0 && (
        <section>
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Sparkles className="w-6 h-6 text-celestial-gold" />
            베딕 요가 (Yoga) 분석
          </h5>
          <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
            {r.yogas.map((yoga: any, i: number) => {
              const quality = typeof yoga.quality === "string" ? yoga.quality : Object.keys(yoga.quality)[0];
              const weakReason = typeof yoga.quality === "object" && yoga.quality.Weak ? yoga.quality.Weak : null;
              const qColor =
                quality === "VeryHigh" ? "text-green-400 border-green-500/30 bg-green-500/10"
                : quality === "High" ? "text-celestial-cyan border-celestial-cyan/30 bg-celestial-cyan/10"
                : quality === "Medium" ? "text-celestial-gold border-celestial-gold/30 bg-celestial-gold/10"
                : "text-white/50 border-white/20 bg-white/5";
              return (
                <div key={i} className={`p-5 rounded-2xl border ${qColor}`}>
                  <div className="flex items-start justify-between mb-2">
                    <p className="text-sm font-bold text-white">{yoga.name}</p>
                    <span className={`text-[10px] font-black px-2 py-0.5 rounded-full border shrink-0 ml-2 ${qColor}`}>{quality}</span>
                  </div>
                  <p className="text-xs text-white/50 leading-relaxed mb-2">{yoga.description}</p>
                  {weakReason && <p className="text-xs text-white/30 italic">※ {weakReason}</p>}
                  {yoga.planets_involved?.length > 0 && (
                    <div className="flex flex-wrap gap-1 mt-2">
                      {yoga.planets_involved.map((pl: string, j: number) => (
                        <span key={j} className="text-[10px] px-2 py-0.5 rounded-full bg-white/10 text-white/60 border border-white/10">{pl}</span>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </section>
      )}

      {/* House Grid */}
      <section>
        <h5 className="text-xl font-bold text-white mb-6">하우스(Bhava)별 에너지 역량</h5>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
          {r.house_summary.map((house: any) => (
            <div
              key={house.house}
              className="glass p-6 rounded-2xl text-center glass-hover cursor-help"
            >
              <p className="text-xs text-white/30 font-bold mb-1">하우스 {house.house}</p>
              <p className="text-2xl font-bold text-white mb-2">
                {Math.round(house.total_score)}
              </p>
              <span
                className={cn(
                  "px-2 py-0.5 rounded text-[10px] font-black uppercase",
                  house.rating === "Excellent"
                    ? "bg-green-500/20 text-green-400"
                    : house.rating === "Strong"
                      ? "bg-blue-500/20 text-blue-400"
                      : house.rating === "Average"
                        ? "bg-yellow-500/20 text-yellow-400"
                        : "bg-red-500/20 text-red-400"
                )}
              >
                {house.rating}
              </span>
            </div>
          ))}
        </div>
      </section>
    </motion.div>
  );
}
