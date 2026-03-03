import { motion } from "framer-motion";
import { Heart, Clock, Star, Shield, Sparkles, Eye, Users } from "lucide-react";
import { cn } from "../../utils";
import type { VedicAnalysisResult } from "../../types";

interface OverviewTabProps {
  report: VedicAnalysisResult;
}

export function OverviewTab({ report }: OverviewTabProps) {
  if (!report || !report.report) return null;
  const r = report.report;
  const chart = report.chart;
  const qualityLabel = (quality: string) => {
    if (quality === "VeryHigh") return "최상";
    if (quality === "High") return "상";
    if (quality === "Medium") return "중";
    return "하";
  };

  const ratingLabel = (rating: string) => {
    if (rating === "Excellent") return "최상";
    if (rating === "Strong") return "강함";
    if (rating === "Average") return "보통";
    return "약함";
  };

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
                    <span className={`text-[10px] font-black px-2 py-0.5 rounded-full border shrink-0 ml-2 ${qColor}`}>{qualityLabel(quality)}</span>
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
          {r.house_summary.map((house: any) => {
            const bhava = chart?.bhava_strengths?.find((b: any) => b.house === house.house);
            return (
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
                  {ratingLabel(house.rating)}
                </span>
                {bhava && (
                  <div className="mt-3 space-y-1 text-left">
                    {[
                      { label: "주인 행성", value: bhava.lord_score },
                      { label: "방위 힘", value: bhava.dig_score },
                      { label: "시선 영향", value: bhava.drishti_score },
                    ].map(({ label, value }) => (
                      <div key={label}>
                        <div className="flex justify-between text-[9px] text-white/40 mb-0.5">
                          <span>{label}</span>
                          <span>{(value ?? 0).toFixed(0)}</span>
                        </div>
                        <div className="w-full bg-white/10 h-1 rounded-full overflow-hidden">
                          <div className="h-full rounded-full bg-celestial-cyan/60 transition-all" style={{ width: `${Math.min(100, Math.max(0, (value ?? 0) / 60 * 100))}%` }} />
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </section>

      {/* 8 카라카 전체 */}
      {chart?.karakas && chart.karakas.length > 0 && (
        <section className="glass p-10 rounded-[2.5rem]">
          <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
            <Users className="w-6 h-6 text-celestial-purple" />
            제미니 카라카 — 8가지 인생 역할 배정
          </h5>
          <p className="text-xs text-white/40 mb-6">태양계 행성들이 당신의 인생에서 맡는 구체적인 역할입니다. 사이드리얼 도수가 높을수록 그 역할의 영향이 큽니다.</p>
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
            {chart.karakas.map((k: any, i: number) => {
              const roleKr: Record<string, string> = {
                Atmakaraka: "영혼 (나 자신)",
                Amatyakaraka: "직업 / 재능",
                Bhratrukaraka: "형제 / 자매",
                Matrukaraka: "어머니 / 보호",
                Pitrikaraka: "아버지 / 권위",
                Putrakaraka: "자식 / 창작",
                Gnatikaraka: "경쟁자 / 친척",
                Darakaraka: "배우자 / 파트너",
              };
              const colors = [
                "border-celestial-gold/40 bg-celestial-gold/10",
                "border-celestial-purple/40 bg-celestial-purple/10",
                "border-green-500/30 bg-green-500/10",
                "border-pink-500/30 bg-pink-500/10",
                "border-blue-500/30 bg-blue-500/10",
                "border-orange-500/30 bg-orange-500/10",
                "border-red-500/30 bg-red-500/10",
                "border-celestial-cyan/30 bg-celestial-cyan/10",
              ];
              return (
                <div key={i} className={`p-4 rounded-2xl border ${colors[i % colors.length]}`}>
                  <p className="text-[10px] font-bold text-white/40 uppercase tracking-wider mb-1">{roleKr[k.role] ?? k.role}</p>
                  <p className="text-lg font-bold text-white">{k.planet}</p>
                  <p className="text-xs text-white/40 mt-1">{(k.degree_in_rasi ?? 0).toFixed(2)}°</p>
                </div>
              );
            })}
          </div>
        </section>
      )}

      {/* 행성 시선(Aspects) */}
      {chart?.aspects && chart.aspects.length > 0 && (
        <section className="glass p-10 rounded-[2.5rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Eye className="w-6 h-6 text-celestial-cyan" />
            행성 시선 (Drishti) — 행성 간 상호 영향
          </h5>
          <p className="text-xs text-white/40 mb-6">각 행성이 바라보는(시선을 보내는) 하우스 번호입니다. 해당 하우스에 있는 행성이나 영역에 직접적인 영향을 미칩니다.</p>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-white/10">
                  <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">행성</th>
                  <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">바라보는 하우스</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                {chart.aspects.map((a: any, i: number) => (
                  <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                    <td className="py-2.5 pr-4 font-bold text-white whitespace-nowrap">{a.aspecting_planet}</td>
                    <td className="py-2.5 pr-4">
                      <div className="flex flex-wrap gap-1.5">
                        {(a.aspected_houses ?? []).map((h: number) => (
                          <span key={h} className="px-2 py-0.5 rounded-lg bg-celestial-cyan/15 border border-celestial-cyan/30 text-celestial-cyan text-xs font-mono font-bold">H{h}</span>
                        ))}
                      </div>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
      )}
    </motion.div>
  );
}
