import { motion } from "framer-motion";
import { Activity, Zap, Shield, Star, TrendingUp } from "lucide-react";
import {
  STEM_INFO, BRANCH_INFO, ELEMENT_INFO,
  STRENGTH_INFO, TENGOD_INFO, STRUCTURE_INFO,
  SPIRIT_INFO, PILLAR_POS_INFO,
} from "../../constants";
import { ganziDisplay, ganziHangul } from "../../utils";

interface SajuTabProps {
  sajuReport: any;
}

export function SajuTab({ sajuReport }: SajuTabProps) {
  if (!sajuReport || !sajuReport.report) return null;
  const reportData = sajuReport.report;
  const p = reportData.pillars;
  const s = reportData.strength;
  const y = reportData.yongshin;
  const st = reportData.structure;
  const sp = reportData.spirit_markers;
  const ml = reportData.major_luck;
  const gt = reportData.golden_time;

  return (
    <motion.div
      key="saju-results"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* 사주팔자 차트 */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
          <Activity className="w-6 h-6 text-celestial-gold" />
          사주팔자 (四柱八字)
        </h5>
        <div className="grid grid-cols-4 gap-4">
          {[
            { label: "시주", pillar: p?.hour },
            { label: "일주", pillar: p?.day },
            { label: "월주", pillar: p?.month },
            { label: "년주", pillar: p?.year },
          ].map(({ label, pillar }) => (
            <div key={label} className="text-center p-4 bg-white/5 rounded-2xl border border-white/10">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">{label}</p>
              <p className="text-2xl font-bold text-celestial-gold mb-1">
                {STEM_INFO[pillar?.stem]?.hanja || "—"}
              </p>
              <p className="text-2xl font-bold text-celestial-cyan">
                {BRANCH_INFO[pillar?.branch]?.hanja || "—"}
              </p>
              <p className="text-xs text-white/30 mt-2">
                {STEM_INFO[pillar?.stem]?.hangul || ""} {BRANCH_INFO[pillar?.branch]?.hangul || ""}
              </p>
            </div>
          ))}
        </div>
      </div>

      {/* 역량 + 용신 + 격국 */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* 역량 분석 */}
        <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
          <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
            <Zap className="w-32 h-32" />
          </div>
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            신강/신약 분석
          </p>
          <h4 className="text-3xl font-bold text-white mb-2">
            {STRENGTH_INFO[s?.strength_type] || s?.strength_type || "—"}
          </h4>
          <div className="flex items-baseline gap-2 mb-4">
            <span className="text-5xl font-black text-gradient leading-none">
              {s?.strength_score != null ? Math.round(s.strength_score) : "—"}
            </span>
            <span className="text-white/20 font-bold">점</span>
          </div>
          <div className="space-y-1 text-xs text-white/50">
            <p>득령: {s?.deuk_ryeong?.acquired ? "✅" : "❌"}</p>
            <p>득지: {s?.deuk_ji?.acquired ? "✅" : "❌"}</p>
            <p>득시: {s?.deuk_si?.acquired ? "✅" : "❌"}</p>
            <p>득세: {s?.deuk_se?.acquired ? "✅" : "❌"}</p>
          </div>
        </div>

        {/* 용신 */}
        <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
          <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
            용신 (用神)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {ELEMENT_INFO[y?.primary]?.hangul || y?.primary || "—"}
            <span className="text-lg text-white/40 ml-2">{ELEMENT_INFO[y?.primary]?.hanja}</span>
          </h4>
          <p className="text-sm text-white/60 mb-2">
            <span className="text-white/40">보조 용신:</span>{" "}
            {ELEMENT_INFO[y?.assistant]?.hangul || y?.assistant || "—"}
          </p>
          <p className="text-xs text-white/40 leading-relaxed">
            용신은 사주의 균형을 맞추는 가장 필요한 오행입니다.
          </p>
        </div>

        {/* 격국 */}
        <div className="glass p-8 rounded-[2rem]">
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            격국 (格局)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {STRUCTURE_INFO[st?.structure]?.hangul || st?.structure || "—"}
            <span className="text-sm text-white/40 ml-2">{STRUCTURE_INFO[st?.structure]?.hanja}</span>
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            {st?.reason || ""}
          </p>
        </div>
      </div>

      {/* 신살 */}
      {sp?.markers && sp.markers.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Shield className="w-6 h-6 text-celestial-cyan" />
            신살 (神煞) 분석
          </h5>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
            {sp.markers.map((m: any, i: number) => (
              <div
                key={i}
                className="p-4 bg-white/5 rounded-xl border border-white/10 text-center"
              >
                <p className="text-sm font-bold text-celestial-gold">{SPIRIT_INFO[m.marker]?.hangul || m.marker || "—"}</p>
                <p className="text-xs text-white/40 mt-1">{PILLAR_POS_INFO[m.position] || m.position || ""}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* 대운 */}
      {ml && ml.cycles && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <TrendingUp className="w-6 h-6 text-celestial-purple" />
            대운 (大運) 흐름
          </h5>
          <p className="text-sm text-white/50 mb-4">
            방향: {ml.direction === "Forward" ? "순행 ▶" : "역행 ◀"} | 시작 나이: {ml.start_age}세
          </p>
          <div className="grid grid-cols-2 sm:grid-cols-5 lg:grid-cols-10 gap-3">
            {ml.cycles.map((c: any, i: number) => (
              <div
                key={i}
                className="p-3 bg-white/5 rounded-xl border border-white/10 text-center hover:bg-white/10 transition-all"
              >
                <p className="text-xs text-white/40 mb-1">{c.start_age}~{c.end_age}세</p>
                <p className="text-lg font-bold text-white">{ganziDisplay(c.ganzi)}</p>
                <p className="text-xs text-white/30">{ganziHangul(c.ganzi)}</p>
                <p className="text-[10px] text-celestial-gold mt-1">{TENGOD_INFO[c.stem_god]?.hangul || c.stem_god || ""}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* 골든타임 */}
      {gt && (
        <div className="glass p-8 rounded-[2rem] border-celestial-gold/20 bg-celestial-gold/5">
          <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
            <Star className="w-6 h-6 text-celestial-gold" />
            골든타임 🏆
          </h5>
          <div className="flex items-baseline gap-4 mb-4">
            <span className="text-5xl font-black text-celestial-gold">
              {gt.start_age}~{gt.end_age}세
            </span>
            <span className="text-white/40">평균 점수: {gt.average_score?.toFixed(1)}</span>
          </div>
          <p className="text-sm text-white/60">{gt.description}</p>
        </div>
      )}

      {/* 인생 시뮬레이션 타임라인 (LifeFrame × 100) */}
      {reportData.simulation_frames && reportData.simulation_frames.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <TrendingUp className="w-6 h-6 text-celestial-cyan" />
            인생 시뮬레이션 (0~100세 System Score)
          </h5>
          <div className="flex items-end gap-0.5 h-24 w-full overflow-x-auto">
            {(reportData.simulation_frames as any[]).map((f: any, i: number) => {
              const score: number = f.score ?? 0;
              const heightPct = Math.max(4, Math.round(score));
              const color =
                score >= 70
                  ? "bg-green-400"
                  : score >= 40
                  ? "bg-celestial-gold"
                  : "bg-red-400";
              return (
                <div
                  key={i}
                  title={`${f.age}세: ${score.toFixed(1)}`}
                  className={`flex-1 min-w-[4px] rounded-sm ${color} opacity-80 hover:opacity-100 transition-opacity cursor-help`}
                  style={{ height: `${heightPct}%` }}
                />
              );
            })}
          </div>
          <div className="flex justify-between text-xs text-white/30 mt-2">
            <span>0세</span>
            <span>25세</span>
            <span>50세</span>
            <span>75세</span>
            <span>100세</span>
          </div>
          <div className="flex gap-4 mt-3 text-xs text-white/40">
            <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-green-400 inline-block" />70+ 좋음</span>
            <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-celestial-gold inline-block" />40~70 보통</span>
            <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-red-400 inline-block" />~40 주의</span>
          </div>
        </div>
      )}
    </motion.div>
  );
}
