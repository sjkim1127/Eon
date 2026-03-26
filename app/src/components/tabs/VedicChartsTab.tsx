import { useState } from "react";
import { motion } from "framer-motion";
import { Star, Grid3x3, BarChart3, Zap, Compass, Heart, Clock, Users } from "lucide-react";
import { VARGA_DEFS } from "../../constants";
import { cn } from "../../utils";
import type { VedicAnalysisResult, Yoga } from "../../types";

import { SouthIndianChart } from "../charts/SouthIndianChart";
import { NorthIndianChart } from "../charts/NorthIndianChart";
import { VargaSignPositionsTable } from "../vedic/VargaSignPositionsTable";
import { BavHeatmap } from "../vedic/BavHeatmap";
import { SavScoreChart } from "../vedic/SavScoreChart";
import { VimshopakaTable } from "../vedic/VimshopakaTable";

import { BhavaRadarSection } from "../sections/BhavaRadarSection";
import { AspectsSection } from "../sections/AspectsSection";
import { DashaTimelineSection } from "../sections/DashaTimelineSection";
import { GocharaSection } from "../sections/GocharaSection";
import { AvasthaKarakaSection } from "../sections/AvasthaKarakaSection";
import { VargaNakshatraTable } from "../sections/VargaNakshatraTable";
import { VedicAdvancedInsightsSection } from "../sections/VedicAdvancedInsightsSection";

interface VedicChartsTabProps {
  report: VedicAnalysisResult;
}

export function VedicChartsTab({ report }: VedicChartsTabProps) {
  const [selectedVargaId, setSelectedVargaId] = useState<string>("rasi");
  const [chartStyle, setChartStyle] = useState<"south" | "north">("south");
  const [activeTab, setActiveTab] = useState<"natal" | "annual">("natal");

  if (!report || !report.chart || !report.chart.planets) return null;
  
  const natalChart = report.chart;
  const annualChart = report.annual_chart;
  const currentChart = (activeTab === "annual" && annualChart) ? annualChart : natalChart;

  const planets: any[] = currentChart.planets;
  const ascendant: any = currentChart.ascendant;
  const panchanga = currentChart.panchanga;
  const sav = currentChart.sav;
  const vimshopaka = currentChart.vimshopaka_scores;
  const avasthas = currentChart.avasthas;
  const karakas = currentChart.karakas;
  const arudhaPadas = currentChart.arudha_padas;

  const yogas: Yoga[] = report.report?.yogas ?? [];
  const bhavaStrengths = currentChart.bhava_strengths ?? [];
  const aspects = currentChart.aspects ?? [];
  const dashaTimeline = report.report?.dasha_timeline ?? [];
  const yoginiTimeline = report.report?.yogini_timeline ?? [];

  const vargaReports = report.varga_nakshatra_reports;
  const reportsMap = vargaReports?.reports;

  const ratingLabel = (rating: string) => {
    if (rating === "Excellent") return "최상";
    if (rating === "Strong") return "강함";
    if (rating === "Average") return "보통";
    return "약함";
  };

  const vargaDef = VARGA_DEFS.find((v) => v.id === selectedVargaId) || VARGA_DEFS[0];
  const lagnaRasi = ascendant[vargaDef.key] as number;

  return (
    <motion.div
      key="vedic-charts"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* ── Chart Mode Switch ────────────────────────────────────────── */}
      <div className="flex justify-center">
        <div className="glass p-1.5 rounded-2xl flex gap-1 border-white/5 bg-white/5">
          <button
            onClick={() => setActiveTab("natal")}
            className={cn(
              "px-6 py-2.5 rounded-xl text-sm font-bold transition-all flex items-center gap-2",
              activeTab === "natal" 
                ? "bg-white/10 text-white shadow-[0_0_20px_rgba(255,255,255,0.1)]" 
                : "text-white/30 hover:text-white/50"
            )}
          >
            <Users className="w-4 h-4" />
            Natal Chart (출생)
          </button>
          <button
            onClick={() => setActiveTab("annual")}
            className={cn(
              "px-6 py-2.5 rounded-xl text-sm font-bold transition-all flex items-center gap-2",
              activeTab === "annual" 
                ? "bg-celestial-gold/20 text-celestial-gold shadow-[0_0_20px_rgba(234,179,8,0.1)]" 
                : "text-white/30 hover:text-white/50"
            )}
          >
            <Compass className="w-4 h-4" />
            Annual Chart (연간)
          </button>
        </div>
      </div>

      {/* ── Overview 통합 ─────────────────────────────────────────────── */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
          <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
            <Heart className="w-32 h-32" />
          </div>
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            영혼의 지표 (Atmakaraka)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {report.report?.primary_karakas.atmakaraka}
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
            {report.report?.primary_karakas.amatyakaraka}
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
            {report.report?.primary_karakas.darakaraka}
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            배우자·가까운 파트너와의 관계 패턴을 나타내는 행성입니다.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
          <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
            현재 대운 (Dasha)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {report.report?.dasha_focus.replace("Current Major Period: ", "")}
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
              {Math.round(report.report?.overall_strength_score ?? 0)}
            </h4>
            <span className="text-white/20 font-bold">/ 600</span>
          </div>
          <div className="w-full bg-white/5 h-2 rounded-full overflow-hidden">
            <div
              className="bg-celestial-purple h-full rounded-full transition-all duration-1000"
              style={{ width: `${((report.report?.overall_strength_score ?? 0) / 600) * 100}%` }}
            />
          </div>
        </div>
      </div>

      <div className="glass p-10 rounded-[2.5rem]">
        <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
          <Star className="w-6 h-6 text-celestial-gold" />
          낙샤트라 청사진
        </h5>
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
          <div className="p-6 bg-white/5 rounded-2xl border border-white/10">
            <p className="text-[10px] text-white/40 font-bold uppercase mb-2">Birth Star Details</p>
            <p className="text-lg text-white font-bold leading-tight">
              {report.report?.nakshatra_info}
            </p>
          </div>
          <div className="p-6 bg-celestial-gold/5 rounded-2xl border border-celestial-gold/10">
            <p className="text-[10px] text-celestial-gold/60 font-bold uppercase mb-2">Lunar Phase & Day</p>
            <p className="text-lg text-celestial-gold font-bold">
              {panchanga.tithi_name} ({panchanga.is_day_birth ? "주간" : "야간"} 출생 · {panchanga.vara}요일)
            </p>
          </div>
        </div>
      </div>

      {/* ── 고급 분석 지표 (Jaimini, D9/D10, Tajika) ─────────────────── */}
      {report.report && (
        <VedicAdvancedInsightsSection report={report.report} />
      )}

      {/* ── 요가 (Yogas) 하이라이트 ────────────────────────────────────────── */}
      {yogas.length > 0 && (
        <div className="flex gap-4 overflow-x-auto pb-4 snap-x">
          {yogas.map((yoga: Yoga, idx: number) => {
            const isVeryHigh = yoga.quality === "VeryHigh";
            return (
              <div
                key={idx}
                className={cn(
                  "min-width-[280px] flex-shrink-0 snap-center glass p-6 rounded-2xl border transition-all glass-hover",
                  isVeryHigh ? "border-celestial-gold/30 bg-celestial-gold/5" : "border-white/10 bg-white/5"
                )}
              >
                <div className="flex items-center gap-2 mb-3">
                  <Zap className={cn("w-4 h-4", isVeryHigh ? "text-celestial-gold" : "text-brand-400")} />
                  <span className="text-sm font-bold text-white">{yoga.name}</span>
                  <span className="text-[10px] px-2 py-0.5 rounded-full bg-white/10 text-white/60">
                    {yoga.yoga_type}
                  </span>
                </div>
                <p className="text-xs text-white/60 line-clamp-2 leading-relaxed">
                  {yoga.description}
                </p>
              </div>
            );
          })}
        </div>
      )}

      {/* ── 분할 차트 시각화 (Varga Charts) ─────────────────────────────────── */}
      <section className="glass p-10 rounded-[3rem] border-white/10 relative overflow-hidden">
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-10">
          <div>
            <h5 className="text-2xl font-bold text-white mb-2 flex items-center gap-3">
              <Grid3x3 className="w-7 h-7 text-celestial-gold" />
              분할 차트 시각화 (Varga Charts)
            </h5>
            <p className="text-sm text-white/40">
              인생의 특정 영역을 현미경처럼 확대하여 분석하는 독립적인 차트들입니다.
            </p>
          </div>
          <div className="flex items-center gap-3 flex-wrap">
            <div className="flex bg-black/40 border border-white/20 rounded-xl overflow-hidden p-0.5">
              <button
                onClick={() => setChartStyle("south")}
                className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${chartStyle === "south" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
              >
                남인도
              </button>
              <button
                onClick={() => setChartStyle("north")}
                className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${chartStyle === "north" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
              >
                북인도
              </button>
            </div>
            <select
              value={selectedVargaId}
              onChange={(e) => setSelectedVargaId(e.target.value)}
              className="bg-white/5 border border-white/10 text-white text-xs font-bold rounded-xl px-4 py-2 focus:ring-2 focus:ring-celestial-gold/50 outline-none transition-all cursor-pointer hover:bg-white/10"
            >
              {VARGA_DEFS.map((v) => (
                <option key={v.id} value={v.id} className="bg-slate-900">
                  {v.label} - {v.name}
                </option>
              ))}
            </select>
          </div>
        </div>

        <div className="space-y-4 mb-8">
          <p className="text-[11px] font-bold text-white/20 uppercase tracking-widest flex items-center gap-2">
            <span className="w-8 h-px bg-white/10" />
            현재 선택된 차트: {vargaDef.label} ({vargaDef.name})
            <span className="text-white/30 hidden md:inline">|</span>
            <span className="text-white/30">황금 테두리 = 라그나 · 오른쪽 숫자 = 하우스 번호</span>
          </p>
          <div className="flex flex-col md:flex-row gap-8 items-start">
            {chartStyle === "south" ? (
              <SouthIndianChart
                lagnaRasi={lagnaRasi}
                planetEntries={planets.map((p: any) => ({ name: p.planet, rasi: p[vargaDef.key], retro: p.is_retrograde, deg: p.sidereal_deg }))}
              />
            ) : (
              <NorthIndianChart
                lagnaRasi={lagnaRasi}
                planetEntries={planets.map((p: any) => ({ name: p.planet, rasi: p[vargaDef.key], retro: p.is_retrograde, deg: p.sidereal_deg }))}
              />
            )}
            <div className="flex-1 w-full overflow-x-auto">
              <VargaNakshatraTable 
                title={`${vargaDef.label} 낙샤트라 상세`} 
                vargaLabel={vargaDef.label} 
                rows={reportsMap?.[selectedVargaId]?.rows || []} 
              />
            </div>
          </div>
        </div>

        <VargaSignPositionsTable planets={planets} ascendant={ascendant} />
      </section>

      <section>
        <h5 className="text-xl font-bold text-white mb-6">하우스(Bhava)별 에너지 역량 상세</h5>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
          {report.report?.house_summary.map((house: any) => {
            const bhava = bhavaStrengths.find((b: any) => b.house === house.house);
            return (
              <div
                key={house.house}
                className="glass p-6 rounded-2xl text-center glass-hover cursor-help"
              >
                <p className="text-[10px] text-white/40 font-bold uppercase mb-1">House {house.house}</p>
                <p className={cn("text-xs font-black mb-3",
                  house.rating === "Excellent" ? "text-emerald-400" :
                    house.rating === "Strong" ? "text-celestial-cyan" :
                      house.rating === "Average" ? "text-white/60" : "text-orange-400"
                )}>
                  {ratingLabel(house.rating)}
                </p>
                <div className="flex flex-col gap-1.5 mt-4">
                  <div className="flex justify-between text-[9px]">
                    <span className="text-white/30">로드</span>
                    <span className="text-white/60 font-bold">{bhava?.lord_score?.toFixed(0)}</span>
                  </div>
                  <div className="flex justify-between text-[9px]">
                    <span className="text-white/30">방위</span>
                    <span className="text-white/60 font-bold">{bhava?.dig_score?.toFixed(0)}</span>
                  </div>
                  <div className="w-full bg-white/5 h-1 rounded-full overflow-hidden mt-1">
                    <div
                      className={cn("h-full rounded-full transition-all duration-1000",
                        house.rating === "Excellent" ? "bg-emerald-500" :
                          house.rating === "Strong" ? "bg-celestial-cyan" : "bg-white/20"
                      )}
                      style={{ width: `${(house.total_score / 600) * 100}%` }}
                    />
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      </section>

      {/* ── 행성별 빈나슈타카바르가 (BAV) 히트맵 ────────────── */}
      {currentChart.bav && currentChart.bav.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
            <BarChart3 className="w-6 h-6 text-celestial-gold" />
            빈나슈타카바르가 (BAV) — 행성×하우스 빈두 히트맵
          </h5>
          <p className="text-xs text-white/40 mb-6">
            각 행성이 12개 하우스에 기여하는 빈두 포인트입니다. 녹색에 가까울수록 강한 하우스를 지원합니다.
          </p>
          <BavHeatmap bav={currentChart.bav} savPoints={currentChart.sav?.points as number[]} />
        </div>
      )}

      {/* ── SAV (Sarvashtakavarga) 12하우스 점수 ────────────── */}
      {sav?.points && sav.points.length === 12 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Grid3x3 className="w-6 h-6 text-celestial-cyan" />
            사르바아슈타카바르가 (SAV) — 12하우스 합산 점수
          </h5>
          <SavScoreChart points={sav.points} />
        </div>
      )}

      {/* ── 빔쇼파카 발라 (Vimshopaka) ────────────── */}
      {vimshopaka && vimshopaka.length > 0 && (
        <VimshopakaTable vimshopaka={vimshopaka} />
      )}

      {/* ── 다샤 타임라인 (Vimshottari & Yogini Dasha) ───────────── */}
      <DashaTimelineSection 
        periods={dashaTimeline} 
        yoginiPeriods={yoginiTimeline}
      />

      {/* ── 12하우스 강도 레이더 (Bhava Strength) ───────────── */}
      <BhavaRadarSection strengths={bhavaStrengths} />

      {/* ── 행성 시선 (Aspects / Drishti) ─────────────────── */}
      <AspectsSection aspects={aspects} />

      {/* ── 고차라 트랜싯 (Gochara) ─────────────────── */}
      <GocharaSection summary={(report as any).gochara ?? null} />

      {/* ── 카라카 + 아바스타 (Karakas + Avasthas) ────────── */}
      <AvasthaKarakaSection
        avasthas={avasthas ?? []}
        karakas={karakas ?? []}
        arudhaPadas={arudhaPadas ?? []}
      />
    </motion.div>
  );
}
