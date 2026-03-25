import { useState } from "react";
import { motion } from "framer-motion";
import { Calendar, Star, Copy, Check, Grid3x3, BarChart3, Zap, AlertCircle, Compass, Heart, Clock, Users } from "lucide-react";
import { toast } from "sonner";
import { SIGN_NAMES, VARGA_DEFS } from "../../constants";
import { getNakshatraInfo, getVargaEffectiveLongitude, formatSiderealPosition, buildNakshatraMarkdownRows, cn } from "../../utils";
import type { VedicAnalysisResult, Yoga } from "../../types";

import { SouthIndianChart } from "../charts/SouthIndianChart";
import { NorthIndianChart } from "../charts/NorthIndianChart";
import { VargaSignPositionsTable } from "../vedic/VargaSignPositionsTable";
import { BavHeatmap } from "../vedic/BavHeatmap";
import { SavScoreChart } from "../vedic/SavScoreChart";
import { VimshopakaTable } from "../vedic/VimshopakaTable";

import { BhavaRadarSection } from "../sections/BhavaRadarSection";
import { AspectsSection } from "../sections/AspectsSection";
import { DashaTimelineSection } from "../sections/DashaTimelineSection";
import { GocharaSection } from "../sections/GocharaSection";
import { AvasthaKarakaSection } from "../sections/AvasthaKarakaSection";
import { VargaNakshatraTable } from "../sections/VargaNakshatraTable";

interface VedicChartsTabProps {
  report: VedicAnalysisResult;
}

export function VedicChartsTab({ report }: VedicChartsTabProps) {
  const [copied, setCopied] = useState(false);
  const [selectedVargaId, setSelectedVargaId] = useState<string>("navamsa");
  const [chartStyle, setChartStyle] = useState<"south" | "north">("south");

  if (!report || !report.chart || !report.chart.planets) return null;
  const chart = report.chart;
  const planets: any[] = chart.planets;
  const ascendant: any = chart.ascendant;
  const panchanga = report.chart.panchanga;
  const sav = report.chart.sav;
  const vimshopaka = report.chart.vimshopaka_scores;
  const yogas: Yoga[] = report.report?.yogas ?? [];
  const sadeSati = report.report?.sade_sati ?? "None";
  const bhavaStrengths = report.chart.bhava_strengths ?? [];
  const aspects = report.chart.aspects ?? [];
  const dashaTimeline = report.report?.dasha_timeline ?? [];

  // ── 복사 텍스트 생성 헬퍼 ──────────────────────────────────────────
  const buildD1ReportText = () => {
    const vargaReports = report.varga_nakshatra_reports;
    const d1Report = vargaReports?.reports?.["rasi"];

    if (d1Report?.rows?.length) {
      const mdRows = buildNakshatraMarkdownRows(d1Report.rows, false);
      return [
        "## D1 낙샤트라 리포트 (전체 행성)",
        "> D1 낙샤트라는 본 차트 기준입니다.",
        "",
        ...mdRows,
      ].join("\n");
    }

    const allPoints = [
      ...planets.map((p: any) => ({ name: p.planet as string, deg: p.sidereal_deg as number, retro: p.is_retrograde as boolean, combust: p.is_combust as boolean })),
      { name: "ASC", deg: ascendant?.sidereal_deg as number, retro: false, combust: false },
    ].filter(x => x.deg != null);

    const rows = allPoints.map(p => {
      const ni = getNakshatraInfo(p.deg);
      return {
        planet: p.name,
        position_str: formatSiderealPosition(p.deg),
        nakshatra_name: ni.name,
        pada: ni.pada,
        pada_range: ni.range,
        nakshatra_lord: ni.lord,
        pada_lord: ni.padaLord,
        deity: ni.deity,
        purpose: ni.purpose,
        is_retrograde: p.retro,
        is_combust: p.combust,
      };
    });

    const mdRows = buildNakshatraMarkdownRows(rows, false);
    return [
      "## D1 낙샤트라 리포트 (전체 행성)",
      "> D1 낙샤트라는 본 차트 기준입니다.",
      "",
      ...mdRows,
    ].join("\n");
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

  const buildVargaNakshatraText = (): string => {
    const vargaReps = report.varga_nakshatra_reports?.reports;
    if (!vargaReps || Object.keys(vargaReps).length === 0) return buildVargaTableText();

    const out: string[] = [
      "## 분할 차트 낙샤트라 리포트 (Varga D-Charts)",
      "> 분할 차트 낙샤트라는 해당 분할 좌표 기준입니다.",
      "",
    ];
    for (const vargaDef of VARGA_DEFS) {
      const rep = vargaReps[vargaDef.id];
      if (!rep?.rows?.length) continue;
      const lagna = rep.lagna_rasi ? ` (라그나: ${SIGN_NAMES[rep.lagna_rasi] ?? rep.lagna_rasi})` : "";
      out.push(`### ${rep.varga_label}${lagna}`);
      const mdRows = buildNakshatraMarkdownRows(rep.rows, true);
      out.push(...mdRows);
      out.push("");
    }
    return out.join("\n");
  };

  const copyReport = async () => {
    const r = report.report;
    const ch = report.chart;
    const lines: string[] = [];

    // ① 메타 요약
    lines.push("# 베딕 점성학 분석 리포트");
    lines.push(`전체 차트 강도: ${Math.round(r?.overall_strength_score ?? 0)} / 600`);
    lines.push(`사데사티: ${r?.sade_sati ?? "None"}`);
    lines.push(`현재 다샤: ${(r?.dasha_focus ?? "").replace("Current Major Period: ", "")}`);
    lines.push("");

    // ② 카라카
    if (r?.primary_karakas) {
      lines.push("## 카라카 (Karakas)");
      lines.push(`- 영혼 지표 (Atmakaraka): ${r.primary_karakas.atmakaraka}`);
      lines.push(`- 직업 지표 (Amatyakaraka): ${r.primary_karakas.amatyakaraka}`);
      lines.push(`- 파트너 지표 (Darakaraka): ${r.primary_karakas.darakaraka}`);
      lines.push("");
    }

    // ③ 낙샤트라 청사진
    if (r?.nakshatra_info) {
      lines.push("## 낙샤트라 청사진");
      lines.push(r.nakshatra_info);
      lines.push("");
    }

    // ④ 역행·연소 행성
    const retro = planets.filter((p: any) => p.is_retrograde).map((p: any) => p.planet);
    const combust = planets.filter((p: any) => p.is_combust).map((p: any) => p.planet);
    if (retro.length > 0 || combust.length > 0) {
      lines.push("## 약화 행성");
      if (retro.length > 0) lines.push(`- 역행(℞): ${retro.join(", ")} — 해당 행성 주제 내면화·지연 경향`);
      if (combust.length > 0) lines.push(`- 연소(☀): ${combust.join(", ")} — 태양 광채에 행성력 흡수`);
      lines.push("");
    }

    // ⑤ 하우스 요약
    if (r?.house_summary?.length) {
      lines.push("## 하우스 강도 요약");
      lines.push("| 하우스 | 점수 | 등급 |");
      lines.push("|--------|------|------|");
      for (const h of r.house_summary as { house: number; total_score: number; rating: string }[]) {
        lines.push(`| H${h.house} | ${Math.round(h.total_score)} | ${h.rating} |`);
      }
      lines.push("");
    }

    // ⑥ SAV 점수
    const savPts = ch?.sav?.points ?? [];
    if (Array.isArray(savPts) && savPts.length === 12) {
      lines.push("## SAV (Sarvashtakavarga) 하우스별 빈두");
      lines.push(savPts.map((p: number, i: number) => `H${i + 1}:${p}`).join("  "));
      lines.push("");
    }

    // ⑦ 요가
    if (r?.yogas?.length) {
      lines.push("## 요가 (Yogas)");
      for (const y of r.yogas as { name: string; quality: string | object; description: string; planets_involved: string[] }[]) {
        const q = typeof y.quality === "string" ? y.quality : Object.keys(y.quality ?? {})[0];
        lines.push(`### ${y.name} [${q}]`);
        lines.push(`행성: ${y.planets_involved.join(", ")}`);
        lines.push(y.description);
        lines.push("");
      }
    }

    // ⑧ 다샤 타임라인
    if (dashaTimeline.length > 0) {
      lines.push("## 다샤 타임라인 (Vimshottari Dasha)");
      for (const d of dashaTimeline) {
        const s = new Date(d.start_time);
        const e = new Date(d.end_time);
        const yrs = ((e.getTime() - s.getTime()) / (365.2425 * 86400000)).toFixed(1);
        lines.push(`- ${d.lord}: ${s.getFullYear()}.${String(s.getMonth()+1).padStart(2,"0")} ~ ${e.getFullYear()}.${String(e.getMonth()+1).padStart(2,"0")} (${yrs}년)`);
        for (const sub of (d.sub_dashas ?? [])) {
          const ss = new Date(sub.start_time);
          const se = new Date(sub.end_time);
          lines.push(`  - ${sub.lord}: ${ss.getFullYear()}.${String(ss.getMonth()+1).padStart(2,"0")} ~ ${se.getFullYear()}.${String(se.getMonth()+1).padStart(2,"0")}`);
        }
      }
      lines.push("");
    }

    // ⑨ 고차라 (Gochara)
    const transits = report.gochara?.transits ?? [];
    if (transits.length > 0) {
      lines.push("## 고차라 트랜짓 (Gochara)");
      lines.push("| 행성 | 현재 라시 | 달로부터 하우스 | 길흉 | Murti |");
      lines.push("|------|-----------|-----------------|------|-------|");
      for (const t of transits as { planet: string; current_rasi: number; house_from_moon: number; is_benefic_transit: boolean; murti: string }[]) {
        const rasi = SIGN_NAMES[t.current_rasi] ?? t.current_rasi;
        const benefic = t.is_benefic_transit ? "길 ✅" : "흉 ⚠️";
        lines.push(`| ${t.planet} | ${rasi} | H${t.house_from_moon} | ${benefic} | ${t.murti} |`);
      }
      lines.push("");
    }

    // ⑩ D1 낙샤트라 + 바르가
    lines.push(buildD1ReportText());
    lines.push("");
    lines.push(buildVargaNakshatraText());

    try {
      await navigator.clipboard.writeText(lines.join("\n"));
      setCopied(true);
      setTimeout(() => setCopied(false), 2500);
    } catch {
      toast.error("클립보드 복사에 실패했습니다.");
    }
  };

  const vargaReports = report.varga_nakshatra_reports;
  const reportsMap = vargaReports?.reports;

  const ratingLabel = (rating: string) => {
    if (rating === "Excellent") return "최상";
    if (rating === "Strong") return "강함";
    if (rating === "Average") return "보통";
    return "약함";
  };

  return (
    <motion.div
      key="vedic-charts"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* ── Overview 통합 ─────────────────────────────────────────────── */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
          <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
            <Heart className="w-32 h-32" />
          </div>
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            영혼의 지표 (Atmakaraka)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {report.report?.primary_karakas.atmakaraka}
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            이번 생에서 영혼이 추구하는 가장 강력한 욕망과 핵심 과제를 나타냅니다.
          </p>
        </div>

        <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
          <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
            직업 지표 (Amatyakaraka)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {report.report?.primary_karakas.amatyakaraka}
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            직업·사회적 역할에서 영혼을 보필하는 행성입니다.
          </p>
        </div>

        <div className="glass p-8 rounded-[2rem] border-celestial-cyan/20 bg-celestial-cyan/5">
          <p className="text-celestial-cyan/80 text-sm font-bold uppercase tracking-wider mb-2">
            파트너 지표 (Darakaraka)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {report.report?.primary_karakas.darakaraka}
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            배우자·가까운 파트너와의 관계 패턴을 나타내는 행성입니다.
          </p>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
          <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
            현재 대운 (Dasha)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {report.report?.dasha_focus.replace("Current Major Period: ", "")}
          </h4>
          <div className="flex items-center gap-2 text-sm text-white/60">
            <Clock className="w-4 h-4" />
            <span>인생의 현재 단계에서 가장 강력한 영향을 미치는 기운입니다.</span>
          </div>
        </div>

        <div className="glass p-8 rounded-[2rem]">
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            전체 차트 강도
          </p>
          <div className="flex items-baseline gap-2 mb-4">
            <h4 className="text-5xl font-black text-gradient leading-none">
              {Math.round(report.report?.overall_strength_score ?? 0)}
            </h4>
            <span className="text-white/20 font-bold">/ 600</span>
          </div>
          <div className="w-full bg-white/5 h-2 rounded-full overflow-hidden">
            <div
              className="bg-celestial-purple h-full rounded-full transition-all duration-1000"
              style={{ width: `${((report.report?.overall_strength_score ?? 0) / 600) * 100}%` }}
            />
          </div>
        </div>
      </div>

      <div className="glass p-10 rounded-[2.5rem]">
        <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
          <Star className="w-6 h-6 text-celestial-gold" />
          낙샤트라 청사진
        </h5>
        <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
          <p className="text-white text-lg font-medium leading-relaxed">
            {report.report?.nakshatra_info}
          </p>
        </div>
      </div>

      {/* ── 역행·연소 행성 요약 ─────────────────────────────── */}
      {(() => {
        const retro = planets.filter((p: any) => p.is_retrograde);
        const combust = planets.filter((p: any) => p.is_combust);
        if (retro.length === 0 && combust.length === 0) return null;
        return (
          <div className="glass p-6 rounded-[2rem]">
            <h5 className="text-sm font-bold text-white/60 uppercase tracking-wider mb-4 flex items-center gap-2">
              <AlertCircle className="w-4 h-4 text-amber-400" />
              약화 행성 — 역행(Retrograde) / 연소(Combust)
            </h5>
            <div className="flex flex-wrap gap-3">
              {retro.map((p: any) => (
                <div key={`retro-${p.planet}`}
                  className="flex items-center gap-2 px-3 py-2 rounded-xl bg-violet-500/10 border border-violet-500/25"
                  title="역행: 행성이 겉보기 역방향으로 이동하는 구간. 해당 행성의 카라카 영역이 내면화·지연되는 경향"
                >
                  <span className="text-[10px] font-black text-violet-400 bg-violet-500/20 px-1.5 py-0.5 rounded">℞</span>
                  <span className="text-sm font-semibold text-white">{p.planet}</span>
                  <span className="text-[10px] text-violet-300/60">역행</span>
                </div>
              ))}
              {combust.map((p: any) => (
                <div key={`comb-${p.planet}`}
                  className="flex items-center gap-2 px-3 py-2 rounded-xl bg-orange-500/10 border border-orange-500/25"
                  title="연소: 태양과 너무 가까워 행성의 빛이 소실되는 상태. 해당 행성의 시그니피케이터가 자아(태양)에 흡수됨"
                >
                  <span className="text-[10px] font-black text-orange-400 bg-orange-500/20 px-1.5 py-0.5 rounded">☀</span>
                  <span className="text-sm font-semibold text-white">{p.planet}</span>
                  <span className="text-[10px] text-orange-300/60">연소</span>
                </div>
              ))}
            </div>
            <p className="text-[11px] text-white/30 mt-3 leading-relaxed">
              역행(℞): 해당 행성 주제가 내면화·지연됩니다. 연소(☀): 태양의 광채에 행성력이 흡수돼 직접적 표현이 약해집니다.
            </p>
          </div>
        );
      })()}

      <section>
        <h5 className="text-xl font-bold text-white mb-6">하우스(Bhava)별 에너지 역량 상세</h5>
        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
          {report.report?.house_summary.map((house: any) => {
            const bhava = chart?.bhava_strengths?.find((b: any) => b.house === house.house);
            return (
              <div
                key={house.house}
                className="glass p-6 rounded-2xl text-center glass-hover cursor-help"
              >
                <p className="text-xs text-white/30 font-bold mb-1">하우스 {house.house}</p>
                <p className="text-2xl font-bold text-white mb-2">
                  {Math.round(house.total_score)}
                </p>
                <span
                  className={cn(
                    "px-2 py-0.5 rounded text-[10px] font-black uppercase",
                    house.rating === "Excellent"
                      ? "bg-green-500/20 text-green-400"
                      : house.rating === "Strong"
                        ? "bg-blue-500/20 text-blue-400"
                        : house.rating === "Average"
                          ? "bg-yellow-500/20 text-yellow-400"
                          : "bg-red-500/20 text-red-400"
                  )}
                >
                  {ratingLabel(house.rating)}
                </span>
                {bhava && (
                  <div className="mt-3 space-y-1 text-left">
                    {[
                      { label: "주인 행성", value: bhava.lord_score },
                      { label: "방위 힘", value: bhava.dig_score },
                      { label: "시선 영향", value: bhava.drishti_score },
                    ].map(({ label, value }) => (
                      <div key={label}>
                        <div className="flex justify-between text-[9px] text-white/40 mb-0.5">
                          <span>{label}</span>
                          <span>{(value ?? 0).toFixed(0)}</span>
                        </div>
                        <div className="w-full bg-white/10 h-1 rounded-full overflow-hidden">
                          <div className="h-full rounded-full bg-celestial-cyan/60 transition-all" style={{ width: `${Math.min(100, Math.max(0, (value ?? 0) / 60 * 100))}%` }} />
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </section>

      {chart?.karakas && chart.karakas.length > 0 && (
        <section className="glass p-10 rounded-[2.5rem]">
          <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
            <Users className="w-6 h-6 text-celestial-purple" />
            제미니 카라카 — 8가지 인생 역할 배정
          </h5>
          <p className="text-xs text-white/40 mb-6">태양계 행성들이 당신의 인생에서 맡는 구체적인 역할입니다. 사이드리얼 도수가 높을수록 그 역할의 영향이 큽니다.</p>
          <div className="grid grid-cols-2 sm:grid-cols-4 gap-4">
            {chart.karakas.map((k: any, i: number) => {
              const roleKr: Record<string, string> = {
                Atmakaraka: "영혼 (나 자신)",
                Amatyakaraka: "직업 / 재능",
                Bhratrukaraka: "형제 / 자매",
                Matrukaraka: "어머니 / 보호",
                Pitrikaraka: "아버지 / 권위",
                Putrakaraka: "자식 / 창작",
                Gnatikaraka: "경쟁자 / 친척",
                Darakaraka: "배우자 / 파트너",
              };
              const colors = [
                "border-celestial-gold/40 bg-celestial-gold/10",
                "border-celestial-purple/40 bg-celestial-purple/10",
                "border-green-500/30 bg-green-500/10",
                "border-pink-500/30 bg-pink-500/10",
                "border-blue-500/30 bg-blue-500/10",
                "border-orange-500/30 bg-orange-500/10",
                "border-red-500/30 bg-red-500/10",
                "border-celestial-cyan/30 bg-celestial-cyan/10",
              ];
              return (
                <div key={i} className={`p-4 rounded-2xl border ${colors[i % colors.length]}`}>
                  <p className="text-[10px] font-bold text-white/40 uppercase tracking-wider mb-1">{roleKr[k.role] ?? k.role}</p>
                  <p className="text-lg font-bold text-white">{k.planet}</p>
                  <p className="text-xs text-white/40 mt-1">{(k.degree_in_rasi ?? 0).toFixed(2)}°</p>
                </div>
              );
            })}
          </div>
        </section>
      )}
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

      {/* 하우스 쿠스프 */}
      {report.chart?.house_cusps?.length === 12 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
            <Compass className="w-6 h-6 text-celestial-cyan" />
            하우스 쿠스프 (House Cusps)
          </h5>
          <p className="text-xs text-white/40 mb-4">각 하우스(바바) 경계의 사이드리얼 경도입니다.</p>
          <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-3">
            {report.chart.house_cusps.map((deg: number, i: number) => (
              <div key={i} className="p-3 bg-white/5 rounded-xl border border-white/10">
                <p className="text-xs text-white/40 font-bold mb-1">H{i + 1}</p>
                <p className="text-sm font-semibold text-white font-mono">{formatSiderealPosition(deg)}</p>
              </div>
            ))}
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

      {/* ── 분할 차트 + 낙샤트라 리포트 (통합) ─────────────────────────────────── */}
      {(() => {
        const vargaDef = VARGA_DEFS.find((v) => v.id === selectedVargaId) ?? VARGA_DEFS[7];
        const backendReport = reportsMap?.[vargaDef.id];
        const lagnaRasi: number = backendReport?.lagna_rasi ?? ascendant?.[vargaDef.key] ?? 1;

        const rows = backendReport?.rows?.length
          ? backendReport.rows
          : [
            ...planets.map((p: any) => {
              const rasi: number = p[vargaDef.key] ?? 1;
              const deg: number = p.sidereal_deg ?? 0;
              const effectiveDeg = vargaDef.id === "rasi"
                ? deg
                : getVargaEffectiveLongitude(deg, rasi, vargaDef.divisionCount);
              const ni = getNakshatraInfo(effectiveDeg);
              const house = ((rasi - lagnaRasi + 12) % 12) + 1;
              return {
                planet: p.planet as string,
                position_str: formatSiderealPosition(effectiveDeg),
                sign: rasi,
                house,
                nakshatra: 0,
                nakshatra_name: ni.name,
                pada: ni.pada,
                pada_range: ni.range,
                nakshatra_lord: ni.lord,
                pada_lord: ni.padaLord,
                deity: ni.deity,
                purpose: ni.purpose,
                is_retrograde: p.is_retrograde as boolean,
                is_combust: p.is_combust as boolean,
              };
            }),
            ...(ascendant ? (() => {
              const rasi: number = ascendant[vargaDef.key] ?? 1;
              const deg: number = ascendant.sidereal_deg ?? 0;
              const effectiveDeg = vargaDef.id === "rasi"
                ? deg
                : getVargaEffectiveLongitude(deg, rasi, vargaDef.divisionCount);
              const ni = getNakshatraInfo(effectiveDeg);
              const house = ((rasi - lagnaRasi + 12) % 12) + 1;
              return [{
                planet: "ASC",
                position_str: formatSiderealPosition(effectiveDeg),
                sign: rasi,
                house,
                nakshatra: 0,
                nakshatra_name: ni.name,
                pada: ni.pada,
                pada_range: ni.range,
                nakshatra_lord: ni.lord,
                pada_lord: ni.padaLord,
                deity: ni.deity,
                purpose: ni.purpose,
                is_retrograde: false,
                is_combust: false,
              }];
            })() : []),
          ];

        return (
          <div className="glass p-8 rounded-[2rem]">
            {/* 헤더 */}
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-2">
              <h5 className="text-xl font-bold text-white flex items-center gap-3">
                <Grid3x3 className="w-6 h-6 text-celestial-cyan" />
                분할 차트 (Varga Charts)
              </h5>
              <div className="flex items-center gap-3 flex-wrap">
                <div className="flex bg-black/40 border border-white/20 rounded-xl overflow-hidden p-0.5">
                  <button
                    onClick={() => setChartStyle("south")}
                    className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${chartStyle === "south" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                  >
                    남인도
                  </button>
                  <button
                    onClick={() => setChartStyle("north")}
                    className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${chartStyle === "north" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                  >
                    북인도
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
                <button
                  onClick={copyReport}
                  className="flex items-center gap-2 px-4 py-2 rounded-xl bg-white/10 hover:bg-white/20 border border-white/20 text-sm text-white font-semibold transition-all"
                >
                  {copied ? <Check className="w-4 h-4 text-green-400" /> : <Copy className="w-4 h-4" />}
                  {copied ? "복사됨!" : "전체 리포트 복사"}
                </button>
              </div>
            </div>
            <p className="text-xs text-white/40 mb-6 flex flex-wrap gap-4 items-center mt-2">
              <span>라그나: <span className="text-white/70 font-semibold">{SIGN_NAMES[lagnaRasi] ?? "—"}</span></span>
              {backendReport && <span className="text-celestial-cyan/60 text-[10px]">✓ 백엔드 정밀값</span>}
              <span className="text-white/30 hidden md:inline">|</span>
              <span className="text-white/30">황금 테두리 = 라그나 · 오른쪽 숫자 = 하우스 번호</span>
            </p>
            {/* 차트 + 낙샤트라 테이블 */}
            <div className="flex flex-col md:flex-row gap-8 items-start">
              {chartStyle === "south" ? (
                <SouthIndianChart
                  lagnaRasi={lagnaRasi}
                  planetEntries={planets.map((p: any) => ({ name: p.planet, rasi: p[vargaDef.key], retro: p.is_retrograde, deg: p.sidereal_deg }))}
                />
              ) : (
                <NorthIndianChart
                  lagnaRasi={lagnaRasi}
                  planetEntries={planets.map((p: any) => ({ name: p.planet, rasi: p[vargaDef.key], retro: p.is_retrograde, deg: p.sidereal_deg }))}
                />
              )}
              <div className="flex-1 overflow-x-auto w-full">
                <VargaNakshatraTable
                  title=""
                  vargaLabel={vargaDef.label}
                  rows={rows}
                  showHouse={vargaDef.id !== "rasi"}
                />
              </div>
            </div>
          </div>
        );
      })()}

      {/* ── D1-D144 분할 차트 사인 위치 테이블 ──────────────────────── */}
      <div className="glass p-8 rounded-[2rem]">
        <div className="flex flex-wrap items-center justify-between gap-4 mb-2">
          <h5 className="text-xl font-bold text-white flex items-center gap-3">
            <Star className="w-6 h-6 text-celestial-purple" />
            D1 – D144 사인 포지션 (전체 분할 차트)
          </h5>
          <button
            type="button"
            onClick={copyReport}
            className="flex items-center gap-2 px-4 py-2 rounded-xl bg-celestial-purple/20 hover:bg-celestial-purple/30 border border-celestial-purple/40 text-celestial-purple text-sm font-semibold transition-colors"
          >
            {copied ? <Check className="w-4 h-4 text-green-400" /> : <Copy className="w-4 h-4" />}
            {copied ? "복사됨!" : "전체 리포트 복사"}
          </button>
        </div>
        <p className="text-xs text-white/40 mb-6">각 셀 = 해당 분할 차트에서의 사인 번호 (1=Aries … 12=Pisces)</p>

        <div className="flex gap-4 text-xs text-white/40 mb-4">
          <span className="flex items-center gap-1"><span className="px-1.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/40 font-bold">℞</span> 역행</span>
          <span className="flex items-center gap-1"><span className="px-1.5 rounded bg-orange-500/20 text-orange-400 border border-orange-500/40 font-bold">☀</span> 소각</span>
        </div>

        <VargaSignPositionsTable planets={planets} ascendant={ascendant} />
      </div>

      {/* ── 행성별 빈나슈타카바르가 (BAV) 히트맵 ────────────── */}
      {report.chart.bav && report.chart.bav.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
            <BarChart3 className="w-6 h-6 text-celestial-gold" />
            빈나슈타카바르가 (BAV) — 행성×하우스 빈두 히트맵
          </h5>
          <p className="text-xs text-white/40 mb-6">
            각 행성이 12개 하우스에 기여하는 빈두 포인트입니다. 녹색에 가까울수록 강한 하우스를 지원합니다.
          </p>
          <BavHeatmap bav={report.chart.bav} savPoints={report.chart.sav?.points as number[]} />
        </div>
      )}

      {/* ── SAV (Sarvashtakavarga) 12하우스 점수 ────────────── */}
      {sav?.points && sav.points.length === 12 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
            <BarChart3 className="w-6 h-6 text-celestial-gold" />
            SAV (사르바아슈타카바르가) — 하우스별 에너지 점수
          </h5>
          <p className="text-xs text-white/40 mb-6">각 하우스의 빈두 포인트 합산입니다. 28점 이상이면 강력한 하우스, 25점 미만이면 약한 하우스로 볼 수 있습니다.</p>
          <SavScoreChart points={sav.points} />
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
          <VimshopakaTable vimshopaka={vimshopaka} />
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

      {/* ── 고차라 트랜싯 (Gochara) ─────────────────── */}
      <GocharaSection summary={(report as any).gochara ?? null} />

      {/* ── 카라카 + 아바스타 (Karakas + Avasthas) ────────── */}
      <AvasthaKarakaSection
        avasthas={report.chart.avasthas ?? []}
        karakas={report.chart.karakas ?? []}
      />
    </motion.div>
  );
}
