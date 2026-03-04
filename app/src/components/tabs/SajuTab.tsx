import { useState } from "react";
import { motion } from "framer-motion";
import { Activity, Zap, Shield, Star, TrendingUp, AlertTriangle, Link2, CircleOff } from "lucide-react";
import {
  ResponsiveContainer,
  ComposedChart,
  Area,
  Line,
  CartesianGrid,
  XAxis,
  YAxis,
  Tooltip,
} from "recharts";
import {
  STEM_INFO, BRANCH_INFO, ELEMENT_INFO,
  STEM_TO_ELEMENT, BRANCH_TO_ELEMENT,
  STRENGTH_INFO, TENGOD_INFO, STRUCTURE_INFO,
  SPIRIT_INFO, PILLAR_POS_INFO, YONGSHIN_TYPE_INFO,
  REL_INFO,
} from "../../constants";
import { ganziDisplay, ganziHangul } from "../../utils";

interface SajuTabProps {
  sajuReport: any;
  unknownTime?: boolean;
}

export function SajuTab({ sajuReport, unknownTime = false }: SajuTabProps) {
  const [visibleLines, setVisibleLines] = useState({
    trend_ma: true,
    wealth: true,
    career: true,
    academic: true,
    health: true,
    volatility: true,
  });

  const toggleLine = (dataKey: string) => {
    setVisibleLines(prev => ({ ...prev, [dataKey as keyof typeof prev]: !prev[dataKey as keyof typeof prev] }));
  };

  if (!sajuReport || !sajuReport.report) return null;
  const reportData = sajuReport.report;
  const p = reportData.pillars;
  const t = reportData.ten_gods;
  const s = reportData.strength;
  const y = reportData.yongshin;
  const st = reportData.structure;
  const sp = reportData.spirit_markers;
  const ml = reportData.major_luck;
  const gt = reportData.golden_time;

  return (
    <motion.div
      key="saju-results"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* 시간 미상 경고 배너 */}
      {unknownTime && (
        <div className="flex items-center gap-3 px-5 py-3 rounded-2xl bg-amber-500/10 border border-amber-500/25 text-amber-300 text-sm">
          <AlertTriangle className="w-4 h-4 shrink-0 text-amber-400" />
          <span>
            <strong>시주(時柱) 미상</strong> — 정오(12:00)기준 분석. 시주 및 시 기반 신살·용신은 참고용입니다.
          </span>
        </div>
      )}

      {/* 사주팔자 차트 */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
          <Activity className="w-6 h-6 text-celestial-gold" />
          사주팔자 (四柱八字)
        </h5>
        <div className={`grid gap-2 md:gap-4 ${unknownTime ? "grid-cols-3" : "grid-cols-4"}`}>
          {[
            { label: "시주", pillar: p?.hour, isHour: true, tStem: t?.hour_stem, tBranch: t?.hour_branch },
            { label: "일주", pillar: p?.day, isHour: false, tStem: t?.day_stem, tBranch: t?.day_branch },
            { label: "월주", pillar: p?.month, isHour: false, tStem: t?.month_stem, tBranch: t?.month_branch },
            { label: "년주", pillar: p?.year, isHour: false, tStem: t?.year_stem, tBranch: t?.year_branch },
          ]
            .filter(({ isHour }) => !(isHour && unknownTime))
            .map(({ label, pillar, tStem, tBranch }) => {
              const stemEleKey = STEM_TO_ELEMENT[pillar?.stem] ?? "";
              const branchEleKey = BRANCH_TO_ELEMENT[pillar?.branch] ?? "";

              return (
                <div key={label} className="flex flex-col text-center p-3 rounded-2xl border transition-all bg-white/5 border-white/10">
                  <p className="text-[11px] text-white/40 font-bold uppercase tracking-wider mb-2 flex items-center justify-center gap-1.5">
                    {label}
                  </p>

                  {/* 천간 영역 */}
                  <div className="flex-1 flex flex-col justify-center py-2 border-b border-white/5 relative group">
                    <p className="text-[11px] text-white/50 mb-1.5 font-semibold">
                      {TENGOD_INFO[tStem]?.hangul || tStem || "—"}
                    </p>
                    <p className="text-2xl sm:text-3xl font-black text-celestial-gold mb-1.5 leading-none">
                      {STEM_INFO[pillar?.stem]?.hanja || "—"}
                    </p>
                    <p className="text-xs text-white/80 flex items-center justify-center gap-1">
                      {STEM_INFO[pillar?.stem]?.hangul || ""}
                      <span className="text-[10px] text-white/40 border border-white/10 px-1 py-0.5 rounded-sm">
                        {ELEMENT_INFO[stemEleKey]?.hangul || ""}
                      </span>
                    </p>
                  </div>

                  {/* 지지 영역 */}
                  <div className="flex-1 flex flex-col justify-center py-2 relative group mt-1">
                    <p className="text-2xl sm:text-3xl font-black text-celestial-cyan mb-1.5 leading-none">
                      {BRANCH_INFO[pillar?.branch]?.hanja || "—"}
                    </p>
                    <p className="text-xs text-white/80 flex items-center justify-center gap-1 mb-1.5">
                      {BRANCH_INFO[pillar?.branch]?.hangul || ""}
                      <span className="text-[10px] text-white/40 border border-white/10 px-1 py-0.5 rounded-sm">
                        {ELEMENT_INFO[branchEleKey]?.hangul || ""}
                      </span>
                    </p>
                    <p className="text-[11px] text-white/50 font-semibold">
                      {TENGOD_INFO[tBranch]?.hangul || tBranch || "—"}
                    </p>
                  </div>
                </div>
              );
            })}
        </div>
      </div>

      {/* 역량 + 용신 + 격국 */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        {/* 역량 분석 */}
        <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
          <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
            <Zap className="w-32 h-32" />
          </div>
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            신강/신약 분석
          </p>
          <h4 className="text-3xl font-bold text-white mb-2">
            {STRENGTH_INFO[s?.strength_type] || s?.strength_type || "—"}
          </h4>
          <div className="flex items-baseline gap-2 mb-4">
            <span className="text-5xl font-black text-gradient leading-none">
              {s?.strength_score != null ? Math.round(s.strength_score) : "—"}
            </span>
            <span className="text-white/20 font-bold">점</span>
          </div>
          <div className="space-y-1 text-xs text-white/50">
            <p>득령: {s?.deuk_ryeong?.acquired ? "✅" : "❌"}</p>
            <p>득지: {s?.deuk_ji?.acquired ? "✅" : "❌"}</p>
            <p>득시: {s?.deuk_si?.acquired ? "✅" : "❌"}</p>
            <p>득세: {s?.deuk_se?.acquired ? "✅" : "❌"}</p>
          </div>
        </div>

        {/* 용신 */}
        <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
          <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
            용신 (用神)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {ELEMENT_INFO[y?.primary]?.hangul || y?.primary || "—"}
            <span className="text-lg text-white/40 ml-2">{ELEMENT_INFO[y?.primary]?.hanja}</span>
          </h4>
          {/* 용신 상세 목록 (조후/억부/통관/병약) */}
          {y?.recommendations && y.recommendations.length > 0 && (
            <div className="space-y-2 mb-3">
              {y.recommendations.map((rec: any, i: number) => (
                <div key={i} className="flex items-start gap-2 text-xs">
                  <span className="shrink-0 px-1.5 py-0.5 rounded bg-celestial-purple/30 text-celestial-purple/90 font-bold">
                    {YONGSHIN_TYPE_INFO[rec.yongshin_type] || rec.yongshin_type}
                  </span>
                  <span className="text-white/70 font-semibold">
                    {ELEMENT_INFO[rec.element]?.hangul || rec.element}
                  </span>
                </div>
              ))}
            </div>
          )}
          <p className="text-xs text-white/40 leading-relaxed">
            희신(喜神): {ELEMENT_INFO[y?.assistant]?.hangul || y?.assistant || "—"}
          </p>
        </div>

        {/* 격국 */}
        <div className="glass p-8 rounded-[2rem]">
          <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
            격국 (格局)
          </p>
          <h4 className="text-3xl font-bold text-white mb-4">
            {STRUCTURE_INFO[st?.structure]?.hangul || st?.structure || "—"}
            <span className="text-sm text-white/40 ml-2">{STRUCTURE_INFO[st?.structure]?.hanja}</span>
          </h4>
          <p className="text-sm text-white/60 leading-relaxed">
            {st?.reason || ""}
          </p>
        </div>
      </div>

      {/* 오행과 십성 분석 (Power) */}
      {reportData.power && (() => {
        const { options, element_scores, ten_god_scores } = reportData.power;

        const getStatusColor = (percent: number) => {
          if (percent < 10) return "text-white/40"; // 부족
          if (percent <= 20) return "text-emerald-400"; // 적정
          if (percent <= 35) return "text-amber-400"; // 발달
          return "text-red-400"; // 과다
        };

        const getStatusText = (percent: number) => {
          if (percent < 10) return "부족";
          if (percent <= 20) return "적정";
          if (percent <= 35) return "발달";
          return "과다";
        };

        return (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
              <Star className="w-6 h-6 text-celestial-gold" />
              오행과 십성 분석
            </h5>

            <div className="flex flex-col gap-1 mb-6 text-xs text-white/50 bg-white/5 p-4 rounded-xl border border-white/10">
              <p className="flex items-center gap-2">
                <span className="w-4 h-4 rounded-full bg-celestial-gold/20 flex items-center justify-center text-[10px] text-celestial-gold">✓</span>
                합에 따른 오행 변화 적용 {options.apply_transform ? "(적용됨)" : "(미적용)"}
              </p>
              <p className="flex items-center gap-2">
                <span className="w-4 h-4 rounded-full bg-celestial-gold/20 flex items-center justify-center text-[10px] text-celestial-gold">✓</span>
                조후와 궁성 보정값 적용 {options.apply_correction ? "(적용됨)" : "(미적용)"}
              </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
              {/* 오행 */}
              <div>
                <h6 className="text-sm font-bold text-brand-400 mb-4 uppercase tracking-wider">오행 분포</h6>
                <div className="space-y-3">
                  {element_scores.map(([elKey, percent]: [string, number, number], idx: number) => {
                    const info = ELEMENT_INFO[elKey];
                    if (!info) return null;
                    return (
                      <div key={idx} className="flex items-center justify-between p-3 rounded-xl bg-white/5 border border-white/10 hover:bg-white/10 transition-colors">
                        <div className="flex items-center gap-3">
                          <span className="text-lg font-black text-white">{info.hangul}({info.hanja})</span>
                        </div>
                        <div className="flex items-center gap-4 text-sm font-bold">
                          <span className="text-white/80 w-12 text-right">{percent > 0 ? `${percent.toFixed(1)}%` : "-"}</span>
                          <span className={`w-10 text-right ${percent > 0 ? getStatusColor(percent) : "text-white/20"}`}>
                            {percent > 0 ? getStatusText(percent) : "부재"}
                          </span>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>

              {/* 십성 */}
              <div>
                <h6 className="text-sm font-bold text-celestial-purple/80 mb-4 uppercase tracking-wider">십성 분포</h6>
                <div className="space-y-2">
                  {ten_god_scores.map(([godKey, percent]: [string, number, number], idx: number) => {
                    const info = TENGOD_INFO[godKey];
                    if (!info) return null;
                    const isPresent = percent > 0;
                    return (
                      <div key={idx} className={`flex items-center justify-between p-2 rounded-lg border ${isPresent ? 'bg-white/5 border-white/10' : 'bg-transparent border-transparent'} transition-colors`}>
                        <div className="flex items-center gap-3">
                          <span className={`${isPresent ? 'text-white' : 'text-white/30'} font-semibold text-sm`}>{info.hangul}({info.hanja})</span>
                        </div>
                        <div className="flex items-center gap-4 text-xs font-bold">
                          <span className={`${isPresent ? 'text-celestial-gold' : 'text-white/20'} w-12 text-right`}>
                            {isPresent ? `${percent.toFixed(1)}%` : "-"}
                          </span>
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            </div>
          </div>
        );
      })()}

      {/* 신살 */}
      {sp?.markers && sp.markers.length > 0 && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Shield className="w-6 h-6 text-celestial-cyan" />
            신살 (神煞) 분석
          </h5>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
            {sp.markers.map((m: any, i: number) => (
              <div
                key={i}
                className="p-4 bg-white/5 rounded-xl border border-white/10 text-center"
              >
                <p className="text-sm font-bold text-celestial-gold">{SPIRIT_INFO[m.marker]?.hangul || m.marker || "—"}</p>
                <p className="text-xs text-white/40 mt-1">{PILLAR_POS_INFO[m.position] || m.position || ""}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* 합충형해 (合沖刑害) 분석 */}
      {sajuReport.relationships && (() => {
        const rel = sajuReport.relationships;
        type RelGroup = { label: string; color: string; items: string[] };

        const formatRel = (r: any) => {
          if (typeof r === "string") return REL_INFO[r] || r;
          if (r && typeof r === "object") {
            if (r.SelfPunishment) return `${BRANCH_INFO[r.SelfPunishment]?.hangul || r.SelfPunishment}자형`;
            if (r.hangul) return r.hangul;
          }
          return JSON.stringify(r);
        };

        const groups: RelGroup[] = [
          { label: "천간합", color: "text-emerald-400 bg-emerald-500/15 border-emerald-500/30", items: (rel.stem_combinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
          { label: "천간충", color: "text-red-400 bg-red-500/15 border-red-500/30", items: (rel.stem_clashes ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
          { label: "삼합", color: "text-amber-400 bg-amber-500/15 border-amber-500/30", items: (rel.triple_combinations ?? []).map((r: any) => formatRel(r)) },
          { label: "방합", color: "text-amber-400 bg-amber-500/15 border-amber-500/30", items: (rel.seasonal_combinations ?? []).map((r: any) => formatRel(r)) },
          { label: "반합(진)", color: "text-yellow-400 bg-yellow-500/15 border-yellow-500/30", items: (rel.dominant_semi_combinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
          { label: "육합", color: "text-green-400 bg-green-500/15 border-green-500/30", items: (rel.six_combinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
          { label: "지지충", color: "text-rose-400 bg-rose-500/15 border-rose-500/30", items: (rel.branch_clashes ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
          { label: "지지형", color: "text-orange-400 bg-orange-500/15 border-orange-500/30", items: (rel.branch_punishments ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
          { label: "지지해", color: "text-pink-400 bg-pink-500/15 border-pink-500/30", items: (rel.branch_harms ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
          { label: "지지파", color: "text-fuchsia-400 bg-fuchsia-500/15 border-fuchsia-500/30", items: (rel.branch_destructions ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
          { label: "암합", color: "text-teal-400 bg-teal-500/15 border-teal-500/30", items: (rel.am_combinations ?? []).map((r: any) => `${formatRel(r[0]?.combination)} (${r[1]}-${r[2]})`) },
          { label: "명암합", color: "text-cyan-400 bg-cyan-500/15 border-cyan-500/30", items: (rel.myung_am_combinations ?? []).map((r: any) => `${formatRel(r[0]?.combination)} (${r[1]}-${r[2]})`) },
        ].filter(g => g.items.length > 0);

        if (groups.length === 0) return null;
        return (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <Link2 className="w-6 h-6 text-emerald-400" />
              합충형해 (合沖刑害) 분석
            </h5>
            <div className="space-y-4">
              {groups.map((g) => (
                <div key={g.label}>
                  <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">{g.label}</p>
                  <div className="flex flex-wrap gap-2">
                    {g.items.map((item: string, j: number) => (
                      <span key={j} className={`text-xs px-3 py-1.5 rounded-full border font-semibold ${g.color}`}>
                        {item}
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          </div>
        );
      })()}

      {/* 공망 (空亡) 분석 */}
      {sajuReport.voids && (() => {
        const va = sajuReport.voids;
        const branches: any[] = va.void_branches ?? [];
        const positions: string[] = va.void_positions ?? [];
        const tenGods: any[] = va.void_ten_gods ?? [];
        const xunGroup: string = va.xun_group ?? "";

        return (
          <div className="glass p-8 rounded-[2rem] border-violet-500/20 bg-violet-500/5">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <CircleOff className="w-6 h-6 text-violet-400" />
              공망 (空亡) 분석
            </h5>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
              <div>
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">공망 지지</p>
                <div className="flex gap-3">
                  {branches.map((b: string, i: number) => {
                    const info = BRANCH_INFO[b];
                    return (
                      <div key={i} className="px-5 py-3 rounded-xl bg-violet-500/15 border border-violet-500/30 text-center">
                        <p className="text-2xl font-black text-violet-300">{info?.hanja ?? b}</p>
                        <p className="text-xs text-violet-400/70 mt-1">{info?.hangul ?? ""}</p>
                      </div>
                    );
                  })}
                </div>
                {xunGroup && (
                  <p className="text-xs text-white/40 mt-3">순(旬) 그룹: <span className="text-violet-300 font-semibold">{xunGroup}</span></p>
                )}
              </div>
              <div>
                {positions.length > 0 ? (
                  <>
                    <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">원국 내 공망 발생</p>
                    <div className="space-y-2">
                      {positions.map((pos: string, i: number) => (
                        <div key={i} className="flex items-center gap-3 text-sm">
                          <span className="px-2.5 py-1 rounded-lg bg-violet-500/20 border border-violet-500/30 text-violet-300 font-bold">{pos}</span>
                          <span className="text-white/60">→</span>
                          <span className="text-white font-semibold">{TENGOD_INFO[tenGods[i]]?.hangul ?? tenGods[i] ?? ""}</span>
                          <span className="text-white/30 text-xs">공망</span>
                        </div>
                      ))}
                    </div>
                  </>
                ) : (
                  <div className="flex items-center h-full">
                    <p className="text-sm text-white/40">원국 내에 공망이 없습니다.</p>
                  </div>
                )}
              </div>
            </div>
          </div>
        );
      })()}

      {/* 대운 */}
      {ml && ml.cycles && (
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <TrendingUp className="w-6 h-6 text-celestial-purple" />
            대운 (大運) 흐름
          </h5>
          <p className="text-sm text-white/50 mb-4">
            방향: {ml.direction === "Forward" ? "순행 ▶" : "역행 ◀"} | 시작 나이: {ml.start_age}세
          </p>
          <div className="grid grid-cols-2 sm:grid-cols-5 lg:grid-cols-10 gap-3">
            {ml.cycles.map((c: any, i: number) => (
              <div
                key={i}
                className="p-3 bg-white/5 rounded-xl border border-white/10 text-center hover:bg-white/10 transition-all"
              >
                <p className="text-xs text-white/40 mb-1">{c.start_age}~{c.end_age}세</p>
                <p className="text-lg font-bold text-white">{ganziDisplay(c.ganzi)}</p>
                <p className="text-xs text-white/30">{ganziHangul(c.ganzi)}</p>
                <p className="text-[10px] text-celestial-gold mt-1">{TENGOD_INFO[c.stem_god]?.hangul || c.stem_god || ""}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* 골든타임 */}
      {gt && (
        <div className="glass p-8 rounded-[2rem] border-celestial-gold/20 bg-celestial-gold/5">
          <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
            <Star className="w-6 h-6 text-celestial-gold" />
            골든타임 🏆
          </h5>
          <div className="flex items-baseline gap-4 mb-4">
            <span className="text-5xl font-black text-celestial-gold">
              {gt.start_age}~{gt.end_age}세
            </span>
            <span className="text-white/40">평균 점수: {gt.average_score?.toFixed(1)}</span>
          </div>
          <p className="text-sm text-white/60">{gt.description}</p>
        </div>
      )}

      {(() => {
        // Debug: timeline / simulation_frames 데이터 확인
        console.log("[SajuTab] reportData keys:", Object.keys(reportData));
        console.log("[SajuTab] timeline:", reportData.timeline?.length ?? "없음");
        console.log("[SajuTab] simulation_frames:", reportData.simulation_frames?.length ?? "없음");
        return null;
      })()}

      {/* 다차원 인생 흐름 그래프 — timeline 우선, fallback으로 simulation_frames */}
      {((reportData.timeline && reportData.timeline.length > 0) || (reportData.simulation_frames && reportData.simulation_frames.length > 0)) && (() => {
        const hasTimeline = reportData.timeline && reportData.timeline.length > 0;
        const chartData = hasTimeline
          ? reportData.timeline.map((f: any) => ({
            age: f.age,
            score: Number(f.total_score ?? 0),
            trend_ma: f.trend_ma_5yr != null ? Number(f.trend_ma_5yr) : null,
            wealth: Number(f.wealth_score ?? 0),
            career: Number(f.career_score ?? 0),
            academic: Number(f.academic_score ?? 0),
            health: Number(f.health_score ?? 0),
            volatility: Number(f.volatility_index ?? 0),
          }))
          : reportData.simulation_frames.map((f: any) => ({
            age: f.age,
            score: Number(f.score ?? 0),
            trend_ma: Number(f.score ?? 0),
            wealth: 0, career: 0, academic: 0, health: 0, volatility: 0,
          }));
        return (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center justify-between">
              <div className="flex items-center gap-3">
                <TrendingUp className="w-6 h-6 text-celestial-cyan" />
                인생 흐름 그래프 (0~100세 {hasTimeline ? "다차원 점수" : "종합 점수"})
              </div>
            </h5>
            <div className="h-64 w-full">
              <ResponsiveContainer width="100%" height="100%">
                <ComposedChart
                  data={chartData}
                  margin={{ top: 8, right: 12, left: 0, bottom: 8 }}
                >
                  <defs>
                    <linearGradient id="sajuScoreGradient" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="5%" stopColor="#06b6d4" stopOpacity={0.7} />
                      <stop offset="95%" stopColor="#06b6d4" stopOpacity={0.05} />
                    </linearGradient>
                  </defs>
                  <CartesianGrid stroke="rgba(255,255,255,0.08)" strokeDasharray="3 3" />
                  <XAxis
                    dataKey="age"
                    stroke="rgba(255,255,255,0.45)"
                    tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 11 }}
                    tickLine={false}
                    axisLine={{ stroke: "rgba(255,255,255,0.15)" }}
                    unit="세"
                  />
                  <YAxis
                    domain={[0, 100]}
                    stroke="rgba(255,255,255,0.45)"
                    tick={{ fill: "rgba(255,255,255,0.55)", fontSize: 11 }}
                    tickLine={false}
                    axisLine={{ stroke: "rgba(255,255,255,0.15)" }}
                  />
                  <Tooltip
                    content={({ active, payload, label }) => {
                      if (active && payload && payload.length) {
                        const data = payload[0].payload;
                        const getStatusColor = (score: number) => score >= 70 ? "text-green-400" : (score >= 40 ? "text-amber-400" : "text-red-400");

                        return (
                          <div className="bg-slate-900 border border-white/20 p-4 rounded-xl shadow-xl min-w-[200px]">
                            <p className="font-bold text-white mb-3 pb-2 border-b border-white/10 flex justify-between">
                              <span>{label}세</span>
                              {data.volatility >= 50 && (
                                <span className="text-rose-400 text-xs px-2 py-0.5 rounded-full bg-rose-500/20">교운기 변동</span>
                              )}
                            </p>
                            <div className="space-y-2 text-xs">
                              <p className="flex justify-between items-center text-white/50">
                                <span>총점 (MA):</span>
                                <span className={`font-bold ${data.trend_ma != null ? getStatusColor(data.trend_ma) : getStatusColor(data.score)}`}>
                                  {(data.trend_ma ?? data.score).toFixed(1)}점
                                </span>
                              </p>
                              <p className="flex justify-between items-center">
                                <span className="text-amber-400">재물/사업운:</span>
                                <span className={`font-bold ${getStatusColor(data.wealth)}`}>{data.wealth.toFixed(1)}점</span>
                              </p>
                              <p className="flex justify-between items-center">
                                <span className="text-purple-400">직장/명예운:</span>
                                <span className={`font-bold ${getStatusColor(data.career)}`}>{data.career.toFixed(1)}점</span>
                              </p>
                              <p className="flex justify-between items-center">
                                <span className="text-blue-400">학업/문서운:</span>
                                <span className={`font-bold ${getStatusColor(data.academic)}`}>{data.academic.toFixed(1)}점</span>
                              </p>
                              <p className="flex justify-between items-center">
                                <span className="text-emerald-400">건강/독립운:</span>
                                <span className={`font-bold ${getStatusColor(data.health)}`}>{data.health.toFixed(1)}점</span>
                              </p>
                            </div>
                          </div>
                        );
                      }
                      return null;
                    }}
                  />
                  {/* 배경 면적: 평활화된 이동평균 총점 */}
                  {visibleLines.trend_ma && (
                    <Area
                      type="monotone"
                      dataKey="trend_ma"
                      stroke="#06b6d4"
                      strokeWidth={2.5}
                      fill="url(#sajuScoreGradient)"
                      activeDot={{ r: 4, stroke: "#06b6d4", strokeWidth: 2, fill: "#111827" }}
                    />
                  )}
                  {visibleLines.wealth && <Line type="monotone" dataKey="wealth" stroke="#fbbf24" strokeWidth={1.5} dot={false} strokeOpacity={0.8} />}
                  {visibleLines.career && <Line type="monotone" dataKey="career" stroke="#a78bfa" strokeWidth={1.5} dot={false} strokeOpacity={0.8} />}
                  {visibleLines.academic && <Line type="monotone" dataKey="academic" stroke="#60a5fa" strokeWidth={1.5} dot={false} strokeOpacity={0.8} />}
                  {visibleLines.health && <Line type="monotone" dataKey="health" stroke="#34d399" strokeWidth={1.5} dot={false} strokeOpacity={0.8} />}
                  {visibleLines.volatility && <Line type="step" dataKey="volatility" stroke="#f43f5e" strokeWidth={1} strokeDasharray="3 3" dot={false} strokeOpacity={0.6} />}
                </ComposedChart>
              </ResponsiveContainer>
            </div>
            <div className="flex justify-between text-xs text-white/30 mt-2">
              <span>0세</span>
              <span>25세</span>
              <span>50세</span>
              <span>75세</span>
              <span>100세</span>
            </div>
            <div className="flex gap-4 mt-3 text-xs text-white/40 flex-wrap">
              <button type="button" onClick={() => toggleLine("trend_ma")} className={`flex items-center gap-1 transition-opacity ${visibleLines.trend_ma ? "opacity-100 hover:opacity-80 text-white" : "opacity-30 hover:opacity-60"}`}>
                <span className="w-2 h-2 rounded-full bg-cyan-400 inline-block" />종합(MA)
              </button>
              <button type="button" onClick={() => toggleLine("wealth")} className={`flex items-center gap-1 transition-opacity ${visibleLines.wealth ? "opacity-100 hover:opacity-80 text-white" : "opacity-30 hover:opacity-60"}`}>
                <span className="w-2 h-2 rounded-full bg-amber-400 inline-block" />재물운
              </button>
              <button type="button" onClick={() => toggleLine("career")} className={`flex items-center gap-1 transition-opacity ${visibleLines.career ? "opacity-100 hover:opacity-80 text-white" : "opacity-30 hover:opacity-60"}`}>
                <span className="w-2 h-2 rounded-full bg-purple-400 inline-block" />명예운
              </button>
              <button type="button" onClick={() => toggleLine("academic")} className={`flex items-center gap-1 transition-opacity ${visibleLines.academic ? "opacity-100 hover:opacity-80 text-white" : "opacity-30 hover:opacity-60"}`}>
                <span className="w-2 h-2 rounded-full bg-blue-400 inline-block" />학업운
              </button>
              <button type="button" onClick={() => toggleLine("health")} className={`flex items-center gap-1 transition-opacity ${visibleLines.health ? "opacity-100 hover:opacity-80 text-white" : "opacity-30 hover:opacity-60"}`}>
                <span className="w-2 h-2 rounded-full bg-emerald-400 inline-block" />건강운
              </button>
              <button type="button" onClick={() => toggleLine("volatility")} className={`flex items-center gap-1 transition-opacity ${visibleLines.volatility ? "opacity-100 hover:opacity-80 text-white" : "opacity-30 hover:opacity-60"}`}>
                <span className="w-2 h-2 rounded-full bg-rose-400 inline-block" />교운기 변동
              </button>
            </div>
          </div>
        );
      })()}
    </motion.div>
  );
}
