import { useState } from "react";
import { motion } from "framer-motion";
import { Calendar, Star, Copy, Check, Grid3x3, BarChart3, Zap, AlertCircle } from "lucide-react";
import { SIGN_NAMES, VARGA_DEFS } from "../../constants";
import { getNakshatraInfo } from "../../utils";
import type { VedicAnalysisResult, Yoga } from "../../types";
import { BhavaRadarSection } from "../sections/BhavaRadarSection";
import { AspectsSection } from "../sections/AspectsSection";
import { DashaTimelineSection } from "../sections/DashaTimelineSection";

// ── 남인도 차트 상수 ────────────────────────────────────────────────────
const SOUTH_GRID: (number | null)[][] = [
  [12, 1, 2, 3],
  [11, null, null, 4],
  [10, null, null, 5],
  [9, 8, 7, 6],
];
const PLANET_ABBR: Record<string, string> = {
  Sun: "Su", Moon: "Mo", Mercury: "Me", Venus: "Ve", Mars: "Ma",
  Jupiter: "Ju", Saturn: "Sa", Rahu: "Ra", Ketu: "Ke",
};
const SIGN_ABBR = ["", "Ar", "Ta", "Ge", "Cn", "Le", "Vi", "Li", "Sc", "Sg", "Cp", "Aq", "Pi"];
const SIGN_LORDS = ["", "Mars", "Venus", "Mercury", "Moon", "Sun", "Mercury", "Venus", "Mars", "Jupiter", "Saturn", "Saturn", "Jupiter"];

function formatDeg(deg?: number) {
  if (deg === undefined) return "";
  const d = ((deg % 360) + 360) % 360;
  return `${Math.floor(d % 30)}°`;
}

function SouthIndianChart({
  lagnaRasi,
  planetEntries,
}: {
  lagnaRasi: number;
  planetEntries: { name: string; rasi: number; retro: boolean; deg?: number }[];
}) {
  const bySign: Record<number, string[]> = {};
  if (!bySign[lagnaRasi]) bySign[lagnaRasi] = [];
  bySign[lagnaRasi].unshift("Lg");
  for (const p of planetEntries) {
    if (!bySign[p.rasi]) bySign[p.rasi] = [];
    const abbr = PLANET_ABBR[p.name] ?? p.name.substring(0, 2);
    bySign[p.rasi].push(p.retro ? `${abbr}\u211e` : abbr);
  }
  return (
    <div className="grid grid-cols-4 gap-1 w-full max-w-[280px] shrink-0">
      {SOUTH_GRID.flatMap((row, ri) =>
        row.map((signNum, ci) => {
          if (signNum === null) return <div key={`${ri}-${ci}`} className="aspect-square" />;
          const houseNum = ((signNum - lagnaRasi + 12) % 12) + 1;
          const isLagna = signNum === lagnaRasi;

          // Get the actual planet objects for this sign to show degrees
          const planetsInSign = planetEntries.filter(p => p.rasi === signNum);
          const hasLg = isLagna;
          return (
            <div
              key={`${ri}-${ci}`}
              className={`border min-h-[64px] rounded-lg p-1.5 flex flex-col ${isLagna ? "border-celestial-gold/50 bg-amber-500/[0.08]" : "border-white/10 bg-white/[0.03]"
                }`}
            >
              <div className="flex justify-between items-center mb-1">
                <span className="text-[9px] text-white/35 font-mono">{SIGN_ABBR[signNum]}</span>
                <span className="text-[9px] text-white/25 font-bold">{houseNum}</span>
              </div>
              <div className="flex flex-col gap-0.5 mt-1">
                {hasLg && (
                  <span className="text-[10px] font-bold leading-none text-celestial-gold">Lg</span>
                )}
                {planetsInSign.map((p, i) => {
                  const abbr = PLANET_ABBR[p.name] ?? p.name.substring(0, 2);
                  const isRetro = p.retro;
                  return (
                    <div key={i} className="flex items-center gap-1">
                      <span className={`text-[10px] font-bold leading-none ${isRetro ? "text-amber-300" : "text-white/80"}`}>
                        {abbr}{isRetro && "℞"}
                      </span>
                      {p.deg !== undefined && (
                        <span className="text-[8px] text-white/40 leading-none">{formatDeg(p.deg)}</span>
                      )}
                    </div>
                  );
                })}
              </div>
            </div>
          );
        })
      )}
    </div>
  );
}

const NORTH_HOUSE_CENTERS = [
  null,
  { x: 200, y: 90 },  // H1
  { x: 90, y: 50 },   // H2
  { x: 50, y: 90 },   // H3
  { x: 90, y: 200 },  // H4
  { x: 50, y: 310 },  // H5
  { x: 90, y: 350 },  // H6
  { x: 200, y: 310 }, // H7
  { x: 310, y: 350 }, // H8
  { x: 350, y: 310 }, // H9
  { x: 310, y: 200 }, // H10
  { x: 350, y: 90 },  // H11
  { x: 310, y: 50 },  // H12
];

const NORTH_SIGN_POS = [
  null,
  { x: 195, y: 175 }, // H1 bottom
  { x: 170, y: 25 },  // H2 
  { x: 25, y: 170 },  // H3 
  { x: 175, y: 195 }, // H4 right
  { x: 25, y: 235 },  // H5 
  { x: 170, y: 385 }, // H6 
  { x: 195, y: 235 }, // H7 top
  { x: 240, y: 385 }, // H8 
  { x: 385, y: 235 }, // H9 
  { x: 235, y: 195 }, // H10 left
  { x: 385, y: 170 }, // H11
  { x: 240, y: 25 },  // H12
];

function NorthIndianChart({
  lagnaRasi,
  planetEntries,
}: {
  lagnaRasi: number;
  planetEntries: { name: string; rasi: number; retro: boolean; deg?: number }[];
}) {
  const byHouse: Record<number, typeof planetEntries> = {};
  for (const p of planetEntries) {
    const houseNum = ((p.rasi - lagnaRasi + 12) % 12) + 1;
    if (!byHouse[houseNum]) byHouse[houseNum] = [];
    byHouse[houseNum].push(p);
  }

  return (
    <div className="relative w-full max-w-[280px] shrink-0 aspect-square">
      <svg viewBox="0 0 400 400" className="w-full h-full overflow-hidden rounded-lg bg-white/[0.02] border border-white/10">
        <g stroke="rgba(255,255,255,0.15)" strokeWidth="2" fill="none">
          <rect x="0" y="0" width="400" height="400" />
          <line x1="0" y1="0" x2="400" y2="400" />
          <line x1="400" y1="0" x2="0" y2="400" />
          <line x1="200" y1="0" x2="400" y2="200" />
          <line x1="400" y1="200" x2="200" y2="400" />
          <line x1="200" y1="400" x2="0" y2="200" />
          <line x1="0" y1="200" x2="200" y2="0" />
        </g>

        {/* Draw contents for all 12 houses */}
        {NORTH_HOUSE_CENTERS.slice(1).map((center, index) => {
          if (!center) return null;
          const houseNum = index + 1;
          const signNum = ((lagnaRasi + houseNum - 2) % 12) + 1;
          const pList = byHouse[houseNum] || [];
          const isLagna = houseNum === 1;
          const signPos = NORTH_SIGN_POS[houseNum] || { x: center.x, y: center.y };

          return (
            <g key={`h${houseNum}`}>
              {/* Sign Number */}
              <text x={signPos.x} y={signPos.y} fill="rgba(255,255,255,0.3)" fontSize="14" fontWeight="bold">
                {signNum}
              </text>

              {/* Planets */}
              <foreignObject x={center.x - 45} y={center.y - 40} width="90" height="80">
                <div className="w-full h-full flex flex-col items-center justify-center pointer-events-none">
                  <div className="flex flex-wrap items-center justify-center gap-x-2 gap-y-0.5">
                    {isLagna && <span className="text-[11px] font-bold text-celestial-gold">Lg</span>}
                    {pList.map((p, i) => {
                      const abbr = PLANET_ABBR[p.name] ?? p.name.substring(0, 2);
                      return (
                        <div key={i} className="flex items-center gap-1">
                          <span className={`text-[11px] font-bold leading-none ${p.retro ? "text-amber-300" : "text-white/80"}`}>
                            {abbr}{p.retro && "℞"}
                          </span>
                          {p.deg !== undefined && (
                            <span className="text-[9px] text-white/40 leading-none">{formatDeg(p.deg)}</span>
                          )}
                        </div>
                      );
                    })}
                  </div>
                </div>
              </foreignObject>
            </g>
          );
        })}
      </svg>
    </div>
  );
}

interface VedicChartsTabProps {
  report: VedicAnalysisResult;
}

export function VedicChartsTab({ report }: VedicChartsTabProps) {
  const [copied, setCopied] = useState(false);
  const [selectedVargaId, setSelectedVargaId] = useState<string>("navamsa");
  const [chartStyle, setChartStyle] = useState<"south" | "north">("south");

  if (!report || !report.chart || !report.chart.planets) return null;
  const planets: any[] = report.chart.planets;
  const ascendant: any = report.chart.ascendant;
  const panchanga = report.chart.panchanga;
  const sav = report.chart.sav;
  const vimshopaka = report.chart.vimshopaka_scores;
  const yogas: Yoga[] = report.report?.yogas ?? [];
  const sadeSati = report.report?.sade_sati ?? "None";
  const bhavaStrengths = report.chart.bhava_strengths ?? [];
  const aspects = report.chart.aspects ?? [];
  const dashaTimeline = report.report?.dasha_timeline ?? [];

  // ── 복사 텍스트 생성 헬퍼 ──────────────────────────────────────────
  const fmtPosition = (sidereal_deg: number) => {
    const deg = ((sidereal_deg % 360) + 360) % 360;
    const sign = Math.floor(deg / 30) + 1;
    const d = deg % 30;
    const dd = Math.floor(d);
    const mm = Math.floor((d - dd) * 60);
    return `${dd}°${String(mm).padStart(2, "0")}' ${SIGN_NAMES[sign]}`;
  };

  const buildD1ReportText = () => {
    const header = ["Planet", "Position", "Nakshatra (Pada)", "Nak Lord", "Pada Lord", "Deity", "Purpose"]
      .join("\t");
    const sep = "─".repeat(120);

    const allPoints = [
      ...planets.map((p: any) => ({ name: p.planet, deg: p.sidereal_deg, retro: p.is_retrograde, combust: p.is_combust })),
      { name: "ASC", deg: ascendant?.sidereal_deg, retro: false, combust: false },
    ].filter(x => x.deg != null);

    const rows = allPoints.map(p => {
      const ni = getNakshatraInfo(p.deg);
      const flags = [p.retro ? "℞" : "", p.combust ? "☀" : ""].filter(Boolean).join(" ");
      return [
        `${p.name}${flags ? " " + flags : ""}`,
        fmtPosition(p.deg),
        `${ni.name} (Pada ${ni.pada})`,
        ni.lord,
        ni.padaLord,
        ni.deity,
        ni.purpose,
      ].join("\t");
    });

    return [sep, "D1 Nakshatra Report", sep, header, ...rows, sep].join("\n");
  };

  const buildVargaTableText = () => {
    const header = ["Planet", ...VARGA_DEFS.map(v => v.label)].join("\t");
    const allPoints = [
      ...planets.map((p: any) => ({ name: p.planet, data: p })),
      { name: "ASC", data: ascendant },
    ].filter(x => x.data != null);

    const rows = allPoints.map(({ name, data }) => {
      const signs = VARGA_DEFS.map(v => SIGN_NAMES[data[v.key]] ?? "-");
      return [name, ...signs].join("\t");
    });

    return ["D1–D144 Sign Positions", header, ...rows].join("\n");
  };

  const copyReport = async () => {
    const text = [buildD1ReportText(), "", buildVargaTableText()].join("\n");
    await navigator.clipboard.writeText(text);
    setCopied(true);
    setTimeout(() => setCopied(false), 2500);
  };

  // ── D1 낙샤트라 상세 테이블 데이터 ─────────────────────────────────
  const d1Rows = [
    ...planets.map((p: any) => ({
      name: p.planet as string,
      sidereal_deg: p.sidereal_deg as number,
      retro: p.is_retrograde as boolean,
      combust: p.is_combust as boolean,
    })),
    ...(ascendant ? [{ name: "ASC", sidereal_deg: ascendant.sidereal_deg, retro: false, combust: false }] : []),
  ].filter(x => x.sidereal_deg != null);

  return (
    <motion.div
      key="vedic-charts"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* Panchanga 섹션 */}
      {panchanga && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Calendar className="w-6 h-6 text-celestial-gold" />
            판창가 (Panchanga) — 출생 시각의 천문 요소
          </h5>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
            {[
              { label: "바라 (요일)", value: panchanga.vara },
              { label: "티티 (음력일)", value: `${panchanga.tithi} — ${panchanga.tithi_name}` },
              { label: "낙샤트라 번호", value: `No. ${panchanga.nakshatra}` },
              { label: "요가 (Nitya)", value: `No. ${panchanga.yoga}` },
              { label: "카라나", value: panchanga.karana_name },
            ].map(({ label, value }) => (
              <div key={label} className="p-4 bg-white/5 rounded-xl border border-white/10 text-center">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">{label}</p>
                <p className="text-sm font-semibold text-white leading-snug">{value ?? "—"}</p>
              </div>
            ))}
          </div>
          <div className="grid grid-cols-2 gap-4 mt-4">
            <div className="p-4 bg-white/5 rounded-xl border border-white/10">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">일주 천주 (Day Lord / Hour Lord)</p>
              <p className="text-sm text-white">{panchanga.day_lord} / {panchanga.hour_lord}</p>
            </div>
            <div className="p-4 bg-white/5 rounded-xl border border-white/10">
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">출생 시간대</p>
              <p className="text-sm text-white">{panchanga.is_day_birth ? "☀️ 주간 출생" : "🌙 야간 출생"}</p>
            </div>
          </div>
        </div>
      )}

      {/* ── 요가 (Yogas) 하이라이트 ────────────────────────────────────────── */}
      {yogas.length > 0 && (
        <div className="flex gap-4 overflow-x-auto pb-4 snap-x">
          {yogas.map((yoga: Yoga, idx: number) => {
            const isVeryHigh = yoga.quality === "VeryHigh";
            const isHigh = yoga.quality === "High";
            const isMedium = yoga.quality === "Medium";
            const isWeak = typeof yoga.quality === 'object' && 'Weak' in yoga.quality;

            let colorCls = "border-white/10 bg-white/5 text-white/70";
            let qualityLabel = isWeak ? "약함" : "알 수 없음";
            let badgeCls = "bg-white/10 text-white/50";

            if (isVeryHigh) {
              colorCls = "border-celestial-gold/40 bg-celestial-gold/5 text-celestial-gold";
              qualityLabel = "매우 강함";
              badgeCls = "bg-celestial-gold/20 text-celestial-gold";
            } else if (isHigh) {
              colorCls = "border-celestial-cyan/40 bg-celestial-cyan/5 text-celestial-cyan";
              qualityLabel = "강함";
              badgeCls = "bg-celestial-cyan/20 text-celestial-cyan";
            } else if (isMedium) {
              colorCls = "border-blue-400/40 bg-blue-400/5 text-blue-300";
              qualityLabel = "보통";
              badgeCls = "bg-blue-400/20 text-blue-300";
            }

            return (
              <div
                key={idx}
                className={`min-w-[280px] max-w-[320px] snap-center shrink-0 p-5 rounded-2xl border backdrop-blur-sm ${colorCls} transition-all hover:scale-[1.02]`}
              >
                <div className="flex items-start justify-between mb-2">
                  <h6 className="font-bold text-lg leading-tight tracking-tight drop-shadow-sm">{yoga.name}</h6>
                  <span className={`text-[10px] font-bold px-2 py-0.5 rounded-full ${badgeCls}`}>
                    {qualityLabel}
                  </span>
                </div>
                <p className="text-xs font-semibold opacity-80 mb-3">{yoga.yoga_type}</p>
                <p className="text-sm opacity-90 leading-relaxed break-keep line-clamp-3 mb-4 flex-1">
                  {yoga.description}
                </p>
                <div className="mt-auto flex flex-wrap gap-1.5">
                  {yoga.planets_involved.map((p: string) => (
                    <span key={p} className="text-[10px] font-bold px-1.5 py-0.5 rounded bg-black/30 border border-white/5 opacity-80">
                      {p}
                    </span>
                  ))}
                </div>
              </div>
            );
          })}
        </div>
      )}

      {/* ── D1 낙샤트라 상세 테이블 ─────────────────────────────────── */}
      <div className="glass p-8 rounded-[2rem]">
        <div className="flex items-center justify-between mb-6">
          <h5 className="text-xl font-bold text-white flex items-center gap-3">
            <Star className="w-6 h-6 text-celestial-gold" />
            D1 낙샤트라 리포트 (전체 행성)
          </h5>
          <button
            onClick={copyReport}
            className="flex items-center gap-2 px-4 py-2 rounded-xl bg-white/10 hover:bg-white/20 border border-white/20 text-sm text-white font-semibold transition-all"
          >
            {copied ? <Check className="w-4 h-4 text-green-400" /> : <Copy className="w-4 h-4" />}
            {copied ? "복사됨!" : "리포트 복사 (D1+분할)"}
          </button>
        </div>

        <div className="overflow-x-auto">
          <table className="w-full text-sm">
            <thead>
              <tr className="border-b border-white/10">
                {["행성", "위치 (사이드리얼)", "낙샤트라 (파다)", "파다 범위", "낙샤트라 로드", "파다 로드", "신 (Deity)", "목적 (Purpose)"].map(h => (
                  <th key={h} className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4 whitespace-nowrap">{h}</th>
                ))}
              </tr>
            </thead>
            <tbody className="divide-y divide-white/5">
              {d1Rows.map((row, i) => {
                const ni = getNakshatraInfo(row.sidereal_deg);
                const purposeColor =
                  ni.purpose === "Dharma" ? "text-celestial-gold"
                    : ni.purpose === "Artha" ? "text-green-400"
                      : ni.purpose === "Kama" ? "text-pink-400"
                        : "text-blue-400";
                return (
                  <tr key={i} className="hover:bg-white/3 transition-colors">
                    <td className="py-3 pr-4 font-bold text-white whitespace-nowrap">
                      {row.name}
                      {row.retro && <span className="ml-1.5 text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/40">℞</span>}
                      {row.combust && <span className="ml-1 text-[10px] px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 border border-orange-500/40">☀</span>}
                    </td>
                    <td className="py-3 pr-4 text-white/70 font-mono text-xs whitespace-nowrap">{fmtPosition(row.sidereal_deg)}</td>
                    <td className="py-3 pr-4 text-celestial-cyan font-semibold whitespace-nowrap">
                      {ni.name}
                      <span className="ml-1.5 text-[10px] text-white/40">(Pada {ni.pada})</span>
                    </td>
                    <td className="py-3 pr-4 text-white/40 text-xs whitespace-nowrap">{ni.range}</td>
                    <td className="py-3 pr-4 text-white/70 whitespace-nowrap">{ni.lord}</td>
                    <td className="py-3 pr-4 text-white/70 whitespace-nowrap">{ni.padaLord}</td>
                    <td className="py-3 pr-4 text-white/60 whitespace-nowrap">{ni.deity}</td>
                    <td className="py-3 pr-4 whitespace-nowrap">
                      <span className={`text-xs font-bold ${purposeColor}`}>{ni.purpose}</span>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      </div>

      {/* ── 선택형 분할 차트 (D1~D144) ─────────────────────────────────── */}
      {(() => {
        const vargaDef = VARGA_DEFS.find((v) => v.id === selectedVargaId) ?? VARGA_DEFS[7]; // fallback to D9 navamsa
        return (
          <div className="glass p-8 rounded-[2rem]">
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-2">
              <h5 className="text-xl font-bold text-white flex items-center gap-3">
                <Grid3x3 className="w-6 h-6 text-celestial-cyan" />
                분할 차트 (Varga Charts)
              </h5>
              <div className="flex bg-black/40 border border-white/20 rounded-xl overflow-hidden p-0.5">
                <button
                  onClick={() => setChartStyle("south")}
                  className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${chartStyle === "south" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                >
                  남인도 스타일
                </button>
                <button
                  onClick={() => setChartStyle("north")}
                  className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${chartStyle === "north" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                >
                  북인도 스타일
                </button>
              </div>
              <select
                value={selectedVargaId}
                onChange={(e) => setSelectedVargaId(e.target.value)}
                className="bg-black/40 border border-white/20 text-white text-sm font-semibold rounded-xl px-4 py-2 outline-none focus:border-celestial-cyan appearance-none cursor-pointer"
              >
                {VARGA_DEFS.map((v) => (
                  <option key={v.id} value={v.id} className="bg-slate-900 text-white">
                    {v.label} - {v.name}
                  </option>
                ))}
              </select>
            </div>
            <p className="text-xs text-white/40 mb-6 flex flex-wrap gap-4 items-center mt-2 md:mt-0">
              <span>라그나: <span className="text-white/70 font-semibold">{SIGN_NAMES[ascendant?.[vargaDef.key]] ?? "—"}</span></span>
              <span className="text-white/30 hidden md:inline">|</span>
              <span className="text-white/30">황금 테두리 = 현재 차트 라그나 · 오른쪽 숫자 = 하우스 번호</span>
            </p>
            <div className="flex flex-col md:flex-row gap-8 items-start">
              {chartStyle === "south" ? (
                <SouthIndianChart
                  lagnaRasi={ascendant?.[vargaDef.key] ?? 1}
                  planetEntries={planets.map((p: any) => ({ name: p.planet, rasi: p[vargaDef.key], retro: p.is_retrograde, deg: p.sidereal_deg }))}
                />
              ) : (
                <NorthIndianChart
                  lagnaRasi={ascendant?.[vargaDef.key] ?? 1}
                  planetEntries={planets.map((p: any) => ({ name: p.planet, rasi: p[vargaDef.key], retro: p.is_retrograde, deg: p.sidereal_deg }))}
                />
              )}
              <div className="flex-1 overflow-x-auto w-full">
                <table className="w-full text-sm">
                  <thead>
                    <tr className="border-b border-white/10">
                      {["행성", `${vargaDef.label} 사인`, `${vargaDef.label} 하우스`, "사인 로드", "낙샤트라(파다)", "낙샤트라 로드"].map((h) => (
                        <th key={h} className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">{h}</th>
                      ))}
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-white/5">
                    {[
                      ...planets.map((p: any) => ({
                        name: p.planet,
                        rasi: p[vargaDef.key] as number,
                        deg: p.sidereal_deg as number,
                        retro: p.is_retrograde,
                        combust: p.is_combust,
                      })),
                      ...(ascendant ? [{ name: "ASC", rasi: ascendant[vargaDef.key] as number, deg: ascendant.sidereal_deg, retro: false, combust: false }] : []),
                    ].map((row, i) => {
                      const houseNum = ((row.rasi - (ascendant?.[vargaDef.key] ?? 1) + 12) % 12) + 1;
                      return (
                        <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                          <td className="py-2.5 pr-4 font-bold text-white whitespace-nowrap">
                            {row.name}
                            {row.retro && <span className="ml-1.5 text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/40">℞</span>}
                            {row.combust && <span className="ml-1 text-[10px] px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 border border-orange-500/40">☀</span>}
                          </td>
                          <td className="py-2.5 pr-4 text-celestial-cyan font-semibold whitespace-nowrap">{SIGN_NAMES[row.rasi] ?? "—"}</td>
                          <td className="py-2.5 pr-4 text-white/70 whitespace-nowrap">
                            <span className="px-2 py-0.5 rounded bg-white/10 font-mono text-xs">H{houseNum}</span>
                          </td>
                          <td className="py-2.5 pr-4 text-white/50 whitespace-nowrap">{SIGN_LORDS[row.rasi] ?? "—"}</td>
                          {row.deg !== undefined && row.deg !== null ? (() => {
                            const ni = getNakshatraInfo(row.deg);
                            return (
                              <>
                                <td className="py-2.5 pr-4 text-white/80 whitespace-nowrap">
                                  {ni.name}
                                  <span className="ml-1 text-[10px] text-white/40">(Pada {ni.pada})</span>
                                </td>
                                <td className="py-2.5 pr-4 text-white/60 whitespace-nowrap">{ni.lord}</td>
                              </>
                            );
                          })() : (
                            <>
                              <td className="py-2.5 pr-4 text-white/30 whitespace-nowrap">—</td>
                              <td className="py-2.5 pr-4 text-white/30 whitespace-nowrap">—</td>
                            </>
                          )}
                        </tr>
                      );
                    })}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        );
      })()}

      {/* ── D1-D144 분할 차트 사인 위치 테이블 ──────────────────────── */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
          <Star className="w-6 h-6 text-celestial-purple" />
          D1 – D144 사인 포지션 (전체 분할 차트)
        </h5>
        <p className="text-xs text-white/40 mb-6">각 셀 = 해당 분할 차트에서의 사인 번호 (1=Aries … 12=Pisces)</p>

        <div className="flex gap-4 text-xs text-white/40 mb-4">
          <span className="flex items-center gap-1"><span className="px-1.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/40 font-bold">℞</span> 역행</span>
          <span className="flex items-center gap-1"><span className="px-1.5 rounded bg-orange-500/20 text-orange-400 border border-orange-500/40 font-bold">☀</span> 소각</span>
        </div>

        <div className="overflow-x-auto">
          <table className="text-xs">
            <thead>
              <tr className="border-b border-white/10">
                <th className="text-left text-white/40 font-bold uppercase tracking-wider pb-3 pr-3 whitespace-nowrap">행성</th>
                {VARGA_DEFS.map(v => (
                  <th key={v.id} className="text-center text-white/40 font-bold pb-3 px-2 whitespace-nowrap">
                    <span className="text-white/70">{v.label}</span>
                  </th>
                ))}
              </tr>
              <tr className="border-b border-white/5">
                <th className="pb-2 pr-3"></th>
                {VARGA_DEFS.map(v => (
                  <th key={v.id} className="text-center text-white/25 font-normal pb-2 px-2 whitespace-nowrap text-[10px]">
                    {v.name.substring(0, 8)}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody className="divide-y divide-white/5">
              {[
                ...planets.map((p: any) => ({ name: p.planet, data: p, retro: p.is_retrograde, combust: p.is_combust })),
                ...(ascendant ? [{ name: "ASC", data: ascendant, retro: false, combust: false }] : []),
              ].map((row, i) => (
                <tr key={i} className="hover:bg-white/3 transition-colors">
                  <td className="py-2 pr-3 font-bold text-white whitespace-nowrap">
                    {row.name}
                    {row.retro && <span className="ml-1 text-[9px] px-1 rounded bg-amber-500/20 text-amber-400">℞</span>}
                    {row.combust && <span className="ml-0.5 text-[9px] px-1 rounded bg-orange-500/20 text-orange-400">☀</span>}
                  </td>
                  {VARGA_DEFS.map(v => {
                    const signNum: number = row.data?.[v.key];
                    const signName = SIGN_NAMES[signNum] ?? "—";
                    return (
                      <td key={v.id} className="py-2 px-2 text-center whitespace-nowrap" title={`${row.name} in ${v.name}: ${signName}`}>
                        <span className="inline-block min-w-[24px] text-white/70 font-mono">
                          {signNum ?? "—"}
                        </span>
                        <br />
                        <span className="text-[9px] text-white/30">{signName.substring(0, 3)}</span>
                      </td>
                    );
                  })}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* ── SAV (Sarvashtakavarga) 12하우스 점수 ────────────── */}
      {sav?.points && sav.points.length === 12 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
            <BarChart3 className="w-6 h-6 text-celestial-gold" />
            SAV (사르바아슈타카바르가) — 하우스별 에너지 점수
          </h5>
          <p className="text-xs text-white/40 mb-6">각 하우스의 빈두 포인트 합산입니다. 28점 이상이면 강력한 하우스, 25점 미만이면 약한 하우스로 볼 수 있습니다.</p>
          <div className="grid grid-cols-12 gap-2 items-end h-48">
            {sav.points.map((pt: number, i: number) => {
              const maxPt = Math.max(...sav.points, 1);
              const pct = (pt / maxPt) * 100;
              const isStrong = pt >= 28;
              const isWeak = pt < 25;
              return (
                <div key={i} className="flex flex-col items-center gap-1 h-full justify-end">
                  <span className={`text-[10px] font-bold ${isStrong ? 'text-green-400' : isWeak ? 'text-red-400' : 'text-white/60'}`}>{pt}</span>
                  <div
                    className={`w-full rounded-t-lg transition-all ${isStrong ? 'bg-green-500/60' : isWeak ? 'bg-red-500/40' : 'bg-celestial-cyan/40'}`}
                    style={{ height: `${pct}%`, minHeight: '4px' }}
                  />
                  <span className="text-[9px] text-white/30 font-bold">H{i + 1}</span>
                </div>
              );
            })}
          </div>
        </div>
      )}

      {/* ── 빔쇼파카 발라 (행성 종합 힘) ────────────────── */}
      {vimshopaka && vimshopaka.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
            <Zap className="w-6 h-6 text-celestial-purple" />
            빔쇼파카 발라 — 행성별 종합 힘 (20점 만점)
          </h5>
          <p className="text-xs text-white/40 mb-6">각 행성이 여러 분할 차트(D1~D60)에서 얼마나 좋은 위치에 있는지를 종합한 점수입니다. 15점 이상이면 매우 강력합니다.</p>
          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-white/10">
                  <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">행성</th>
                  <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">6분할 점수</th>
                  <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">16분할 점수</th>
                  <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3">힘 그래프</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                {vimshopaka.map(([planet, score]: [string, any], i: number) => {
                  const shad = score?.shadvarga_score ?? 0;
                  const shod = score?.shodashavarga_score ?? 0;
                  const avg = (shad + shod) / 2;
                  const pct = (avg / 20) * 100;
                  return (
                    <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                      <td className="py-2.5 pr-4 font-bold text-white whitespace-nowrap">{planet}</td>
                      <td className="py-2.5 pr-4 text-celestial-cyan font-mono">{shad.toFixed(1)}</td>
                      <td className="py-2.5 pr-4 text-celestial-purple font-mono">{shod.toFixed(1)}</td>
                      <td className="py-2.5 pr-4 w-48">
                        <div className="flex items-center gap-2">
                          <div className="flex-1 bg-white/10 h-2 rounded-full overflow-hidden">
                            <div
                              className={`h-full rounded-full transition-all ${avg >= 15 ? 'bg-green-500' : avg >= 10 ? 'bg-celestial-cyan' : avg >= 5 ? 'bg-yellow-500' : 'bg-red-500'}`}
                              style={{ width: `${pct}%` }}
                            />
                          </div>
                          <span className="text-xs text-white/50 font-mono w-8 text-right">{avg.toFixed(1)}</span>
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {/* ── 사데사티 (Sade Sati) 경고 배너 ─────────────────────── */}
      {sadeSati !== "None" && (
        <div className={`p-5 rounded-2xl border flex items-center gap-4 ${sadeSati === "Peak" ? "bg-red-500/10 border-red-500/30" : "bg-orange-500/10 border-orange-500/30"
          }`}>
          <AlertCircle className={`w-8 h-8 flex-shrink-0 ${sadeSati === "Peak" ? "text-red-400" : "text-orange-400"}`} />
          <div>
            <h6 className="text-base font-bold text-white mb-1">
              Sade Sati ({sadeSati === "Rising" ? "상승기" : sadeSati === "Peak" ? "절정기" : "하강기"}) 활성 중
            </h6>
            <p className="text-sm text-white/60">
              {sadeSati === "Rising" && "토성이 달의 12번째 하우스에 진입했습니다. 내적 변화의 시기가 시작됩니다."}
              {sadeSati === "Peak" && "토성이 달 위를 지나고 있습니다. 감정적 회복력과 인내에 집중하세요."}
              {sadeSati === "Setting" && "토성이 달의 2번째 하우스로 이동 중입니다. 강도가 점차 약해지고 있습니다."}
            </p>
          </div>
        </div>
      )}

      {/* ── 다샤 타임라인 (Vimshottari Dasha) ───────────── */}
      <DashaTimelineSection periods={dashaTimeline} />

      {/* ── 12하우스 강도 레이더 (Bhava Strength) ───────────── */}
      <BhavaRadarSection strengths={bhavaStrengths} />

      {/* ── 행성 시선 (Aspects / Drishti) ─────────────────── */}
      <AspectsSection aspects={aspects} />
    </motion.div>
  );
}
