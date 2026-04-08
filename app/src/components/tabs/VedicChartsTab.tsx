import { useState } from "react";
import { motion } from "framer-motion";
import { Compass, Users, Star } from "lucide-react";
import { cn } from "../../utils";
import type { VedicAnalysisResult } from "../../types";

import { BhavaRadarSection } from "../sections/vedic/BhavaRadarSection";
import { AspectsSection } from "../sections/vedic/AspectsSection";
import { DashaTimelineSection } from "../sections/vedic/DashaTimelineSection";
import { GocharaSection } from "../sections/vedic/GocharaSection";
import { AvasthaKarakaSection } from "../sections/vedic/AvasthaKarakaSection";
import { VedicAdvancedInsightsSection } from "../sections/vedic/VedicAdvancedInsightsSection";
import { VedicOverviewSection } from "../sections/vedic/VedicOverviewSection";
import { VedicPanchangaSection } from "../sections/vedic/VedicPanchangaSection";
import { VedicYogaHighlightSection } from "../sections/vedic/VedicYogaHighlightSection";
import { VargaVisualizationSection } from "../sections/vedic/VargaVisualizationSection";
import { HouseStrengthGrid } from "../sections/vedic/HouseStrengthGrid";
import { VedicMetricsGrid } from "../sections/vedic/VedicMetricsGrid";

interface VedicChartsTabProps {
  report: VedicAnalysisResult;
}

export function VedicChartsTab({ report }: VedicChartsTabProps) {
  const [activeTab, setActiveTab] = useState<"natal" | "annual">("natal");

  if (!report || !report.chart || !report.chart.planets) return null;
  
  const natalChart = report.chart;
  const annualChart = report.annualChart;
  const currentChart = (activeTab === "annual" && annualChart) ? annualChart : natalChart;

  const planets = currentChart.planets;
  const ascendant = currentChart.ascendant;
  const panchanga = currentChart.panchanga;
  const rr = report.report;
  const dashaTimeline = rr?.dashaTimeline ?? [];
  const yoginiTimeline = rr?.yoginiTimeline ?? [];
  const charaDashaTimeline = rr?.charaDashaTimeline ?? [];

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

      {activeTab === "annual" && (
        <motion.div 
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          className="p-6 rounded-[2rem] bg-celestial-gold/5 border border-celestial-gold/20 text-center"
        >
          <h3 className="text-xl font-bold text-celestial-gold flex items-center justify-center gap-2">
            <Star className="w-5 h-5" /> ✨ 연간 차트 (Solar Return) 모드
          </h3>
          <p className="text-sm text-celestial-gold/70 mt-3 font-semibold">
            하단에 표기되는 <strong>분할 차트, 하우스 점수, 세부 행성 포지션</strong>이 출생 기준이 아닌 '올해 태양 위치'를 기준으로 재계산되어 표시 중입니다.
          </p>
        </motion.div>
      )}

      {/* ── Overview 통합 (Karakas, Dasha Focus, Strength) ─────────────── */}
      {report.report && <VedicOverviewSection report={report.report} />}

      {/* ── Nakshatra & Panchanga ─────────────────────────────────────── */}
      <VedicPanchangaSection 
        panchanga={panchanga} 
        nakshatraInfo={rr?.nakshatraInfo ?? "N/A"} 
      />

      {/* ── 고급 분석 지표 (Jaimini, D9/D10, Tajika) ─────────────────── */}
      {report.report && (
        <VedicAdvancedInsightsSection 
          report={report.report} 
          tajikaReport={report.tajikaReport ?? null}
        />
      )}

      {/* ── 요가 (Yogas) 하이라이트 ────────────────────────────────────────── */}
      <VedicYogaHighlightSection yogas={report.report?.yogas || []} />

      {/* ── 분할 차트 시각화 (Varga Charts) ─────────────────────────────────── */}
      <VargaVisualizationSection 
        planets={planets} 
        ascendant={ascendant} 
        vargaNakshatraReportsMap={report.vargaNakshatraReports?.reports}
      />

      {/* ── 하우스별 세부 지표 (Bhavas, Ashtakavarga) ──────────── */}
      <HouseStrengthGrid 
        houseSummary={rr?.houseSummary ?? []}
        bhavaStrengths={currentChart?.bhavaStrengths ?? []} 
      />

      {/* ── 하우스 점령 및 주인 (Occupation) ──────────── */}
      <VedicMetricsGrid 
        bav={currentChart?.bav || []} 
        savPoints={currentChart?.sav?.points ?? []} 
        vimshopaka={currentChart?.vimshopakaScores ?? []}
      />
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* ── 고차라 트랜싯 / 아스펙트 ─────────────────────────── */}
        <GocharaSection summary={report.gochara} />
        <AspectsSection aspects={currentChart?.aspects ?? []} />
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
        {/* ── 행성별 상태 (Avasthas) ─────────── */}
        <AvasthaKarakaSection
            avasthas={currentChart.avasthas ?? []}
            karakas={rr?.allKarakas ?? currentChart.karakas ?? []}
            arudhaPadas={currentChart.arudhaPadas ?? []}
        />

        {/* ── 다샤 다이어그램 (Dasha Timeline) ───────── */}
        <DashaTimelineSection 
          periods={dashaTimeline}
          yoginiPeriods={yoginiTimeline}
          charaPeriods={charaDashaTimeline}
        />
      </div>

      {/* ── 하우스 분석 상세 (Bhava Analysis / Radar) ───────────────── */}
      {currentChart && currentChart.bhavaStrengths && <BhavaRadarSection strengths={currentChart.bhavaStrengths} />}

    </motion.div>
  );
}
