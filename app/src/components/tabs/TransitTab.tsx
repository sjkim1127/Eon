import { motion } from "framer-motion";
import { Activity, Shield, AlertCircle } from "lucide-react";
import {
  ResponsiveContainer,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  CartesianGrid,
  Cell,
} from "recharts";
import { CHART_TOOLTIP_STYLE } from "../../lib/chartTheme";
import { TENGOD_INFO } from "../../constants";
import { ganziHangul, ganziDisplay } from "../../utils";

interface TransitTabProps {
  transitReport: any;
  transitError?: string | null;
}

export function TransitTab({ transitReport, transitError }: TransitTabProps) {
  if (!transitReport) {
    return (
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        className="h-[40vh] flex flex-col items-center justify-center text-center"
      >
        <div className="w-20 h-20 rounded-full bg-white/5 flex items-center justify-center mb-5">
          {transitError
            ? <AlertCircle className="w-10 h-10 text-red-400" />
            : <Activity className="w-10 h-10 text-white/20 animate-pulse" />
          }
        </div>
        {transitError ? (
          <>
            <h3 className="text-xl font-semibold text-red-400 mb-2">운세 분석 오류</h3>
            <p className="text-white/50 text-xs max-w-md font-mono bg-red-500/10 border border-red-500/20 rounded-xl px-4 py-3 break-all">
              {transitError}
            </p>
          </>
        ) : (
          <>
            <h3 className="text-xl font-semibold text-white mb-2">운세 데이터 없음</h3>
            <p className="text-white/40 text-sm max-w-sm">
              출생 정보를 입력하고 통합 분석을 시작하면<br />현재 세운·월운 분석이 표시됩니다.
            </p>
          </>
        )}
      </motion.div>
    );
  }
  const yr = transitReport.yearly_luck;
  const mo = transitReport.monthly_luck;
  const frame = transitReport.current_frame;
  const nearby: any[] = transitReport.nearby_diagnostics ?? [];
  const age: number = transitReport.current_age ?? 0;

  const scoreColor = (s: number) =>
    s >= 70 ? "text-green-400" : s >= 40 ? "text-yellow-400" : "text-red-400";

  return (
    <motion.div
      key="transit-tab"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* 세운 / 월운 카드 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass p-8 rounded-[2rem]">
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-3">세운 (年運) — {yr?.year}년</p>
          <h4 className="text-4xl font-black text-celestial-gold mb-3">{ganziHangul(yr?.ganzi) || "—"}</h4>
          <div className="space-y-1 text-sm text-white/60 mb-4">
            <p>천간 십성: <span className="text-white font-semibold">{TENGOD_INFO[yr?.stem_god]?.hangul ?? yr?.stem_god ?? "—"}</span></p>
            <p>지지 십성: <span className="text-white font-semibold">{TENGOD_INFO[yr?.branch_god]?.hangul ?? yr?.branch_god ?? "—"}</span></p>
            {yr?.twelve_stage && (
              <p>12운성: <span className="text-celestial-cyan font-semibold">{yr.twelve_stage}</span></p>
            )}
            {yr?.special_events?.length > 0 && (
              <p className="text-red-400 font-semibold">⚠️ {yr.special_events.join(" / ")}</p>
            )}
          </div>
          {yr?.influence?.relations_with_natal?.length > 0 && (
            <div className="flex flex-wrap gap-1.5 mt-3">
              {yr.influence.relations_with_natal.map((rel: string, i: number) => (
                <span key={i} className="text-xs px-2.5 py-1 rounded-full bg-celestial-gold/15 text-celestial-gold border border-celestial-gold/30 font-medium">{rel}</span>
              ))}
            </div>
          )}
        </div>
        <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
          <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-3">월운 (月運) — {mo?.month}월</p>
          <h4 className="text-4xl font-black text-white mb-3">{ganziHangul(mo?.ganzi) || "—"}</h4>
          <div className="space-y-1 text-sm text-white/60 mb-4">
            <p>천간 십성: <span className="text-white font-semibold">{TENGOD_INFO[mo?.stem_god]?.hangul ?? mo?.stem_god ?? "—"}</span></p>
            <p>지지 십성: <span className="text-white font-semibold">{TENGOD_INFO[mo?.branch_god]?.hangul ?? mo?.branch_god ?? "—"}</span></p>
            {mo?.twelve_stage && (
              <p>12운성: <span className="text-celestial-cyan font-semibold">{mo.twelve_stage}</span></p>
            )}
          </div>
          {mo?.influence?.relations_with_natal?.length > 0 && (
            <div className="flex flex-wrap gap-1.5 mt-3">
              {mo.influence.relations_with_natal.map((rel: string, i: number) => (
                <span key={i} className="text-xs px-2.5 py-1 rounded-full bg-celestial-purple/15 text-celestial-purple border border-celestial-purple/30 font-medium">{rel}</span>
              ))}
            </div>
          )}
        </div>
      </div>

      {/* 현재 운세 상태 */}
      {frame && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Activity className="w-6 h-6 text-celestial-cyan" />
            현재 운세 상태 ({age}세)
          </h5>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">운세 점수</p>
              <p className={`text-5xl font-black ${scoreColor(frame.score ?? 0)}`}>
                {frame.score?.toFixed(1) ?? "—"}
              </p>
            </div>
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">현재 간지</p>
              <p className="text-4xl font-black text-white">{ganziDisplay(frame.ganzi) || "—"}</p>
            </div>
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">특징</p>
              <div className="flex flex-wrap gap-1">
                {(frame.tags ?? []).map((tag: string, i: number) => (
                  <span key={i} className="text-xs px-2 py-1 rounded-full bg-celestial-purple/20 text-celestial-purple border border-celestial-purple/30">{tag}</span>
                ))}
              </div>
            </div>
          </div>
          {/* 흐름 추적 */}
          {frame.esil_trace && (
            <div className="bg-black/40 rounded-xl p-4 font-mono text-xs border border-white/10 overflow-x-auto">
              <p className="text-white/40 mb-2">// 흐름 추적 로그</p>
              {frame.esil_trace.split("; ").map((line: string, i: number) => (
                <p key={i} className={line.includes("irq") || line.includes("panic") ? "text-red-400" : "text-green-300/70"}>
                  {line}
                </p>
              ))}
            </div>
          )}

          {/* 오행 에너지 분포 */}
          {frame.register_state && (
            <div className="mt-4">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">오행 에너지 분포</p>
              <div className="h-56 bg-white/5 rounded-xl border border-white/10 p-3">
                <ResponsiveContainer width="100%" height="100%">
                  <BarChart
                    data={[
                      { label: "木", value: frame.register_state.r0_wood ?? 0, color: "#4ade80" },
                      { label: "火", value: frame.register_state.r1_fire ?? 0, color: "#f87171" },
                      { label: "土", value: frame.register_state.r2_earth ?? 0, color: "#facc15" },
                      { label: "金", value: frame.register_state.r3_metal ?? 0, color: "#d1d5db" },
                      { label: "水", value: frame.register_state.r4_water ?? 0, color: "#60a5fa" },
                    ]}
                    margin={{ top: 10, right: 10, left: 0, bottom: 0 }}
                  >
                    <CartesianGrid stroke="rgba(255,255,255,0.08)" strokeDasharray="3 3" />
                    <XAxis dataKey="label" tick={{ fill: "rgba(255,255,255,0.65)", fontSize: 11 }} axisLine={false} tickLine={false} />
                    <YAxis tick={{ fill: "rgba(255,255,255,0.65)", fontSize: 11 }} axisLine={false} tickLine={false} />
                    <Tooltip
                      formatter={(value: number) => [`${value.toFixed(1)}`, "에너지"]}
                      contentStyle={CHART_TOOLTIP_STYLE}
                    />
                    <Bar dataKey="value" radius={[10, 10, 0, 0]}>
                      {[
                        { color: "#4ade80" },
                        { color: "#f87171" },
                        { color: "#facc15" },
                        { color: "#d1d5db" },
                        { color: "#60a5fa" },
                      ].map((entry, idx) => (
                        <Cell key={idx} fill={entry.color} />
                      ))}
                    </Bar>
                  </BarChart>
                </ResponsiveContainer>
              </div>
            </div>
          )}
        </div>
      )}

      {/* 주변 부하 진단 */}
      {nearby.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Shield className="w-6 h-6 text-brand-400" />
            전후 운세 부하 ({age - 3}~{age + 5}세)
          </h5>
          <div className="space-y-3">
            {nearby.map((d: any, i: number) => (
              <div key={i} className={`p-4 rounded-xl border flex gap-4 items-center ${
                d.status === "SystemDown" ? "bg-red-500/10 border-red-500/30"
                : d.status === "Overloaded" ? "bg-orange-500/10 border-orange-500/30"
                : "bg-white/5 border-white/10"
              } ${d.age === age ? "ring-2 ring-celestial-purple" : ""}`}>
                <span className="text-2xl">{d.status === "SystemDown" ? "🚫" : d.status === "Overloaded" ? "🔥" : "ℹ️"}</span>
                <div className="flex-1">
                  <p className="text-sm font-bold text-white">
                    {d.age === age && <span className="text-celestial-purple mr-1">[현재]</span>}
                    [{d.age}세] {d.reason}
                  </p>
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
