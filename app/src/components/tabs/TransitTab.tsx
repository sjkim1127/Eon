import { motion } from "framer-motion";
import { Activity, Shield, AlertCircle, Calendar, CloudLightning, CloudRain, Cloud, CloudSun, Sun } from "lucide-react";
import {
  ResponsiveContainer,
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  CartesianGrid,
  ReferenceLine,
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
  const dl = transitReport.daily_luck;
  const monthlyAll: any[] = transitReport.monthly_lucks ?? [];
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
      {/* ── 운세 기상도 (Weather Forecast) ────────────────────── */}
      {frame && (
        <div className="glass p-8 rounded-[2rem] bg-gradient-to-br from-white/5 to-black/20 relative overflow-hidden">
          <div className="absolute top-0 right-0 p-8 opacity-10 pointer-events-none">
            {(frame.score ?? 0) >= 80 ? <Sun className="w-48 h-48" />
              : (frame.score ?? 0) >= 60 ? <CloudSun className="w-48 h-48" />
                : (frame.score ?? 0) >= 40 ? <Cloud className="w-48 h-48" />
                  : (frame.score ?? 0) >= 20 ? <CloudRain className="w-48 h-48" />
                    : <CloudLightning className="w-48 h-48" />
            }
          </div>

          <div className="relative z-10 flex flex-col md:flex-row gap-8 items-center md:items-start">
            <div className="text-center md:text-left">
              <p className="text-xs text-brand-300 font-bold uppercase tracking-wider mb-2">현재 운세 기상도</p>
              <div className="flex items-center gap-4 mb-4 justify-center md:justify-start">
                {(frame.score ?? 0) >= 80 ? <Sun className="w-12 h-12 text-yellow-400" />
                  : (frame.score ?? 0) >= 60 ? <CloudSun className="w-12 h-12 text-orange-300" />
                    : (frame.score ?? 0) >= 40 ? <Cloud className="w-12 h-12 text-gray-300" />
                      : (frame.score ?? 0) >= 20 ? <CloudRain className="w-12 h-12 text-blue-400" />
                        : <CloudLightning className="w-12 h-12 text-purple-400" />
                }
                <div>
                  <h3 className="text-3xl font-black text-white">
                    {(frame.score ?? 0) >= 80 ? "맑고 화창함 (매우 긍정적)"
                      : (frame.score ?? 0) >= 60 ? "가끔 구름 (순조로움)"
                        : (frame.score ?? 0) >= 40 ? "흐림 (무난함 / 정체기)"
                          : (frame.score ?? 0) >= 20 ? "비 (주의 및 대비 필요)"
                            : "뇌우 (변화와 시련기)"
                    }
                  </h3>
                  <p className="text-white/60 text-sm mt-1">
                    운세 점수: <span className={`font-bold ${scoreColor(frame.score ?? 0)}`}>{frame.score?.toFixed(1) ?? "—"}</span> / 100
                  </p>
                </div>
              </div>

              <div className="flex flex-wrap gap-2 justify-center md:justify-start mb-2">
                {(frame.tags ?? []).map((tag: string, i: number) => (
                  <span key={i} className="text-xs px-2.5 py-1 rounded-full bg-white/10 text-white/90 border border-white/20 font-medium">{tag}</span>
                ))}
              </div>
            </div>

            <div className="md:ml-auto flex gap-4 text-center">
              <div className="p-4 bg-black/30 rounded-2xl border border-white/10 min-w-[100px]">
                <p className="text-xs text-white/40 mb-1">현재 나이</p>
                <p className="text-2xl font-bold text-white">{age}세</p>
              </div>
              <div className="p-4 bg-black/30 rounded-2xl border border-white/10 min-w-[100px]">
                <p className="text-xs text-white/40 mb-1">현재 간지</p>
                <p className="text-2xl font-bold text-white">{ganziDisplay(frame.ganzi) || "—"}</p>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* 세운 / 월운 / 일운 카드 (가로 스크롤 또는 그리드로 재배치) */}
      <div className="grid grid-cols-1 sm:grid-cols-3 gap-6">
        <div className="glass p-6 md:p-8 rounded-[2rem]">
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
        <div className="glass p-6 md:p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
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

        {/* 일운 카드 */}
        {dl && (
          <div className="glass p-6 md:p-8 rounded-[2rem] border-celestial-cyan/20 bg-celestial-cyan/5">
            <p className="text-celestial-cyan/80 text-sm font-bold uppercase tracking-wider mb-3">일운 (日運) — {dl.month}월 {dl.day}일</p>
            <h4 className="text-4xl font-black text-white mb-3">{ganziHangul(dl.ganzi) || "—"}</h4>
            <div className="space-y-1 text-sm text-white/60 mb-4">
              <p>천간 십성: <span className="text-white font-semibold">{TENGOD_INFO[dl.stem_god]?.hangul ?? dl.stem_god ?? "—"}</span></p>
              <p>지지 십성: <span className="text-white font-semibold">{TENGOD_INFO[dl.branch_god]?.hangul ?? dl.branch_god ?? "—"}</span></p>
              {dl.twelve_stage && (
                <p>12운성: <span className="text-celestial-cyan font-semibold">{dl.twelve_stage}</span></p>
              )}
            </div>
            {dl.influence?.relations_with_natal?.length > 0 && (
              <div className="flex flex-wrap gap-1.5 mt-3">
                {dl.influence.relations_with_natal.map((rel: string, i: number) => (
                  <span key={i} className="text-xs px-2.5 py-1 rounded-full bg-celestial-cyan/15 text-celestial-cyan border border-celestial-cyan/30 font-medium">{rel}</span>
                ))}
              </div>
            )}
          </div>
        )}
      </div>

      {/* 12개월 월운 그리드 */}
      {monthlyAll.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Calendar className="w-6 h-6 text-celestial-purple" />
            {yr?.year ?? new Date().getFullYear()}년 월별 운세 한눈에 보기
          </h5>
          <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-3">
            {monthlyAll.map((m: any, i: number) => {
              const isCurrent = m.month === mo?.month;
              return (
                <div
                  key={i}
                  className={`p-4 rounded-2xl border text-center transition-all ${isCurrent
                    ? "border-celestial-purple/50 bg-celestial-purple/15 ring-2 ring-celestial-purple/40"
                    : "border-white/10 bg-white/[0.03] hover:bg-white/[0.06]"
                    }`}
                >
                  <p className={`text-xs font-bold mb-1 ${isCurrent ? "text-celestial-purple" : "text-white/30"}`}>{m.month}월</p>
                  <p className="text-lg font-bold text-white mb-1">{ganziHangul(m.ganzi) || "—"}</p>
                  <div className="text-[10px] text-white/50 space-y-0.5">
                    <p>{TENGOD_INFO[m.stem_god]?.hangul ?? m.stem_god} / {TENGOD_INFO[m.branch_god]?.hangul ?? m.branch_god}</p>
                    {m.twelve_stage && <p className="text-celestial-cyan/70">{m.twelve_stage}</p>}
                  </div>
                  {m.influence?.relations_with_natal?.length > 0 && (
                    <div className="flex flex-wrap justify-center gap-1 mt-2">
                      {m.influence.relations_with_natal.slice(0, 2).map((rel: string, j: number) => (
                        <span key={j} className="text-[9px] px-1.5 py-0.5 rounded-full bg-celestial-purple/10 text-celestial-purple/80 border border-celestial-purple/20">{rel}</span>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>
      )}

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


          {/* 오행 에너지 분포 */}
          {frame.register_state && (
            <div className="mt-4">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">오행 에너지 분포</p>
              <div className="h-56 bg-white/5 rounded-xl border border-white/10 p-3">
                <ResponsiveContainer width="100%" height="100%">
                  <BarChart
                    data={[
                      { label: "木", value: frame.register_state.r0_wood ?? 0, gradient: "url(#gradWood)", shadow: "#16a34a" },
                      { label: "火", value: frame.register_state.r1_fire ?? 0, gradient: "url(#gradFire)", shadow: "#dc2626" },
                      { label: "土", value: frame.register_state.r2_earth ?? 0, gradient: "url(#gradEarth)", shadow: "#ca8a04" },
                      { label: "金", value: frame.register_state.r3_metal ?? 0, gradient: "url(#gradMetal)", shadow: "#9ca3af" },
                      { label: "水", value: frame.register_state.r4_water ?? 0, gradient: "url(#gradWater)", shadow: "#2563eb" },
                    ]}
                    margin={{ top: 20, right: 10, left: -20, bottom: 0 }}
                  >
                    <defs>
                      <linearGradient id="gradWood" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stopColor="#4ade80" />
                        <stop offset="100%" stopColor="#16a34a" />
                      </linearGradient>
                      <linearGradient id="gradFire" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stopColor="#fb7185" />
                        <stop offset="100%" stopColor="#e11d48" />
                      </linearGradient>
                      <linearGradient id="gradEarth" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stopColor="#fde047" />
                        <stop offset="100%" stopColor="#d97706" />
                      </linearGradient>
                      <linearGradient id="gradMetal" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stopColor="#f3f4f6" />
                        <stop offset="100%" stopColor="#6b7280" />
                      </linearGradient>
                      <linearGradient id="gradWater" x1="0" y1="0" x2="0" y2="1">
                        <stop offset="0%" stopColor="#38bdf8" />
                        <stop offset="100%" stopColor="#2563eb" />
                      </linearGradient>
                    </defs>
                    <CartesianGrid stroke="rgba(255,255,255,0.05)" vertical={false} />
                    <XAxis dataKey="label" tick={{ fill: "rgba(255,255,255,0.65)", fontSize: 13, fontWeight: "bold" }} axisLine={false} tickLine={false} dy={5} />
                    <YAxis tick={{ fill: "rgba(255,255,255,0.4)", fontSize: 10 }} axisLine={false} tickLine={false} />
                    <Tooltip
                      formatter={(value: number) => [`${value.toFixed(1)}`, "에너지"]}
                      contentStyle={{ ...CHART_TOOLTIP_STYLE, borderRadius: "12px", border: "1px solid rgba(255,255,255,0.1)", background: "rgba(0,0,0,0.8)" }}
                      cursor={{ fill: "rgba(255,255,255,0.03)" }}
                    />
                    {/* Add ReferenceLine to make the zero line pop out and show negative values properly */}
                    <ReferenceLine y={0} stroke="rgba(255,255,255,0.2)" strokeWidth={1} />
                    {/* Conditional rounding based on value to prevent flat tops and rounded bottoms for negative numbers */}
                    <Bar dataKey="value" shape={(props: any) => {
                      const { x, y, width, height, payload } = props;
                      const isNegative = payload.value < 0;
                      const rad = 8;
                      const yPos = isNegative ? y : y; 
                      const h = isNegative ? Math.abs(height) : height;
                      // Path with rounded top (or rounded bottom if negative)
                      const path = isNegative
                        ? `M${x},${yPos} h${width} v${h - rad} a${rad},${rad} 0 0 1 -${rad},${rad} h-${width - 2 * rad} a${rad},${rad} 0 0 1 -${rad},-${rad} Z`
                        : `M${x},${yPos + rad} a${rad},${rad} 0 0 1 ${rad},-${rad} h${width - 2 * rad} a${rad},${rad} 0 0 1 ${rad},${rad} v${h - rad} h-${width} Z`;
                      return (
                        <g>
                          <path d={path} fill={payload.gradient} />
                          <path d={path} fill="none" stroke="rgba(255,255,255,0.2)" strokeWidth="1" opacity={0.5} />
                        </g>
                      );
                    }} />
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
          <div className="relative pl-6 space-y-8 before:absolute before:inset-y-0 before:left-[11px] before:w-px before:bg-white/10">
            {nearby.map((d: any, i: number) => {
              const isDanger = d.status === "SystemDown";
              const isWarning = d.status === "Overloaded";
              const isCurrent = d.age === age;

              const nodeColor = isDanger ? "bg-red-500 shadow-[0_0_12px_rgba(239,68,68,0.5)]"
                : isWarning ? "bg-orange-500 shadow-[0_0_12px_rgba(249,115,22,0.5)]"
                  : isCurrent ? "bg-celestial-purple shadow-[0_0_12px_rgba(168,85,247,0.5)]"
                    : "bg-white/40";
              const borderColor = isDanger ? "border-red-500/30" : isWarning ? "border-orange-500/30" : isCurrent ? "border-celestial-purple/50" : "border-white/10";
              const bgColor = isDanger ? "bg-red-500/5 hover:bg-red-500/10" : isWarning ? "bg-orange-500/5 hover:bg-orange-500/10" : isCurrent ? "bg-celestial-purple/10 hover:bg-celestial-purple/15" : "bg-white/5 hover:bg-white/[0.07]";

              return (
                <div key={i} className="relative group transition-all">
                  {/* Timeline Node */}
                  <div className={`absolute -left-[30px] top-6 w-[10px] h-[10px] rounded-full ${nodeColor} ring-4 ring-black/50 transition-transform group-hover:scale-125 z-10`} />

                  {/* Content Card */}
                  <div className={`p-5 rounded-2xl border backdrop-blur-sm transition-all duration-300 ${bgColor} ${borderColor} ${isCurrent ? 'scale-[1.02] shadow-lg shadow-celestial-purple/5' : ''}`}>
                    <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 mb-3">
                      <div className="flex items-center gap-3">
                        <span className="text-2xl drop-shadow-sm">{isDanger ? "🚫" : isWarning ? "🔥" : isCurrent ? "🎯" : "ℹ️"}</span>
                        <div>
                          <div className="flex items-center gap-2">
                            <h6 className="text-base font-bold text-white tracking-tight">
                              [{d.age}세] {d.reason}
                            </h6>
                            {isCurrent && <span className="text-[10px] px-2 py-0.5 rounded-full bg-celestial-purple text-white font-bold animate-pulse">현재</span>}
                          </div>
                          <p className="text-xs text-white/50 font-medium mt-0.5">
                            상태: <span className={isDanger ? "text-red-400" : isWarning ? "text-orange-400" : "text-white/70"}>
                              {isDanger ? "위험 (System Down)" : isWarning ? "주의 (Overloaded)" : "안정적 (Normal)"}
                            </span>
                          </p>
                        </div>
                      </div>
                    </div>
                    <div className="pl-9 box-border">
                      <p className="text-sm text-white/80 leading-relaxed bg-black/20 p-3 rounded-xl border border-white/5">
                        <span className="text-celestial-cyan font-bold mr-2">▶ 전략:</span>
                        {d.strategy}
                      </p>
                    </div>
                  </div>
                </div>
              );
            })}
          </div>
        </div>
      )}
    </motion.div>
  );
}
