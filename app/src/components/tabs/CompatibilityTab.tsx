import { motion } from "framer-motion";
import { Heart } from "lucide-react";
import { ASHTA_LABELS, ASHTA_MAX } from "../../constants";
import { BirthInputForm } from "../shared/BirthInputForm";
import type { BirthData, CompReport } from "../../types";

interface CompatibilityTabProps {
  birthData2: BirthData;
  setBirthData2: React.Dispatch<React.SetStateAction<BirthData>>;
  isMale2: boolean;
  setIsMale2: (v: boolean) => void;
  selectedCity2: string;
  onCitySelect2: (city: { name: string; lat: number; lon: number; timezone: string }) => void;
  compReport: CompReport | null;
  compLoading: boolean;
  onRunCompatibility: () => void;
}

export function CompatibilityTab({
  birthData2,
  setBirthData2,
  isMale2,
  setIsMale2,
  selectedCity2,
  onCitySelect2,
  compReport,
  compLoading,
  onRunCompatibility,
}: CompatibilityTabProps) {
  return (
    <motion.div key="compatibility" initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -20 }} className="space-y-8">
      <h2 className="text-2xl font-bold text-white">궁합 분석</h2>

      {/* 두 번째 사람 입력 폼 */}
      <div className="glass p-6 rounded-2xl">
        <h3 className="text-lg font-bold text-white mb-4">상대방 출생 정보</h3>
        <BirthInputForm
          birthData={birthData2}
          setBirthData={setBirthData2}
          selectedCity={selectedCity2}
          onCitySelect={onCitySelect2}
          isMale={isMale2}
          setIsMale={setIsMale2}
          isDST={false}
          loading={compLoading}
          onAnalysis={onRunCompatibility}
          sajuReport={null}
          compact
          submitLabel="궁합 분석 시작"
          hideUnknownTime
        />
      </div>

      {!compReport && (
        <div className="h-40 flex items-center justify-center text-white/40">
          <Heart className="w-6 h-6 mr-2" /> 출생 정보를 입력하고 궁합 분석을 시작하세요
        </div>
      )}

      {compReport && (
        <>
          {/* 사주 궁합 점수 */}
          {compReport.saju && (
            <div className="glass p-6 rounded-2xl">
              <h3 className="text-lg font-bold text-white mb-4">사주 궁합 결과</h3>
              <div className="flex items-center gap-6 mb-6">
                <div className="text-6xl font-bold" style={{ color: compReport.saju.sync_score > 70 ? "#4ade80" : compReport.saju.sync_score > 40 ? "#facc15" : "#f87171" }}>
                  {compReport.saju.sync_score?.toFixed(0) ?? "--"}
                </div>
                <div className="text-white/60 text-sm">/ 100점<br />궁합 점수</div>
              </div>
              {compReport.saju.synergies?.length > 0 && (
                <div className="mb-4">
                  <p className="text-xs font-bold text-green-400 uppercase mb-2">시너지 {compReport.saju.synergies.length}개</p>
                  <div className="space-y-1">{compReport.saju.synergies.slice(0, 5).map((s: string, i: number) => (<div key={i} className="text-xs text-green-300 bg-green-500/10 rounded-lg px-3 py-1.5">{s}</div>))}</div>
                </div>
              )}
              {compReport.saju.conflicts?.length > 0 && (
                <div className="mb-4">
                  <p className="text-xs font-bold text-red-400 uppercase mb-2">충돌 {compReport.saju.conflicts.length}개</p>
                  <div className="space-y-1">{compReport.saju.conflicts.slice(0, 5).map((s: string, i: number) => (<div key={i} className="text-xs text-red-300 bg-red-500/10 rounded-lg px-3 py-1.5">{s}</div>))}</div>
                </div>
              )}
              {compReport.saju.deadlocks?.length > 0 && (
                <div>
                  <p className="text-xs font-bold text-amber-400 uppercase mb-2">갈등 요소 {compReport.saju.deadlocks.length}개</p>
                  <div className="space-y-1">{compReport.saju.deadlocks.slice(0, 3).map((s: string, i: number) => (<div key={i} className="text-xs text-amber-300 bg-amber-500/10 rounded-lg px-3 py-1.5">{s}</div>))}</div>
                </div>
              )}
            </div>
          )}

          {/* 베딕 궁합 Ashta Kuta */}
          {compReport.vedic && (
            <div className="glass p-6 rounded-2xl">
              <div className="flex items-center justify-between mb-4">
                <h3 className="text-lg font-bold text-white">베딕 궁합 (Ashta Kuta)</h3>
                <div className="text-2xl font-bold text-celestial-cyan">
                  {compReport.vedic.total_score?.toFixed(1) ?? "--"} <span className="text-sm text-white/40">/ 36점</span>
                </div>
              </div>
              {compReport.vedic.message && (
                <p className="text-sm text-white/60 mb-4 italic">{compReport.vedic.message}</p>
              )}
              <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                {Object.entries(ASHTA_LABELS).map(([key, label]) => {
                  const val = (compReport.vedic as unknown as Record<string, number | undefined>)[key];
                  const max = ASHTA_MAX[key];
                  const pct = val != null && typeof val === "number" ? (val / max) * 100 : 0;
                  return (
                    <div key={key} className="bg-white/5 rounded-xl p-3">
                      <p className="text-xs text-brand-400 mb-1">{label}</p>
                      <div className="flex items-end gap-1 mb-2">
                        <span className="text-xl font-bold text-white">{typeof val === "number" ? val.toFixed(1) : val ?? "--"}</span>
                        <span className="text-xs text-white/40 mb-0.5">/ {max}</span>
                      </div>
                      <div className="w-full bg-white/10 rounded-full h-1.5">
                        <div className="h-1.5 rounded-full bg-gradient-to-r from-celestial-purple to-celestial-cyan transition-all" style={{ width: `${pct}%` }} />
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>
          )}
        </>
      )}
    </motion.div>
  );
}
