import { motion } from "framer-motion";
import { LifeGraphSection } from "../sections/saju/LifeGraphSection";
import { SimulationTimelineSection } from "../sections/saju/SimulationTimelineSection";
import { VulnerabilitySection } from "../sections/common/VulnerabilitySection";
import type { SajuAnalysisResult, VedicAnalysisResult } from "../../types";

interface SimulationTabProps {
  sajuReport: SajuAnalysisResult | null;
  vedicReport: VedicAnalysisResult | null;
}

export function SimulationTab({ sajuReport, vedicReport }: SimulationTabProps) {
  if (!sajuReport || !sajuReport.report) return null;
  
  const reportData = sajuReport.report;
  const gt = reportData.goldenTime;

  return (
    <motion.div
      key="simulation-results"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* 인생 흐름 그래프 + 골든타임 */}
      <LifeGraphSection
        timeline={reportData.timeline ?? []}
        goldenTime={gt}
        simulationFrames={reportData.simulationFrames}
        vedicReport={vedicReport}
      />

      {/* 생애 시뮬레이션 타임라인 */}
      <SimulationTimelineSection frames={reportData.simulationFrames ?? []} />

      {/* 주의가 필요한 시기 (대운·세운 전수 조사) */}
      <VulnerabilitySection report={sajuReport.vulnerabilityReport ?? null} />
    </motion.div>
  );
}
