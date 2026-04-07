import { Star } from "lucide-react";
import { ELEMENT_INFO, TENGOD_INFO } from "../../constants";

interface Props {
  power: any;
}

export function PowerAnalysis({ power }: Props) {
  if (!power) return null;
  const { options, elementScores, tenGodScores } = power;

  const getStatusColor = (percent: number) => {
    if (percent < 10) return "text-white/40"; // 부족
    if (percent <= 20) return "text-emerald-400"; // 적정
    if (percent <= 35) return "text-amber-400"; // 발달
    return "text-red-400"; // 과다
  };

  const getStatusText = (percent: number) => {
    if (percent < 10) return "부족";
    if (percent <= 20) return "적정";
    if (percent <= 35) return "발달";
    return "과다";
  };

  return (
    <div className="glass p-8 rounded-[2rem]">
      <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
        <Star className="w-6 h-6 text-celestial-gold" />
        오행과 십성 분석
      </h5>

      <div className="flex flex-col gap-1 mb-6 text-xs text-white/50 bg-white/5 p-4 rounded-xl border border-white/10">
        <p className="flex items-center gap-2">
          <span className="w-4 h-4 rounded-full bg-celestial-gold/20 flex items-center justify-center text-[10px] text-celestial-gold">✓</span>
          합에 따른 오행 변화 적용 {options?.applyTransform ? "(적용됨)" : "(미적용)"}
        </p>
        <p className="flex items-center gap-2">
          <span className="w-4 h-4 rounded-full bg-celestial-gold/20 flex items-center justify-center text-[10px] text-celestial-gold">✓</span>
          조후와 궁성 보정값 적용 {options?.applyCorrection ? "(적용됨)" : "(미적용)"}
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
        {/* 오행 */}
        <div>
          <h6 className="text-sm font-bold text-brand-400 mb-4 uppercase tracking-wider">오행 분포</h6>
          <div className="space-y-3">
            {elementScores?.map(([elKey, percent]: [string, number, number], idx: number) => {
              const info = ELEMENT_INFO[elKey];
              if (!info) return null;
              return (
                <div key={idx} className="flex items-center justify-between p-3 rounded-xl bg-white/5 border border-white/10 hover:bg-white/10 transition-colors">
                  <div className="flex items-center gap-3">
                    <span className="text-lg font-black text-white">{info.hangul}({info.hanja})</span>
                  </div>
                  <div className="flex items-center gap-4 text-sm font-bold">
                    <span className="text-white/80 w-12 text-right">{percent > 0 ? `${percent.toFixed(1)}%` : "-"}</span>
                    <span className={`w-10 text-right ${percent > 0 ? getStatusColor(percent) : "text-white/20"}`}>
                      {percent > 0 ? getStatusText(percent) : "부재"}
                    </span>
                  </div>
                </div>
              );
            })}
          </div>
        </div>

        {/* 십성 */}
        <div>
          <h6 className="text-sm font-bold text-celestial-purple/80 mb-4 uppercase tracking-wider">십성 분포</h6>
          <div className="space-y-2">
            {tenGodScores?.map(([godKey, percent]: [string, number, number], idx: number) => {
              const info = TENGOD_INFO[godKey];
              if (!info) return null;
              const isPresent = percent > 0;
              return (
                <div key={idx} className={`flex items-center justify-between p-2 rounded-lg border ${isPresent ? 'bg-white/5 border-white/10' : 'bg-transparent border-transparent'} transition-colors`}>
                  <div className="flex items-center gap-3">
                    <span className={`${isPresent ? 'text-white' : 'text-white/30'} font-semibold text-sm`}>{info.hangul}({info.hanja})</span>
                  </div>
                  <div className="flex items-center gap-4 text-xs font-bold">
                    <span className={`${isPresent ? 'text-celestial-gold' : 'text-white/20'} w-12 text-right`}>
                      {isPresent ? `${percent.toFixed(1)}%` : "-"}
                    </span>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      </div>
    </div>
  );
}
