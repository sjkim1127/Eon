import { Compass, Briefcase, Heart, Star, Zap } from "lucide-react";
import type { VedicAnalysisReport, TajikaReport, Saham, VargaInterpretation } from "../../../types/vedic";

interface VedicAdvancedInsightsSectionProps {
  report: VedicAnalysisReport;
  tajikaReport?: TajikaReport | null;
}

export function VedicAdvancedInsightsSection({ report, tajikaReport }: VedicAdvancedInsightsSectionProps) {
  const arudhaLagna = report.arudhaLagna ?? 0;
  const upapadaLagna = report.upapadaLagna ?? 0;
  const specialLagnasSummary = report.specialLagnasSummary ?? [];
  const d9MarriageAnalysis = report.d9MarriageAnalysis ?? "";
  const d10CareerAnalysis = report.d10CareerAnalysis ?? "";
  const vargaInterpretations = report.vargaInterpretations ?? [];

  const sahams = tajikaReport?.sahams ?? [];
  const harshaBalaSummary = tajikaReport?.harshaBalaSummary ?? [];
  const yearLord = tajikaReport?.yearLord;
  const munthaRasi = tajikaReport?.munthaRasi;

  // Filter special planets
  const specialPlanets = (vargaInterpretations as VargaInterpretation[])?.filter((vi: VargaInterpretation) => vi.isVargottama || vi.isPushkarNavamsa) || [];

  return (
    <div className="space-y-8">
      {/* ── Jaimini & Special Lagnas ──────────────────────────────── */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
          <Compass className="w-6 h-6 text-celestial-gold" />
          제미니 지표 및 특별 라그나 (Jaimini & Special Lagnas)
        </h5>
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
          {arudhaLagna > 0 && (
            <div className="p-4 bg-white/5 rounded-xl border border-white/10 text-center">
              <p className="text-[10px] text-white/40 font-bold uppercase mb-1">Arudha Lagna (AL)</p>
              <p className="text-lg font-bold text-celestial-gold">사인 {arudhaLagna}</p>
              <p className="text-[10px] text-white/30 mt-1">사회적 이미지·외부적 자아</p>
            </div>
          )}
          {upapadaLagna > 0 && (
            <div className="p-4 bg-white/5 rounded-xl border border-white/10 text-center">
              <p className="text-[10px] text-white/40 font-bold uppercase mb-1">Upapada Lagna (UL)</p>
              <p className="text-lg font-bold text-pink-400">사인 {upapadaLagna}</p>
              <p className="text-[10px] text-white/30 mt-1">배우자·결혼 생활의 기운</p>
            </div>
          )}
          {(specialLagnasSummary as [string, number][])?.map(([name, rasi]: [string, number]) => (
            <div key={name} className="p-4 bg-white/5 rounded-xl border border-white/10 text-center">
              <p className="text-[10px] text-white/40 font-bold uppercase mb-1">{name}</p>
              <p className="text-lg font-bold text-white">사인 {rasi}</p>
            </div>
          ))}
        </div>
      </div>

      {/* ── Divisional Analysis (D9/D10) ─────────────────────────── */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass p-8 rounded-[2rem] border-pink-500/20 bg-pink-500/5">
          <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-3">
            <Heart className="w-5 h-5 text-pink-400" />
            나밤사(D9) — 결혼 및 내적 성숙
          </h5>
          <p className="text-sm text-white/80 leading-relaxed bg-black/20 p-4 rounded-xl border border-white/5">
            {d9MarriageAnalysis}
          </p>
          <p className="text-[11px] text-white/40 mt-4 leading-relaxed">
            D9 차트는 영혼의 열매이자 배우자와의 깊은 카르마적 연결, 중년 이후의 운을 나타냅니다.
          </p>
        </div>

        <div className="glass p-8 rounded-[2rem] border-celestial-cyan/20 bg-celestial-cyan/5">
          <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-3">
            <Briefcase className="w-5 h-5 text-celestial-cyan" />
            다샴사(D10) — 사회적 성취 및 직업
          </h5>
          <p className="text-sm text-white/80 leading-relaxed bg-black/20 p-4 rounded-xl border border-white/5">
            {d10CareerAnalysis}
          </p>
          <p className="text-[11px] text-white/40 mt-4 leading-relaxed">
            D10 차트는 전문적인 명성, 직업적 권위, 그리고 사회에 기여하는 방식을 분석합니다.
          </p>
        </div>
      </div>

      {/* ── Tajika & Special Status ─────────────────────────────── */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* Tajika Sahams */}
        <div className="glass p-6 rounded-[2rem] lg:col-span-1">
          <h5 className="text-sm font-bold text-white/60 uppercase tracking-wider mb-4 flex items-center gap-2">
            <Star className="w-4 h-4 text-celestial-gold" />
            타지카 연간 지표 (Annual)
          </h5>
          <div className="space-y-3 mb-6">
            <div className="p-3 rounded-xl bg-celestial-cyan/10 border border-celestial-cyan/20">
              <p className="text-[10px] text-celestial-cyan font-black uppercase mb-1">연차 로드 (Year Lord)</p>
              <p className="text-sm font-bold text-white">{yearLord || "N/A"}</p>
            </div>
            <div className="p-3 rounded-xl bg-celestial-purple/10 border border-celestial-purple/20">
              <p className="text-[10px] text-celestial-purple font-black uppercase mb-1">문타 (Muntha Rasi)</p>
              <p className="text-sm font-bold text-white">사인 {munthaRasi || "N/A"}</p>
            </div>
          </div>
          <div className="grid grid-cols-2 gap-2">
            {(sahams as Saham[]).map((s) => (
              <div key={s.name} className="p-2.5 rounded-xl bg-white/[0.03] border border-white/5">
                <p className="text-[9px] text-white/30 font-bold mb-0.5">{s.name}</p>
                <div className="flex items-center justify-between">
                  <span className="text-xs font-mono text-celestial-gold">사인 {s.rasi}</span>
                  {s.name.includes("Fortune") && <Zap className="w-3 h-3 text-emerald-400" />}
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Vargottama / Special Positions */}
        <div className="glass p-8 rounded-[2rem] lg:col-span-2">
          <h5 className="text-sm font-bold text-white/60 uppercase tracking-wider mb-6 flex items-center gap-2">
            <Zap className="w-5 h-5 text-brand-400" />
            바르고타마 및 특수 지위 (Special Planetary Status)
          </h5>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {specialPlanets.map((vi, i) => (
              <div key={i} className="p-4 bg-white/5 rounded-2xl border border-white/10 group hover:border-celestial-gold/50 transition-all">
                <div className="flex items-center justify-between mb-2">
                  <span className="text-white font-black">{vi.planet}</span>
                  <div className="flex gap-1.5">
                    {vi.isVargottama && <span className="text-[9px] bg-celestial-gold/20 text-celestial-gold px-2 py-0.5 rounded font-bold">VARGOTTAMA</span>}
                    {vi.isPushkarNavamsa && <span className="text-[9px] bg-emerald-500/20 text-emerald-400 px-2 py-0.5 rounded font-bold">PUSHKAR</span>}
                  </div>
                </div>
                <p className="text-xs text-white/50 leading-relaxed italic line-clamp-2">
                  "{vi.summary}"
                </p>
              </div>
            ))}
            {specialPlanets.length === 0 && (
              <div className="col-span-2 py-10 text-center border-2 border-dashed border-white/5 rounded-2xl">
                <p className="text-sm text-white/20">이 차트에는 바르고타마 지위를 가진 행성이 없습니다.</p>
              </div>
            )}
          </div>

          {harshaBalaSummary.length > 0 && (
            <div className="mt-6 pt-6 border-t border-white/5">
               <h6 className="text-[10px] text-white/30 font-bold uppercase mb-3">Harsha Bala (진정한 기쁨)</h6>
               <div className="flex flex-wrap gap-2">
                  {(harshaBalaSummary as [string, number][]).map(([planet, score]: [string, number]) => (
                    <div key={planet} className="px-3 py-1 rounded-lg bg-emerald-500/10 border border-emerald-500/20 text-[10px] text-emerald-400">
                      <span className="font-bold">{planet}</span>: {score} pts
                    </div>
                  ))}
               </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
