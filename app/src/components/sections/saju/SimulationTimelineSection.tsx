import { useState } from "react";
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
import { Activity } from "lucide-react";
import { CHART_TOOLTIP_STYLE } from "../../../lib/chartTheme";
import { ganziDisplay } from "../../../utils";

export interface SimulationFrame {
  age: number;
  score?: number;
  ganzi?: { stem?: string; branch?: string };
  tags?: string[];
  esil_trace?: string;
}

interface SimulationTimelineSectionProps {
  frames: SimulationFrame[];
}

function getScoreColor(score: number): string {
  if (score >= 70) return "#4ade80";
  if (score >= 40) return "#facc15";
  return "#f87171";
}

export function SimulationTimelineSection({ frames }: SimulationTimelineSectionProps) {
  const [hoveredAge, setHoveredAge] = useState<number | null>(null);

  if (!frames || frames.length === 0) return null;

  const chartData = frames.map((f) => ({
    age: f.age,
    score: Number(f.score ?? 0),
    ganzi: f.ganzi && f.ganzi.stem != null && f.ganzi.branch != null
      ? ganziDisplay({ stem: f.ganzi.stem, branch: f.ganzi.branch })
      : "—",
    tags: (f.tags ?? []).slice(0, 3).join(", "),
  }));

  return (
    <div className="glass p-8 rounded-[2rem]">
      <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
        <Activity className="w-6 h-6 text-celestial-cyan" />
        생애 시뮬레이션 타임라인 (연령별 운세 점수)
      </h5>
      <p className="text-sm text-white/50 mb-4">
        나이별 시뮬레이션 프레임 점수입니다. 막대를 hover하면 해당 연도의 간지·태그를 볼 수 있습니다.
      </p>
      <div className="h-72 w-full">
        <ResponsiveContainer width="100%" height="100%">
          <BarChart
            data={chartData}
            margin={{ top: 8, right: 12, left: 0, bottom: 24 }}
            onMouseMove={(e) => (e?.activePayload?.[0]?.payload?.age != null ? setHoveredAge(e.activePayload[0].payload.age) : setHoveredAge(null))}
            onMouseLeave={() => setHoveredAge(null)}
          >
            <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.08)" />
            <XAxis
              dataKey="age"
              stroke="rgba(255,255,255,0.45)"
              tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 10 }}
              tickLine={false}
              axisLine={{ stroke: "rgba(255,255,255,0.15)" }}
              unit="세"
              interval="preserveStartEnd"
            />
            <YAxis
              domain={[0, 100]}
              stroke="rgba(255,255,255,0.45)"
              tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 10 }}
              tickLine={false}
              axisLine={{ stroke: "rgba(255,255,255,0.15)" }}
            />
            <Tooltip
              content={({ active, payload, label }) => {
                if (!active || !payload?.length) return null;
                const d = payload[0].payload;
                return (
                  <div className="bg-slate-900 border border-white/20 p-4 rounded-xl shadow-xl min-w-[180px]" style={{ background: CHART_TOOLTIP_STYLE.background, border: CHART_TOOLTIP_STYLE.border, borderRadius: CHART_TOOLTIP_STYLE.borderRadius }}>
                    <p className="font-bold text-white mb-2">{label}세</p>
                    <p className="text-sm text-white/80">점수: <span className="font-bold" style={{ color: getScoreColor(d.score) }}>{d.score.toFixed(1)}</span></p>
                    <p className="text-xs text-white/60 mt-1">간지: {d.ganzi}</p>
                    {d.tags && <p className="text-xs text-white/50 mt-1 line-clamp-2">태그: {d.tags}</p>}
                  </div>
                );
              }}
            />
            <Bar dataKey="score" radius={[2, 2, 0, 0]} maxBarSize={12}>
              {chartData.map((entry) => (
                <Cell
                  key={entry.age}
                  fill={getScoreColor(entry.score)}
                  opacity={hoveredAge == null ? 0.85 : hoveredAge === entry.age ? 1 : 0.4}
                />
              ))}
            </Bar>
          </BarChart>
        </ResponsiveContainer>
      </div>
      <div className="flex gap-4 mt-4 text-[10px] text-white/40">
        <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded-sm bg-green-400" /> 70+ 양호</span>
        <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded-sm bg-amber-400" /> 40~69 보통</span>
        <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded-sm bg-red-400" /> 0~39 주의</span>
      </div>
    </div>
  );
}
