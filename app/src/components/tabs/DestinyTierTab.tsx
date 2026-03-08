import { motion } from "framer-motion";
import { Trophy } from "lucide-react";
import type { SajuAnalysisResult, VedicAnalysisResult, TransitResult } from "../../types";
import { computeTierResult } from "../../utils/tierScore";
import { TierCard } from "../destiny/TierCard";
import { RadarChartSection } from "../destiny/RadarChartSection";
import { ScoreBreakdown } from "../destiny/ScoreBreakdown";
import { StrengthsWeaknesses } from "../destiny/StrengthsWeaknesses";
import { InsightsList } from "../destiny/InsightsList";

export interface DestinyTierTabProps {
  sajuReport: SajuAnalysisResult | null;
  report: VedicAnalysisResult | null;
  transitReport?: TransitResult | null;
  unknownTime?: boolean;
}

export function DestinyTierTab({ sajuReport, report, transitReport, unknownTime }: DestinyTierTabProps) {
  const result = computeTierResult(sajuReport, report, transitReport);

  if (!result) {
    return (
      <motion.div
        key="destiny-tier-empty"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="flex flex-col items-center justify-center py-24 text-center"
      >
        <Trophy className="w-16 h-16 text-white/20 mb-4" />
        <h3 className="text-xl font-bold text-white mb-2">운명 티어 분석</h3>
        <p className="text-white/50 max-w-sm">
          출생 정보를 입력하고 통합 분석을 실행하면, 사주와 베딕을 종합한 운명·잠재력·분야별 티어를 확인할 수 있습니다.
        </p>
      </motion.div>
    );
  }

  const { domainTiers, sajuResult, vedicResult, transitResult, strengths, weaknesses } = result;

  const hasSaju = !!sajuReport?.report;
  const hasVedic = !!report?.report;
  const hasTransit = !!transitReport?.current_frame;

  // ── 점수 분해: 사주 ──
  const sajuSt = sajuReport?.report?.strength;
  const strengthNorm = Math.min(100, (sajuSt?.strength_score ?? 0) * 2);
  const rawSupportRatioUi = sajuSt?.deuk_se?.support_ratio ?? 0;
  const supportPctUi = rawSupportRatioUi > 1 ? rawSupportRatioUi : rawSupportRatioUi * 100;
  const deukSum = (sajuSt?.deuk_ryeong?.acquired ? 8 : 0)
    + (sajuSt?.deuk_ji?.acquired ? 8 : 0)
    + (sajuSt?.deuk_si?.acquired ? 6 : 0)
    + (supportPctUi / 100) * 15;
  const throughput = sajuReport?.qi_topology?.throughput ?? 0;
  const goldenTimePt = sajuReport?.report?.golden_time ? 10 : 0;
  const stabilityGrade = sajuReport?.complexity?.stability_grade ?? "";
  const stabilityPt = /^[AB]$/.test(stabilityGrade) || /A|B/.test(stabilityGrade) ? 4 : 0;

  // ── 점수 분해: 베딕 ──
  const vr = report?.report;
  const planetStrengthNorm = Math.min(50, ((vr?.overall_strength_score ?? 0) / 6) * 0.5);
  const highYogaCount = (vr?.yogas ?? []).filter((y: { quality: string | object }) => {
    const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
    return q === "VeryHigh" || q === "High";
  }).length;
  const yogaPt = Math.min(20, highYogaCount * 5);
  const strongHouses = (vr?.house_summary ?? []).filter((h: { rating: string }) =>
    h.rating === "Excellent" || h.rating === "Strong"
  ).length;
  const housePt = (strongHouses / 12) * 20;
  const satiPt = vr?.sade_sati === "None" ? 5 : (vr?.sade_sati === "Peak" || vr?.sade_sati === "Rising" ? 0 : 2.5);
  const BENEFIC_PLANETS = ["Jupiter", "Venus", "Mercury", "Moon"];
  const dashaPt = BENEFIC_PLANETS.some((p) => (vr?.dasha_focus ?? "").includes(p)) ? 4 : 0;

  return (
    <motion.div
      key="destiny-tier"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {unknownTime && (
        <div className="flex items-center gap-3 px-5 py-3 rounded-2xl bg-amber-500/10 border border-amber-500/25 text-amber-300 text-sm">
          <span>시주 미상 시 티어 산정은 시 기반 항목을 제외하고 계산됩니다.</span>
        </div>
      )}

      {/* ── 메인 티어 카드 & 운명/잠재력 카드 ── */}
      <TierCard result={result} hasTransit={hasTransit} transitResult={transitResult} />

      {/* ── 분야별 티어 레이더 차트 ── */}
      <RadarChartSection domainTiers={domainTiers} />

      {/* ── 사주/베딕 점수 세부 분해 ── */}
      <ScoreBreakdown
        hasSaju={hasSaju} hasVedic={hasVedic}
        sajuResult={sajuResult} vedicResult={vedicResult}
        strengthNorm={strengthNorm} deukSum={deukSum} throughput={throughput} goldenTimePt={goldenTimePt} stabilityPt={stabilityPt}
        planetStrengthNorm={planetStrengthNorm} highYogaCount={highYogaCount} yogaPt={yogaPt} strongHouses={strongHouses} housePt={housePt} satiPt={satiPt} dashaPt={dashaPt}
      />

      {/* ── 강점·약점 ── */}
      <StrengthsWeaknesses strengths={strengths} weaknesses={weaknesses} />

      {/* ── 종합 인사이트 ── */}
      <InsightsList result={result} />
    </motion.div>
  );
}
