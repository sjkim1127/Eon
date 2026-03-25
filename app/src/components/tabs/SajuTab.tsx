import { motion } from "framer-motion";
import { Activity, AlertTriangle } from "lucide-react";

import { PillarsChart } from "../saju/PillarsChart";
import { SajuSummary } from "../saju/SajuSummary";
import { PowerAnalysis } from "../saju/PowerAnalysis";
import { SpiritsList } from "../saju/SpiritsList";
import { RelationshipsAnalysis } from "../saju/RelationshipsAnalysis";
import { VoidAnalysis } from "../saju/VoidAnalysis";
import { MajorLuckTimeline } from "../saju/MajorLuckTimeline";

interface SajuTabProps {
  sajuReport: import("../../types").SajuAnalysisResult | null;
  unknownTime?: boolean;
}

export function SajuTab({ sajuReport, unknownTime = false }: SajuTabProps) {

  if (!sajuReport || !sajuReport.report) return null;
  const reportData = sajuReport.report;
  const p = reportData.pillars;
  const t = reportData.ten_gods;
  const s = reportData.strength;
  const y = reportData.yongshin;
  const st = reportData.structure;
  const sp = reportData.spirit_markers;
  const ml = reportData.major_luck;

  return (
    <motion.div
      key="saju-results"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* 시간 미상 경고 배너 */}
      {unknownTime && (
        <div className="flex items-center gap-3 px-5 py-3 rounded-2xl bg-amber-500/10 border border-amber-500/25 text-amber-300 text-sm">
          <AlertTriangle className="w-4 h-4 shrink-0 text-amber-400" />
          <span>
            <strong>시주(時柱) 미상</strong> — 정오(12:00)기준 분석. 시주 및 시 기반 신살·용신은 참고용입니다.
          </span>
        </div>
      )}

      {/* 사주팔자 차트 */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
          <Activity className="w-6 h-6 text-celestial-gold" />
          사주팔자 (四柱八字)
        </h5>
        <PillarsChart p={p} t={t} unknownTime={unknownTime} />
      </div>

      {/* 역량 + 용신 + 격국 */}
      <SajuSummary s={s} y={y} st={st} />

      {/* 오행과 십성 분석 (Power) */}
      <PowerAnalysis power={reportData.power} />

      {/* 신살 */}
      <SpiritsList sp={sp} />

      {/* 합충형해 (合沖刑害) 분석 */}
      <RelationshipsAnalysis relationships={sajuReport.relationships} />

      {/* 공망 (空亡) 분석 */}
      <VoidAnalysis voidAnalysis={sajuReport.void_analysis ?? sajuReport.report?.voids} />

      {/* 대운 */}
      <MajorLuckTimeline ml={ml} />

    </motion.div>
  );
}
