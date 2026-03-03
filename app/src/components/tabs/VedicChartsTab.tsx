import { useState } from "react";
import { motion } from "framer-motion";
import { Calendar, Star, Copy, Check, Grid3x3, BarChart3, Zap } from "lucide-react";
import { SIGN_NAMES, VARGA_DEFS } from "../../constants";
import { getNakshatraInfo } from "../../utils";
import type { VedicAnalysisResult } from "../../types";

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

function SouthIndianChart({
  lagnaRasi,
  planetEntries,
}: {
  lagnaRasi: number;
  planetEntries: { name: string; rasi: number; retro: boolean }[];
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
          const planetList = bySign[signNum] ?? [];
          const isLagna = signNum === lagnaRasi;
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
              <div className="flex flex-wrap gap-x-1 gap-y-0.5">
                {planetList.map((p, i) => (
                  <span key={i} className={`text-[10px] font-bold leading-none ${p === "Lg" ? "text-celestial-gold" : p.includes("\u211e") ? "text-amber-300" : "text-white/80"
                    }`}>{p}</span>
                ))}
              </div>
            </div>
          );
        })
      )}
    </div>
  );
}

interface VedicChartsTabProps {
  report: VedicAnalysisResult;
}

export function VedicChartsTab({ report }: VedicChartsTabProps) {
  const [copied, setCopied] = useState(false);

  if (!report || !report.chart || !report.chart.planets) return null;
  const planets: any[] = report.chart.planets;
  const ascendant: any = report.chart.ascendant;
  const panchanga = report.chart.panchanga;
  const sav = report.chart.sav;
  const vimshopaka = report.chart.vimshopaka_scores;

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

      {/* ── D9 Navamsa 차트 ──────────────────────────────────────────── */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
          <Grid3x3 className="w-6 h-6 text-celestial-cyan" />
          D9 나바암사 (Navamsa) — 영혼·결혼 차트
        </h5>
        <p className="text-xs text-white/40 mb-6">
          라그나: <span className="text-white/70 font-semibold">{SIGN_NAMES[ascendant?.navamsa_rasi] ?? "—"}</span>
          <span className="ml-4 text-white/30">황금 테두리 = D9 라그나 · 오른쪽 숫자 = 하우스</span>
        </p>
        <div className="flex flex-col md:flex-row gap-8 items-start">
          <SouthIndianChart
            lagnaRasi={ascendant?.navamsa_rasi ?? 1}
            planetEntries={planets.map((p: any) => ({ name: p.planet, rasi: p.navamsa_rasi, retro: p.is_retrograde }))}
          />
          <div className="flex-1 overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-white/10">
                  {["행성", "D9 사인", "D9 하우스", "사인 로드"].map(h => (
                    <th key={h} className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">{h}</th>
                  ))}
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                {[
                  ...planets.map((p: any) => ({ name: p.planet, rasi: p.navamsa_rasi as number, retro: p.is_retrograde, combust: p.is_combust })),
                  ...(ascendant ? [{ name: "ASC", rasi: ascendant.navamsa_rasi as number, retro: false, combust: false }] : []),
                ].map((row, i) => {
                  const houseNum = ((row.rasi - (ascendant?.navamsa_rasi ?? 1) + 12) % 12) + 1;
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
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </div>

      {/* ── D60 Shastiamsha 차트 ─────────────────────────────────────── */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
          <Grid3x3 className="w-6 h-6 text-celestial-purple" />
          D60 샤쉬탸암사 (Shastiamsha) — 카르마 차트
        </h5>
        <p className="text-xs text-white/40 mb-6">
          라그나: <span className="text-white/70 font-semibold">{SIGN_NAMES[ascendant?.shashtyamsa_rasi] ?? "—"}</span>
          <span className="ml-4 text-white/30">전생 카르마 패턴을 나타내는 60분할 차트</span>
        </p>
        <div className="flex flex-col md:flex-row gap-8 items-start">
          <SouthIndianChart
            lagnaRasi={ascendant?.shashtyamsa_rasi ?? 1}
            planetEntries={planets.map((p: any) => ({ name: p.planet, rasi: p.shashtyamsa_rasi, retro: p.is_retrograde }))}
          />
          <div className="flex-1 overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-white/10">
                  {["행성", "D60 사인", "D60 하우스", "사인 로드"].map(h => (
                    <th key={h} className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">{h}</th>
                  ))}
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                {[
                  ...planets.map((p: any) => ({ name: p.planet, rasi: p.shashtyamsa_rasi as number, retro: p.is_retrograde, combust: p.is_combust })),
                  ...(ascendant ? [{ name: "ASC", rasi: ascendant.shashtyamsa_rasi as number, retro: false, combust: false }] : []),
                ].map((row, i) => {
                  const houseNum = ((row.rasi - (ascendant?.shashtyamsa_rasi ?? 1) + 12) % 12) + 1;
                  return (
                    <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                      <td className="py-2.5 pr-4 font-bold text-white whitespace-nowrap">
                        {row.name}
                        {row.retro && <span className="ml-1.5 text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/40">℞</span>}
                        {row.combust && <span className="ml-1 text-[10px] px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 border border-orange-500/40">☀</span>}
                      </td>
                      <td className="py-2.5 pr-4 text-celestial-purple font-semibold whitespace-nowrap">{SIGN_NAMES[row.rasi] ?? "—"}</td>
                      <td className="py-2.5 pr-4 text-white/70 whitespace-nowrap">
                        <span className="px-2 py-0.5 rounded bg-white/10 font-mono text-xs">H{houseNum}</span>
                      </td>
                      <td className="py-2.5 pr-4 text-white/50 whitespace-nowrap">{SIGN_LORDS[row.rasi] ?? "—"}</td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>
      </div>

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
    </motion.div>
  );
}
