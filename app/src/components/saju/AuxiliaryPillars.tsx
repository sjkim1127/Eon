import { Info, Sparkles } from "lucide-react";
import type { SupplementaryPillars } from "../../types/saju";
import { ganziDisplay, ganziHangul } from "../../utils/ganzi";

interface AuxiliaryPillarsProps {
  data: SupplementaryPillars;
  auxShinsals?: [string, string, string][];
  unknownTime?: boolean;
}

export function AuxiliaryPillars({ data, auxShinsals = [], unknownTime = false }: AuxiliaryPillarsProps) {
  const pillars = [
    { label: "태원 (胎元)", sub: "잉태 기운", value: data.taewon, desc: "잉태 시점(출생 10개월 전)의 기운으로, 선천적인 복록과 신체적 근간을 상징합니다.", key: "태원" },
    { label: "명궁 (命宮)", sub: "정신·의지", value: data.myeonggung, desc: "본인의 타고난 정신적 지향점과 운세의 바탕이 됩니다. 운의 길흉을 중재하는 역할을 합니다.", warning: unknownTime, key: "명궁" },
    { label: "신궁 (身宮)", sub: "행태·태도", value: data.shingung, desc: "후천적인 삶의 태도와 환경에 대한 적응력을 상징합니다. 중년 이후 삶에 큰 영향을 미칩니다.", warning: unknownTime, key: "신궁" },
  ];

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between px-2">
        <h3 className="text-white/30 text-[10px] font-bold uppercase tracking-widest">
          {data.meta.formula_name} {data.meta.formula_version}
        </h3>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {pillars.map((p, idx) => (
          <div key={idx} className="glass p-6 rounded-3xl relative overflow-hidden group">
            {/* Background Highlight */}
            <div className="absolute -right-4 -top-4 w-24 h-24 bg-white/5 rounded-full blur-2xl group-hover:bg-white/10 transition-colors" />
            
            <div className="relative z-10">
              <div className="flex justify-between items-start mb-4">
                <div>
                  <h4 className="text-white font-bold text-lg">{p.label}</h4>
                  <p className="text-white/40 text-xs uppercase tracking-wider font-semibold">{p.sub}</p>
                </div>
                <div className="p-2 rounded-xl bg-white/5 text-white/40 group-hover:text-celestial-gold transition-colors">
                  <Info className="w-4 h-4" />
                </div>
              </div>

              <div className="flex items-baseline gap-3 mb-4">
                <span className="text-4xl font-black text-white tracking-tighter">
                  {ganziHangul(p.value)}
                </span>
                <span className="text-xl font-medium text-white/20">
                  {ganziDisplay(p.value)}
                </span>
              </div>

              <p className="text-sm text-white/50 leading-relaxed min-h-[3rem]">
                {p.desc}
              </p>

              {/* Shinsal Badges */}
              <div className="flex flex-wrap gap-2 mt-4">
                {auxShinsals
                  .filter(([pillarName]) => pillarName === p.key)
                  .map(([_, criteria, shinsalName], sIdx) => {
                    const isAuspicious = ["천을귀인", "건록", "장성살", "반안살", "지살"].includes(shinsalName);
                    return (
                      <div
                        key={sIdx}
                        className={`flex items-center gap-1.5 px-2 py-1 rounded-lg border text-[10px] font-bold transition-all hover:scale-105 ${
                          isAuspicious 
                            ? "bg-celestial-gold/10 border-celestial-gold/20 text-celestial-gold" 
                            : "bg-white/5 border-white/10 text-white/40"
                        }`}
                      >
                        <Sparkles className={`w-3 h-3 ${isAuspicious ? "text-celestial-gold" : "text-white/20"}`} />
                        <span>{shinsalName}</span>
                        <span className="text-white/10 font-normal">({criteria})</span>
                      </div>
                    );
                  })}
              </div>

              {/* Dynamic Interpretation (SSOT from Engine) */}
              {(() => {
                const interp = data.interpretations?.find(i => i.pillar_name === p.key);
                if (!interp) return null;

                const isAuspicious = interp.level === "Auspicious";
                
                return (
                  <div className={`mt-4 p-3 rounded-2xl text-[11px] leading-relaxed border ${
                    isAuspicious ? "bg-celestial-gold/5 border-celestial-gold/10 text-celestial-gold/70" : "bg-rose-500/5 border-rose-500/10 text-rose-300/60"
                  }`}>
                    <p className="font-semibold mb-1 opacity-80">
                      {interp.summary}
                    </p>
                    <p>{interp.description}</p>
                  </div>
                );
              })()}

              {p.warning && (
                <p className="mt-3 text-[10px] text-amber-500/60 font-medium">
                  * 시간 미상으로 인해 정확도가 낮을 수 있습니다.
                </p>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
