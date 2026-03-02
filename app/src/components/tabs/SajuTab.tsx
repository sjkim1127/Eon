import { motion } from "framer-motion";
import { Activity, Zap, Shield, Star, TrendingUp, AlertTriangle } from "lucide-react";
import {
  ResponsiveContainer,
  AreaChart,
  Area,
  CartesianGrid,
  XAxis,
  YAxis,
  Tooltip,
} from "recharts";
import { CHART_TOOLTIP_STYLE } from "../../lib/chartTheme";
import {
  STEM_INFO, BRANCH_INFO, ELEMENT_INFO,
  STRENGTH_INFO, TENGOD_INFO, STRUCTURE_INFO,
  SPIRIT_INFO, PILLAR_POS_INFO, YONGSHIN_TYPE_INFO,
} from "../../constants";
import { ganziDisplay, ganziHangul } from "../../utils";

interface SajuTabProps {
  sajuReport: any;
  unknownTime?: boolean;
}

export function SajuTab({ sajuReport, unknownTime = false }: SajuTabProps) {
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
        <div className="grid grid-cols-4 gap-4">
          {[
            { label: "시주", pillar: p?.hour, isHour: true },
            { label: "일주", pillar: p?.day, isHour: false },
            { label: "월주", pillar: p?.month, isHour: false },
            { label: "년주", pillar: p?.year, isHour: false },
          ].map(({ label, pillar, isHour }) => (
            <div key={label} className={`text-center p-4 rounded-2xl border transition-all ${
              isHour && unknownTime
                ? "bg-amber-500/5 border-amber-500/25"
                : "bg-white/5 border-white/10"
            }`}>
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3 flex items-center justify-center gap-1.5">
                {label}
                {isHour && unknownTime && (
                  <span className="text-[9px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-400 border border-amber-400/30 font-bold">미상</span>
                )}
              </p>
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
          {/* 용신 상세 목록 (조후/억부/통관/병약) */}
          {y?.recommendations && y.recommendations.length > 0 && (
            <div className="space-y-2 mb-3">
              {y.recommendations.map((rec: any, i: number) => (
                <div key={i} className="flex items-start gap-2 text-xs">
                  <span className="shrink-0 px-1.5 py-0.5 rounded bg-celestial-purple/30 text-celestial-purple/90 font-bold">
                    {YONGSHIN_TYPE_INFO[rec.yongshin_type] || rec.yongshin_type}
                  </span>
                  <span className="text-white/70 font-semibold">
                    {ELEMENT_INFO[rec.element]?.hangul || rec.element}
                  </span>
                </div>
              ))}
            </div>
          )}
          <p className="text-xs text-white/40 leading-relaxed">
            희신(喜神): {ELEMENT_INFO[y?.assistant]?.hangul || y?.assistant || "—"}
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

      {/* 인생 시뮬레이션 타임라인 */}
      {reportData.simulation_frames && reportData.simulation_frames.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <TrendingUp className="w-6 h-6 text-celestial-cyan" />
            인생 흐름 그래프 (0~100세 운세 점수)
          </h5>
          <div className="h-64 w-full">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart
                data={(reportData.simulation_frames as any[]).map((f: any) => ({
                  age: f.age,
                  score: Number(f.score ?? 0),
                }))}
                margin={{ top: 8, right: 12, left: 0, bottom: 8 }}
              >
                <defs>
                  <linearGradient id="sajuScoreGradient" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#06b6d4" stopOpacity={0.7} />
                    <stop offset="95%" stopColor="#06b6d4" stopOpacity={0.05} />
                  </linearGradient>
                </defs>
                <CartesianGrid stroke="rgba(255,255,255,0.08)" strokeDasharray="3 3" />
                <XAxis
                  dataKey="age"
                  stroke="rgba(255,255,255,0.45)"
                  tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 11 }}
                  tickLine={false}
                  axisLine={{ stroke: "rgba(255,255,255,0.15)" }}
                  unit="세"
                />
                <YAxis
                  domain={[0, 100]}
                  stroke="rgba(255,255,255,0.45)"
                  tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 11 }}
                  tickLine={false}
                  axisLine={{ stroke: "rgba(255,255,255,0.15)" }}
                />
                <Tooltip
                  formatter={(value: number) => [`${value.toFixed(1)}점`, "운세 점수"]}
                  labelFormatter={(label: number) => `${label}세`}
                  contentStyle={CHART_TOOLTIP_STYLE}
                />
                <Area
                  type="monotone"
                  dataKey="score"
                  stroke="#06b6d4"
                  strokeWidth={2.5}
                  fill="url(#sajuScoreGradient)"
                  activeDot={{ r: 5, stroke: "#06b6d4", strokeWidth: 2, fill: "#111827" }}
                />
              </AreaChart>
            </ResponsiveContainer>
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
