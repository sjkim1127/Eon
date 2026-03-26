import { motion } from "framer-motion";
import { Activity, AlertTriangle } from "lucide-react";

import { PillarsChart } from "../saju/PillarsChart";
import { SajuSummary } from "../saju/SajuSummary";
import { PowerAnalysis } from "../saju/PowerAnalysis";
import { SpiritsList } from "../saju/SpiritsList";
import { RelationshipsAnalysis } from "../saju/RelationshipsAnalysis";
import { VoidAnalysis } from "../saju/VoidAnalysis";
import { MajorLuckTimeline } from "../saju/MajorLuckTimeline";
import { AuxiliaryPillars } from "../saju/AuxiliaryPillars";

interface SajuTabProps {
  sajuReport: import("../../types").SajuAnalysisResult | null;
  unknownTime?: boolean;
}

function RelationshipBadges({ relationships }: { relationships: any }) {
  const summary = [];
  if ((relationships.stem_clashes?.length || 0) + (relationships.branch_clashes?.length || 0) > 0) 
    summary.push({ label: "충(沖)", color: "text-rose-400 bg-rose-500/10 border-rose-500/20" });
  if ((relationships.branch_punishments?.length || 0) > 0) 
    summary.push({ label: "형(刑)", color: "text-orange-400 bg-orange-500/10 border-orange-500/20" });
  if ((relationships.stem_combinations?.length || 0) + (relationships.six_combinations?.length || 0) + (relationships.triple_combinations?.length || 0) + (relationships.seasonal_combinations?.length || 0) > 0) 
    summary.push({ label: "합(合)", color: "text-emerald-400 bg-emerald-500/10 border-emerald-500/20" });
  if ((relationships.branch_harms?.length || 0) > 0) 
    summary.push({ label: "해(害)", color: "text-pink-400 bg-pink-500/10 border-pink-500/20" });
  if ((relationships.branch_destructions?.length || 0) > 0) 
    summary.push({ label: "파(破)", color: "text-fuchsia-400 bg-fuchsia-500/10 border-fuchsia-500/20" });

  if (summary.length === 0) return null;

  return (
    <div className="flex flex-wrap gap-1.5">
      {summary.map(s => (
        <span key={s.label} className={`text-[10px] font-bold px-2 py-0.5 rounded-md border ${s.color}`}>
          {s.label}
        </span>
      ))}
    </div>
  );
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
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
          <h5 className="text-xl font-bold text-white flex items-center gap-3">
            <Activity className="w-6 h-6 text-celestial-gold" />
            사주팔자 (四柱八字)
          </h5>
          
          {/* 합충형해 요약 배지 */}
          {!!sajuReport.relationships && <RelationshipBadges relationships={sajuReport.relationships as any} />}
        </div>
        <PillarsChart p={p} t={t} unknownTime={unknownTime} />
      </div>

      {/* 보조 기둥 (태원/명궁/신궁) */}
      {reportData.supplementary_pillars && (
        <AuxiliaryPillars 
          data={reportData.supplementary_pillars} 
          auxShinsals={sp?.aux_shinsals}
          unknownTime={unknownTime} 
        />
      )}

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
