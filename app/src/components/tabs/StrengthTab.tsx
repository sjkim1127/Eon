import { motion } from "framer-motion";
import { Activity, Zap, Shield, TrendingUp, AlertTriangle } from "lucide-react";
import {
  ResponsiveContainer,
  RadarChart,
  PolarGrid,
  PolarAngleAxis,
  PolarRadiusAxis,
  Radar,
  Tooltip,
} from "recharts";
import { CHART_TOOLTIP_STYLE } from "../../lib/chartTheme";

interface StrengthTabProps {
  sajuReport: any;
  unknownTime?: boolean;
}

const ELEMENT_COLORS: Record<string, string> = {
  Wood: "text-green-400  border-green-500/30 bg-green-500/10",
  Fire: "text-red-400    border-red-500/30   bg-red-500/10",
  Earth: "text-yellow-400 border-yellow-500/30 bg-yellow-500/10",
  Metal: "text-gray-300  border-gray-400/30  bg-gray-400/10",
  Water: "text-blue-400  border-blue-500/30  bg-blue-500/10",
};

const ELEMENT_KOREAN: Record<string, string> = {
  Wood: "목(木)",
  Fire: "화(火)",
  Earth: "토(土)",
  Metal: "금(金)",
  Water: "수(水)",
};

export function StrengthTab({ sajuReport, unknownTime = false }: StrengthTabProps) {
  if (!sajuReport) return null;
  const entropy = sajuReport.entropy;
  const topology = sajuReport.qi_topology;
  const lints: any[] = sajuReport.lints ?? [];
  const loadDiag: any[] = sajuReport.load_diagnostics ?? [];
  const crashCount: number = sajuReport.crash_count ?? 0;

  return (
    <motion.div
      key="strength-tab"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* 시간 미상 경고 */}
      {unknownTime && (
        <div className="flex items-center gap-3 px-5 py-3 rounded-2xl bg-amber-500/10 border border-amber-500/25 text-amber-300 text-sm">
          <AlertTriangle className="w-4 h-4 shrink-0 text-amber-400" />
          <span>
            <strong>시주(時柱) 미상</strong> — 득시(得時) 판정 및 시 기반 십이운성 수치는 시주 시간에 따라 달라집니다. 참고용으로 활용하세요.
          </span>
        </div>
      )}

      {/* 오행 에너지 균형 */}
      {topology?.nodes && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Activity className="w-6 h-6 text-celestial-cyan" />
            오행 기운 분포 및 에너지 흐름
          </h5>

          {/* 전체 에너지 흐름 원활도 (순환 지수) */}
          <div className="mb-6 bg-white/5 border border-white/10 p-5 rounded-2xl flex flex-col sm:flex-row items-start sm:items-center justify-between gap-4">
            <div>
              <p className="text-sm font-bold text-white mb-1">전체 에너지 흐름 원활도 (순환 지수)</p>
              <p className="text-xs text-white/50">사주 내 오행이 서로 상생, 상극하며 얼마나 막힘없이 원활하게 흐르는지 나타냅니다.</p>
            </div>
            <div className="text-right shrink-0">
              <span className="text-3xl font-black text-celestial-cyan">
                {Math.round((topology.throughput ?? 0) * 100)}%
              </span>
            </div>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
            <div className="h-72 bg-white/5 rounded-2xl border border-white/10 p-3">
              <ResponsiveContainer width="100%" height="100%">
                <RadarChart
                  data={topology.nodes.map((node: any) => {
                    const elName = typeof node.element === "string" ? (ELEMENT_KOREAN[node.element] ?? node.element) : (node.element?.hangul ?? node.element);
                    return {
                      element: elName,
                      score: Math.min(100, Math.round((node.output ?? 0) * 5)),
                    };
                  })}
                >
                  <PolarGrid stroke="rgba(255,255,255,0.2)" />
                  <PolarAngleAxis dataKey="element" tick={{ fill: "rgba(255,255,255,0.75)", fontSize: 12 }} />
                  <PolarRadiusAxis
                    angle={30}
                    domain={[0, 100]}
                    tick={{ fill: "rgba(255,255,255,0.35)", fontSize: 10 }}
                    stroke="rgba(255,255,255,0.2)"
                  />
                  <Tooltip
                    formatter={(value: number) => [`${value.toFixed(0)}`, "에너지 지수"]}
                    contentStyle={CHART_TOOLTIP_STYLE}
                  />
                  <Radar dataKey="score" stroke="#6366f1" fill="#6366f1" fillOpacity={0.45} />
                </RadarChart>
              </ResponsiveContainer>
            </div>

            <div className="space-y-3">
              {topology.nodes.map((node: any, i: number) => {
                const colorClass = ELEMENT_COLORS[typeof node.element === "string" ? node.element : node.element?.english] ?? "text-white/60";
                const elName = typeof node.element === "string" ? (ELEMENT_KOREAN[node.element] ?? node.element) : (node.element?.hangul ?? node.element);
                const pct = Math.min(100, Math.round((node.output ?? 0) * 5));

                const isBottleneck = topology?.bottleneck && (
                  node.element === topology.bottleneck ||
                  (typeof node.element === "object" && node.element?.hangul && node.element.hangul === topology.bottleneck?.hangul)
                );

                return (
                  <div key={i} className={`p-4 rounded-2xl border ${colorClass}`}>
                    <div className="flex items-center justify-between gap-2 mb-2">
                      <p className="text-xs font-bold tracking-wider">{elName}</p>
                      <p className="text-lg font-black text-white">{pct}</p>
                    </div>
                    <div className="w-full bg-white/10 h-2 rounded-full overflow-hidden">
                      <div className="h-full rounded-full bg-current transition-all" style={{ width: `${pct}%` }} />
                    </div>
                    {isBottleneck && (
                      <span className="mt-2 inline-block text-[10px] px-2 py-0.5 rounded-full bg-red-500/20 text-red-400 border border-red-500/40">흐름이 막히거나 정체되는 기운</span>
                    )}
                  </div>
                );
              })}
            </div>
          </div>
        </div>
      )}

      {/* 운명 복잡도 */}
      {entropy && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Zap className="w-6 h-6 text-celestial-gold" />
            운명 복잡도 지수
          </h5>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">복잡도 등급</p>
              <p className="text-4xl font-black text-celestial-gold">{entropy.level ?? "—"}</p>
            </div>
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">복잡도 수치</p>
              <p className="text-4xl font-black text-white">{entropy.score?.toFixed(3) ?? "—"}</p>
            </div>
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">시뮬레이션 중 위기 상황 발생 횟수</p>
              <p className={`text-4xl font-black ${crashCount > 5 ? "text-red-400" : "text-green-400"}`}>{crashCount}번</p>
            </div>
          </div>

          <div className="mb-6 p-5 bg-white/5 border border-white/10 rounded-2xl">
            <p className="text-sm text-white/80 leading-relaxed font-medium">
              {entropy.description ?? "오행의 분포에 따른 삶의 복잡성을 나타냅니다."}
            </p>
            {entropy.unpacker_element && (
              <div className="flex items-center gap-3 mt-4 pt-4 border-t border-white/10">
                <span className="text-xs font-bold text-white/50">운명적 꼬임을 풀어주는 해결 열쇠 (오행):</span>
                <span className="px-3 py-1 bg-celestial-gold/20 border border-celestial-gold/40 text-celestial-gold rounded-full text-xs font-bold">
                  {typeof entropy.unpacker_element === 'string' ? (ELEMENT_KOREAN[entropy.unpacker_element] ?? entropy.unpacker_element) : (entropy.unpacker_element?.hangul ?? "알 수 없음")}
                </span>
              </div>
            )}
          </div>

          {/* 운명 복잡도 게이지 — SVG 도넛 링 */}
          {(() => {
            const score = entropy.score ?? 0;
            const pct = Math.min(100, Math.max(0, (score / 4.0) * 100));
            const r = 72;
            const circ = 2 * Math.PI * r;
            const dash = (pct / 100) * circ;
            const level = score < 1.5 ? "단순" : score < 2.5 ? "보통" : score < 3.2 ? "복잡" : "매우 복잡";
            const levelColor = score < 1.5 ? "#4ade80" : score < 2.5 ? "#facc15" : score < 3.2 ? "#f97316" : "#ef4444";
            return (
              <div className="relative flex flex-col items-center justify-center mt-6 py-6">
                <svg width="180" height="180" viewBox="0 0 180 180" className="drop-shadow-lg">
                  <defs>
                    <linearGradient id="gaugeGrad" x1="0%" y1="0%" x2="100%" y2="100%">
                      <stop offset="0%" stopColor="#06b6d4" />
                      <stop offset="50%" stopColor="#8b5cf6" />
                      <stop offset="100%" stopColor="#ef4444" />
                    </linearGradient>
                  </defs>
                  {/* Background ring */}
                  <circle cx="90" cy="90" r={r} fill="none" stroke="rgba(255,255,255,0.06)" strokeWidth="14" />
                  {/* Filled arc */}
                  <circle
                    cx="90" cy="90" r={r}
                    fill="none"
                    stroke="url(#gaugeGrad)"
                    strokeWidth="14"
                    strokeLinecap="round"
                    strokeDasharray={`${dash} ${circ}`}
                    transform="rotate(-90 90 90)"
                    style={{ transition: "stroke-dasharray 0.8s ease" }}
                  />
                </svg>
                {/* Center text */}
                <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
                  <p className="text-4xl font-black text-white tracking-tight">{score.toFixed(2)}</p>
                  <p className="text-xs font-bold mt-1 tracking-wider" style={{ color: levelColor }}>{level}</p>
                  <p className="text-[10px] text-white/30 mt-0.5">/ 4.00</p>
                </div>
              </div>
            );
          })()}
        </div>
      )}

      {/* 사주 체크업 */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
          <Shield className="w-6 h-6 text-celestial-purple" />
          사주 체크업
        </h5>
        {lints.length === 0 ? (
          <p className="text-green-400 font-semibold">✅ 문제 없음! 균형 잡힌 흐름입니다.</p>
        ) : (
          <div className="space-y-3">
            {lints.map((lint: any, i: number) => (
              <div key={i} className={`p-4 rounded-xl border flex gap-3 items-start ${lint.severity === "Error"
                ? "bg-red-500/10 border-red-500/30"
                : lint.severity === "Warning"
                  ? "bg-yellow-500/10 border-yellow-500/30"
                  : "bg-blue-500/10 border-blue-500/30"
                }`}>
                <span className={`text-xs font-black px-2 py-1 rounded shrink-0 ${lint.severity === "Error" ? "bg-red-500/30 text-red-400"
                  : lint.severity === "Warning" ? "bg-yellow-500/30 text-yellow-400"
                    : "bg-blue-500/30 text-blue-400"
                  }`}>{lint.severity?.toUpperCase()}</span>
                <div>
                  <p className="text-sm font-bold text-white">[{lint.code}] {lint.message}</p>
                  <p className="text-xs text-white/50 mt-1">└ {lint.advice}</p>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {/* 인생 흐름 진단 */}
      {loadDiag.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <TrendingUp className="w-6 h-6 text-brand-400" />
            인생 흐름 진단
          </h5>
          <div className="space-y-3">
            {loadDiag.slice(0, 8).map((d: any, i: number) => (
              <div key={i} className={`p-4 rounded-xl border flex gap-4 items-center ${d.status === "SystemDown" ? "bg-red-500/10 border-red-500/30"
                : d.status === "Overloaded" ? "bg-orange-500/10 border-orange-500/30"
                  : "bg-white/5 border-white/10"
                }`}>
                <span className="text-2xl">
                  {d.status === "SystemDown" ? "🚫" : d.status === "Overloaded" ? "🔥" : "ℹ️"}
                </span>
                <div className="flex-1">
                  <p className="text-sm font-bold text-white">[{d.age}세] {d.reason}</p>
                  <p className="text-xs text-white/50 mt-0.5">▶ {d.strategy}</p>
                </div>
                <span className="text-xs text-white/30 shrink-0">{d.status === "SystemDown" ? "위험" : d.status === "Overloaded" ? "주의" : "안정"}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </motion.div>
  );
}
