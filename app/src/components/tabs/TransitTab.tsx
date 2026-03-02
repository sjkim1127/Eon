import { motion } from "framer-motion";
import { Activity, Shield } from "lucide-react";

interface TransitTabProps {
  transitReport: any;
}

export function TransitTab({ transitReport }: TransitTabProps) {
  if (!transitReport) return null;
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
          <h4 className="text-4xl font-black text-celestial-gold mb-3">{yr?.ganzi?.hangul ?? "—"}</h4>
          <div className="space-y-1 text-sm text-white/60 mb-4">
            <p>천간 십성: <span className="text-white font-semibold">{yr?.stem_god?.hangul ?? yr?.stem_god ?? "—"}</span></p>
            <p>지지 십성: <span className="text-white font-semibold">{yr?.branch_god?.hangul ?? yr?.branch_god ?? "—"}</span></p>
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
          <h4 className="text-4xl font-black text-white mb-3">{mo?.ganzi?.hangul ?? "—"}</h4>
          <div className="space-y-1 text-sm text-white/60 mb-4">
            <p>천간 십성: <span className="text-white font-semibold">{mo?.stem_god?.hangul ?? mo?.stem_god ?? "—"}</span></p>
            <p>지지 십성: <span className="text-white font-semibold">{mo?.branch_god?.hangul ?? mo?.branch_god ?? "—"}</span></p>
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

      {/* 현재 나이 LifeFrame */}
      {frame && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Activity className="w-6 h-6 text-celestial-cyan" />
            현재 시스템 상태 ({age}세 LifeFrame)
          </h5>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">System Score</p>
              <p className={`text-5xl font-black ${scoreColor(frame.score ?? 0)}`}>
                {frame.score?.toFixed(1) ?? "—"}
              </p>
            </div>
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">현재 간지</p>
              <p className="text-4xl font-black text-white">{frame.ganzi?.hangul ?? "—"}</p>
            </div>
            <div className="p-6 bg-white/5 rounded-2xl border border-white/10">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">Tags</p>
              <div className="flex flex-wrap gap-1">
                {(frame.tags ?? []).map((tag: string, i: number) => (
                  <span key={i} className="text-xs px-2 py-1 rounded-full bg-celestial-purple/20 text-celestial-purple border border-celestial-purple/30">{tag}</span>
                ))}
              </div>
            </div>
          </div>
          {/* ESIL Trace */}
          {frame.esil_trace && (
            <div className="bg-black/40 rounded-xl p-4 font-mono text-xs border border-white/10 overflow-x-auto">
              <p className="text-white/40 mb-2">// ESIL TRACE</p>
              {frame.esil_trace.split("; ").map((line: string, i: number) => (
                <p key={i} className={line.includes("irq") || line.includes("panic") ? "text-red-400" : "text-green-300/70"}>
                  {line}
                </p>
              ))}
            </div>
          )}

          {/* QiRegisters 미니 바 차트 */}
          {frame.register_state && (
            <div className="mt-4">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">오행 레지스터 (QiRegisters)</p>
              <div className="grid grid-cols-5 gap-3">
                {[
                  { label: "木", key: "r0_wood", color: "bg-green-400" },
                  { label: "火", key: "r1_fire", color: "bg-red-400" },
                  { label: "土", key: "r2_earth", color: "bg-yellow-400" },
                  { label: "金", key: "r3_metal", color: "bg-gray-300" },
                  { label: "水", key: "r4_water", color: "bg-blue-400" },
                ].map(({ label, key, color }) => {
                  const val: number = frame.register_state[key] ?? 0;
                  const pct = Math.min(100, Math.round(Math.abs(val) * 4));
                  return (
                    <div key={key} className="text-center">
                      <div className="h-16 bg-white/5 rounded-lg flex items-end overflow-hidden mb-1">
                        <div className={`w-full ${color} rounded-lg opacity-80 transition-all`} style={{ height: `${pct}%` }} />
                      </div>
                      <p className="text-xs text-white/40">{label}</p>
                      <p className="text-[10px] text-white/30">{val.toFixed(1)}</p>
                    </div>
                  );
                })}
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
              </div>
            ))}
          </div>
        </div>
      )}
    </motion.div>
  );
}
