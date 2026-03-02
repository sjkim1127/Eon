import { useState } from "react";
import { motion } from "framer-motion";
import { Calendar, Star, Copy, Check } from "lucide-react";
import { SIGN_NAMES, VARGA_DEFS } from "../../constants";
import { getNakshatraInfo } from "../../utils";
import type { VedicAnalysisResult } from "../../types";

interface VedicChartsTabProps {
  report: VedicAnalysisResult;
}

export function VedicChartsTab({ report }: VedicChartsTabProps) {
  const [copied, setCopied] = useState(false);

  if (!report || !report.chart || !report.chart.planets) return null;
  const planets: any[] = report.chart.planets;
  const ascendant: any = report.chart.ascendant;
  const panchanga = report.chart.panchanga;

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
    </motion.div>
  );
}
