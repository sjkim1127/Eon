import { useState } from "react";
import { motion } from "framer-motion";
import { Compass, Users } from "lucide-react";
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
  const annualChart = report.annual_chart;
  const currentChart = (activeTab === "annual" && annualChart) ? annualChart : natalChart;

  const planets = currentChart.planets;
  const ascendant = currentChart.ascendant;
  const panchanga = currentChart.panchanga;
  const dashaTimeline = report.report?.dasha_timeline ?? [];
  const yoginiTimeline = report.report?.yogini_timeline ?? [];
  const charaDashaTimeline = report.report?.chara_dasha_timeline ?? [];

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

      {/* ── Overview 통합 (Karakas, Dasha Focus, Strength) ─────────────── */}
      {report.report && <VedicOverviewSection report={report.report} />}

      {/* ── Nakshatra & Panchanga ─────────────────────────────────────── */}
      <VedicPanchangaSection 
        panchanga={panchanga} 
        nakshatraInfo={report.report?.nakshatra_info ?? "N/A"} 
      />

      {/* ── 고급 분석 지표 (Jaimini, D9/D10, Tajika) ─────────────────── */}
      {report.report && (
        <VedicAdvancedInsightsSection 
          report={report.report} 
          tajikaReport={report.tajika_report}
        />
      )}

      {/* ── 요가 (Yogas) 하이라이트 ────────────────────────────────────────── */}
      <VedicYogaHighlightSection yogas={report.report?.yogas ?? []} />

      {/* ── 분할 차트 시각화 (Varga Charts) ─────────────────────────────────── */}
      <VargaVisualizationSection 
        planets={planets} 
        ascendant={ascendant} 
        vargaNakshatraReportsMap={report.varga_nakshatra_reports?.reports} 
      />

      {/* ── 하우스(Bhava) 에너지 상세 ────────────────────────────────── */}
      <HouseStrengthGrid 
        houseSummary={report.report?.house_summary ?? []} 
        bhavaStrengths={currentChart.bhava_strengths ?? []} 
      />

      {/* ── BAV Heatmap, SAV, Vimshopaka ───────────────────────────── */}
      <VedicMetricsGrid 
        bav={currentChart.bav ?? []} 
        savPoints={currentChart.sav?.points ?? []} 
        vimshopaka={currentChart.vimshopaka_scores ?? []} 
      />

      {/* ── 다샤 타임라인 (Vimshottari, Yogini, Chara) ───────────── */}
      <DashaTimelineSection 
        periods={dashaTimeline} 
        yoginiPeriods={yoginiTimeline}
        charaPeriods={charaDashaTimeline}
      />

      {/* ── 12하우스 강도 레이더 (Bhava Strength) ───────────── */}
      <BhavaRadarSection strengths={currentChart.bhava_strengths ?? []} />

      {/* ── 행성 시선 (Aspects / Drishti) ─────────────────── */}
      <AspectsSection aspects={currentChart.aspects ?? []} />

      {/* ── 고차라 트랜싯 (Gochara) ─────────────────── */}
      <GocharaSection summary={report.gochara ?? null} />

      {/* ── 카라카 + 아바스타 (Karakas + Avasthas) ────────── */}
      <AvasthaKarakaSection
        avasthas={currentChart.avasthas ?? []}
        karakas={report.report?.all_karakas ?? currentChart.karakas ?? []}
        arudhaPadas={currentChart.arudha_padas ?? []}
      />
    </motion.div>
  );
}
