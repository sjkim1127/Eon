import { ResponsiveContainer, RadarChart, PolarGrid, PolarAngleAxis, PolarRadiusAxis, Radar, Tooltip } from "recharts";
import { TIER_SCORE_MAP } from "./destinyUtils";

interface Props {
  domainTiers: any[];
}

export function RadarChartSection({ domainTiers }: Props) {
  if (!domainTiers || domainTiers.length === 0) return null;

  const radarData = domainTiers.map(({ domain, tier }) => ({
    subject: domain.split("·")[0],
    fullMark: 5,
    value: TIER_SCORE_MAP[tier] ?? 1,
  }));

  return (
    <div className="glass p-6 rounded-2xl">
      <h5 className="text-lg font-bold text-white mb-1">분야별 티어</h5>
      <p className="text-xs text-white/40 mb-4">베딕 12하우스 강도 기반 (S+=10 · S=9 · A+=8 · A=7 · B+=6 · B=5 · C+=4 · C=3 · D+=2 · D=1)</p>
      <ResponsiveContainer width="100%" height={300}>
        <RadarChart data={radarData} margin={{ top: 10, right: 20, bottom: 10, left: 20 }}>
          <PolarGrid stroke="rgba(255,255,255,0.1)" />
          <PolarAngleAxis
            dataKey="subject"
            tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 11 }}
          />
          <PolarRadiusAxis
            angle={90}
            domain={[0, 10]}
            tick={{ fill: "rgba(255,255,255,0.3)", fontSize: 9 }}
            tickCount={6}
          />
          <Radar
            name="티어"
            dataKey="value"
            stroke="#a78bfa"
            fill="#a78bfa"
            fillOpacity={0.35}
            strokeWidth={2}
          />
          <Tooltip
            contentStyle={{
              background: "rgba(15,10,30,0.9)",
              border: "1px solid rgba(167,139,250,0.3)",
              borderRadius: 12,
              fontSize: 12,
            }}
            formatter={(val: number) => {
              const t = Object.entries(TIER_SCORE_MAP).find(([, v]) => v === val)?.[0] ?? "?";
              return [`${t} (${val})`, "티어"];
            }}
          />
        </RadarChart>
      </ResponsiveContainer>
      {/* 티어 범례 */}
      <div className="flex flex-wrap justify-center gap-1.5 mt-2">
        {[
          { t: "S+", c: "bg-orange-500/30 text-orange-300 border-orange-500/40" },
          { t: "S",  c: "bg-amber-500/30 text-amber-300 border-amber-500/40" },
          { t: "A+", c: "bg-lime-500/30 text-lime-300 border-lime-500/40" },
          { t: "A",  c: "bg-emerald-500/30 text-emerald-300 border-emerald-500/40" },
          { t: "B+", c: "bg-sky-500/30 text-sky-300 border-sky-500/40" },
          { t: "B",  c: "bg-celestial-cyan/20 text-celestial-cyan border-celestial-cyan/40" },
          { t: "C+", c: "bg-violet-500/30 text-violet-300 border-violet-500/40" },
          { t: "C",  c: "bg-slate-500/20 text-slate-300 border-slate-500/40" },
          { t: "D+", c: "bg-orange-500/20 text-orange-400 border-orange-500/30" },
          { t: "D",  c: "bg-rose-500/20 text-rose-400 border-rose-500/40" },
        ].map(({ t, c }) => (
          <span key={t} className={`px-2 py-0.5 rounded text-xs font-bold border ${c}`}>{t}</span>
        ))}
      </div>
      {/* 보조 배지 그리드 */}
      <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-2 mt-4">
        {domainTiers.map(({ house, domain, tier }) => {
          const tierColor =
            tier === "S+" ? "bg-orange-500/15 text-orange-300 border-orange-500/30"
            : tier === "S"  ? "bg-amber-500/15 text-amber-400 border-amber-500/30"
            : tier === "A+" ? "bg-lime-500/15 text-lime-400 border-lime-500/30"
            : tier === "A"  ? "bg-emerald-500/15 text-emerald-400 border-emerald-500/30"
            : tier === "B+" ? "bg-sky-500/15 text-sky-400 border-sky-500/30"
            : tier === "B"  ? "bg-celestial-cyan/15 text-celestial-cyan border-celestial-cyan/30"
            : tier === "C+" ? "bg-violet-500/15 text-violet-400 border-violet-500/30"
            : tier === "C"  ? "bg-slate-500/15 text-slate-300 border-slate-500/30"
            : tier === "D+" ? "bg-orange-500/10 text-orange-400 border-orange-500/20"
            : "bg-rose-500/15 text-rose-400 border-rose-500/30";
          return (
            <div key={house} className={`p-2 rounded-lg border text-center ${tierColor}`}>
              <p className="text-[9px] text-white/50 mb-0.5 leading-tight">{domain}</p>
              <p className="text-sm font-black">{tier}</p>
            </div>
          );
        })}
      </div>
    </div>
  );
}
