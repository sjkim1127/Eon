import { Zap } from "lucide-react";
import { STRENGTH_INFO, ELEMENT_INFO, STRUCTURE_INFO, YONGSHIN_TYPE_INFO } from "../../constants";

interface Props {
  s: any;
  y: any;
  st: any;
}

export function SajuSummary({ s, y, st }: Props) {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
      {/* 역량 분석 */}
      <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
        <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
          <Zap className="w-32 h-32" />
        </div>
        <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
          신강/신약 분석
        </p>
        <h4 className="text-3xl font-bold text-white mb-2">
          {STRENGTH_INFO[s?.strengthType] || s?.strengthType || "—"}
        </h4>
        <div className="flex items-baseline gap-2 mb-4">
          <span className="text-5xl font-black text-gradient leading-none">
            {s?.strengthScore != null ? Math.round(s.strengthScore) : "—"}
          </span>
          <span className="text-white/20 font-bold">점</span>
        </div>
        <div className="space-y-1 text-xs text-white/50">
          <p>득령: {s?.deukRyeong?.acquired ? "✅" : "❌"}</p>
          <p>득지: {s?.deukJi?.acquired ? "✅" : "❌"}</p>
          <p>득시: {s?.deukSi?.acquired ? "✅" : "❌"}</p>
          <p>득세: {s?.deukSe?.acquired ? "✅" : "❌"}</p>
        </div>
      </div>

      {/* 용신 */}
      <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
        <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
          용신 (用神)
        </p>
        <h4 className="text-3xl font-bold text-white mb-4">
          {ELEMENT_INFO[y?.primary]?.hangul || y?.primary || "—"}
          <span className="text-lg text-white/40 ml-2">{ELEMENT_INFO[y?.primary]?.hanja}</span>
        </h4>
        {/* 용신 상세 목록 (조후/억부/통관/병약) */}
        {y?.recommendations && y.recommendations.length > 0 && (
          <div className="space-y-4 mb-3">
            {y.recommendations.map((rec: any, i: number) => (
              <div key={i} className="flex flex-col gap-1.5 p-3 rounded-2xl bg-white/5 border border-white/10">
                <div className="flex items-center gap-2">
                  <span className="shrink-0 px-1.5 py-0.5 rounded bg-celestial-purple/30 text-celestial-purple/90 text-[10px] font-bold">
                    {YONGSHIN_TYPE_INFO[rec.yongshinType] || rec.yongshinType}
                  </span>
                  <span className="text-white font-bold text-xs">
                    {ELEMENT_INFO[rec.element]?.hangul || rec.element} ({ELEMENT_INFO[rec.element]?.hanja})
                  </span>
                </div>
                <p className="text-[11px] text-white/80 font-semibold leading-tight">{rec.summary}</p>
                <p className="text-[10px] text-white/40 leading-relaxed italic line-clamp-2">{rec.description}</p>
                {rec.reasons?.length > 0 && (
                  <div className="flex flex-wrap gap-1 mt-1">
                    {rec.reasons.map((reason: string, idx: number) => (
                      <span key={idx} className="text-[9px] px-1.5 py-0.5 rounded-full bg-white/5 text-white/30 border border-white/10">
                        {reason}
                      </span>
                    ))}
                  </div>
                )}
              </div>
            ))}
          </div>
        )}
        <div className="flex items-center justify-between mt-auto pt-2 border-t border-white/5">
          <p className="text-[10px] text-white/30 uppercase tracking-tighter font-bold">희신 (喜神)</p>
          <p className="text-xs text-white/60 font-bold italic">
            {ELEMENT_INFO[y?.assistant]?.hangul || y?.assistant || "—"} ({ELEMENT_INFO[y?.assistant]?.hanja})
          </p>
        </div>
      </div>

      {/* 격국 */}
      <div className="glass p-8 rounded-[2rem]">
        <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
          격국 (格局)
        </p>
        <h4 className="text-3xl font-bold text-white mb-3">
          {STRUCTURE_INFO[st?.structure]?.hangul || st?.structure || "—"}
          <span className="text-sm text-white/40 ml-2 font-medium">{STRUCTURE_INFO[st?.structure]?.hanja}</span>
        </h4>
        
        <div className="space-y-3">
          <p className="text-xs text-white shadow-sm p-3 rounded-2xl bg-brand-500/10 border border-brand-500/20 font-bold leading-snug">
            {st?.summary || "격국이 뚜렷하지 않은 일반 사주입니다."}
          </p>
          <p className="text-[11px] text-white/50 leading-relaxed px-1">
            {st?.description || ""}
          </p>
          
          {st?.reasons?.length > 0 && (
            <div className="pt-3 border-t border-white/5 space-y-2">
              <p className="text-[9px] text-white/20 font-bold uppercase tracking-wider">판정 근거</p>
              <div className="flex flex-wrap gap-1.5">
                {st.reasons.map((reason: string, idx: number) => (
                  <span key={idx} className="text-[9px] px-2 py-0.5 rounded-md bg-white/5 text-white/40 border border-white/5 font-medium">
                    {reason}
                  </span>
                ))}
                {st.projectionPath && (
                  <span className="text-[9px] px-2 py-0.5 rounded-md bg-sky-500/10 text-sky-400 border border-sky-500/20 font-bold">
                    {st.projectionPath} 투출
                  </span>
                )}
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
