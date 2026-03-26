import { Info } from "lucide-react";
import type { SupplementaryPillars } from "../../types/saju";
import { ganziDisplay, ganziHangul } from "../../utils/ganzi";

interface AuxiliaryPillarsProps {
  data: SupplementaryPillars;
  unknownTime?: boolean;
}

export function AuxiliaryPillars({ data, unknownTime = false }: AuxiliaryPillarsProps) {
  const pillars = [
    { label: "태원 (胎元)", sub: "잉태 기운", value: data.taewon, desc: "잉태 시점(출생 10개월 전)의 기운으로, 선천적인 복록과 신체적 근간을 상징합니다." },
    { label: "명궁 (命宮)", sub: "정신·의지", value: data.myeonggung, desc: "본인의 타고난 정신적 지향점과 운세의 바탕이 됩니다. 운의 길흉을 중재하는 역할을 합니다.", warning: unknownTime },
    { label: "신궁 (身宮)", sub: "행태·태도", value: data.shingung, desc: "후천적인 삶의 태도와 환경에 대한 적응력을 상징합니다. 중년 이후 삶에 큰 영향을 미칩니다.", warning: unknownTime },
  ];

  return (
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

            {p.warning && (
              <p className="mt-3 text-[10px] text-amber-500/60 font-medium">
                * 시간 미상으로 인해 정확도가 낮을 수 있습니다.
              </p>
            )}
          </div>
        </div>
      ))}
    </div>
  );
}
