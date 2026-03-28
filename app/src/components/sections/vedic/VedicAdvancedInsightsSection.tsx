import { Compass, Briefcase, Heart, Star, Zap } from "lucide-react";
import type { VedicAnalysisReport, TajikaReport, Saham } from "../../../types/vedic";

interface VedicAdvancedInsightsSectionProps {
  report: VedicAnalysisReport;
  tajikaReport?: TajikaReport | null;
}

export function VedicAdvancedInsightsSection({ report, tajikaReport }: VedicAdvancedInsightsSectionProps) {
  const {
    arudha_lagna,
    upapada_lagna,
    special_lagnas_summary,
    d9_marriage_analysis,
    d10_career_analysis,
    varga_interpretations,
  } = report;

  const sahams = tajikaReport?.sahams ?? [];
  const harsha_bala_summary = tajikaReport?.harsha_bala_summary ?? [];
  const year_lord = tajikaReport?.year_lord;
  const muntha_rasi = tajikaReport?.muntha_rasi;

  // Filter special planets
  const specialPlanets = varga_interpretations?.filter(vi => vi.is_vargottama || vi.is_pushkar_navamsa) || [];

  return (
    <div className="space-y-8">
      {/* ── Jaimini & Special Lagnas ──────────────────────────────── */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
          <Compass className="w-6 h-6 text-celestial-gold" />
          제미니 지표 및 특별 라그나 (Jaimini & Special Lagnas)
        </h5>
        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-4 gap-4">
          {arudha_lagna > 0 && (
            <div className="p-4 bg-white/5 rounded-xl border border-white/10 text-center">
              <p className="text-[10px] text-white/40 font-bold uppercase mb-1">Arudha Lagna (AL)</p>
              <p className="text-lg font-bold text-celestial-gold">사인 {arudha_lagna}</p>
              <p className="text-[10px] text-white/30 mt-1">사회적 이미지·외부적 자아</p>
            </div>
          )}
          {upapada_lagna > 0 && (
            <div className="p-4 bg-white/5 rounded-xl border border-white/10 text-center">
              <p className="text-[10px] text-white/40 font-bold uppercase mb-1">Upapada Lagna (UL)</p>
              <p className="text-lg font-bold text-pink-400">사인 {upapada_lagna}</p>
              <p className="text-[10px] text-white/30 mt-1">배우자·결혼 생활의 기운</p>
            </div>
          )}
          {special_lagnas_summary?.map(([name, rasi]) => (
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
            {d9_marriage_analysis}
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
            {d10_career_analysis}
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
              <p className="text-sm font-bold text-white">{year_lord || "N/A"}</p>
            </div>
            <div className="p-3 rounded-xl bg-amber-500/10 border border-amber-500/20">
              <p className="text-[10px] text-amber-400 font-black uppercase mb-1">문타 (Muntha)</p>
              <p className="text-sm font-bold text-white">사인 {muntha_rasi}</p>
            </div>
          </div>
          <div className="space-y-3">
            <p className="text-[10px] text-white/40 font-bold uppercase mb-2">주요 사함 (Sahams)</p>
            {sahams?.map((s: Saham) => (
              <div key={s.name} className="flex items-center justify-between p-3 rounded-xl bg-white/5 border border-white/5">
                <span className="text-xs font-bold text-white/70">{s.name}</span>
                <span className="text-xs font-mono text-celestial-gold">사인 {s.rasi}</span>
              </div>
            ))}
          </div>
        </div>

        {/* Special Status Planets */}
        <div className="glass p-6 rounded-[2rem] lg:col-span-2">
          <h5 className="text-sm font-bold text-white/60 uppercase tracking-wider mb-4 flex items-center gap-2">
            <Zap className="w-4 h-4 text-brand-400" />
            특별 행성 지위 (Vargottama / Pushkar)
          </h5>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {specialPlanets.length > 0 ? (
              specialPlanets.map((vi) => (
                <div key={vi.planet} className="p-5 rounded-2xl bg-brand-400/5 border border-brand-400/20 flex flex-col h-full">
                  <div className="flex justify-between items-start mb-3">
                    <p className="text-sm font-bold text-white tracking-tight">{vi.planet}</p>
                    <div className="flex gap-1.5">
                      {vi.is_vargottama && (
                        <span className="text-[9px] px-2 py-0.5 rounded-full bg-violet-500/20 text-violet-300 border border-violet-500/30 font-black uppercase">Vargottama</span>
                      )}
                      {vi.is_pushkar_navamsa && (
                        <span className="text-[9px] px-2 py-0.5 rounded-full bg-celestial-gold/20 text-celestial-gold border border-celestial-gold/30 font-black uppercase">Pushkar</span>
                      )}
                    </div>
                  </div>
                  
                  <h6 className="text-[11px] font-bold text-white/80 mb-1">{vi.summary}</h6>
                  <p className="text-[10px] text-white/40 leading-relaxed mb-4 flex-1">
                    {vi.description}
                  </p>

                  <div className="space-y-1 mt-auto pt-3 border-t border-white/5">
                    {vi.reasons.map((reason, idx) => (
                      <div key={idx} className="flex items-center gap-2 text-[9px] text-brand-400/60">
                        <Zap className="w-2 h-2" />
                        <span>{reason}</span>
                      </div>
                    ))}
                  </div>
                </div>
              ))
            ) : (
              <p className="text-xs text-white/30 italic">특별한 지위에 있는 행성이 발견되지 않았습니다.</p>
            )}
          </div>
          
          <div className="mt-6">
            <h6 className="text-[10px] font-bold text-white/40 uppercase mb-3">Harsha Bala (기쁨의 강도)</h6>
            <div className="flex flex-wrap gap-2">
              {harsha_bala_summary?.filter(([_, score]: [string, number]) => score >= 5).map(([planet, score]: [string, number]) => (
                <div key={planet} className="px-3 py-1.5 rounded-lg bg-white/5 border border-white/10 flex items-center gap-2">
                  <span className="text-xs font-bold text-white/80">{planet}</span>
                  <span className={`text-[10px] font-black ${score >= 15 ? 'text-green-400' : score >= 10 ? 'text-celestial-cyan' : 'text-white/40'}`}>{score}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
