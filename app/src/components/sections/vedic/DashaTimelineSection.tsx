import { useState } from "react";
import { Clock, ChevronDown, ChevronUp, Zap } from "lucide-react";
import type { DashaPeriod, SignDashaPeriod } from "../../../types/vedic";

const PLANET_COLORS: Record<string, string> = {
    Sun: "bg-orange-500/20 text-orange-300 border-orange-500/30",
    Moon: "bg-slate-300/20 text-slate-200 border-slate-300/30",
    Mars: "bg-red-500/20 text-red-300 border-red-500/30",
    Mercury: "bg-emerald-500/20 text-emerald-300 border-emerald-500/30",
    Jupiter: "bg-yellow-500/20 text-yellow-300 border-yellow-500/30",
    Venus: "bg-pink-500/20 text-pink-300 border-pink-500/30",
    Saturn: "bg-blue-500/20 text-blue-300 border-blue-500/30",
    Rahu: "bg-violet-500/20 text-violet-300 border-violet-500/30",
    Ketu: "bg-gray-500/20 text-gray-300 border-gray-500/30",
};

const PLANET_BAR_COLORS: Record<string, string> = {
    Sun: "#f97316", Moon: "#94a3b8", Mars: "#ef4444", Mercury: "#10b981",
    Jupiter: "#eab308", Venus: "#ec4899", Saturn: "#3b82f6",
    Rahu: "#8b5cf6", Ketu: "#6b7280",
};

function formatDate(iso: string): string {
    try {
        const d = new Date(iso);
        return `${d.getFullYear()}.${(d.getMonth() + 1).toString().padStart(2, "0")}`;
    } catch {
        return "—";
    }
}

function yearsBetween(startIso: string, endIso: string): number {
    try {
        const s = new Date(startIso).getTime();
        const e = new Date(endIso).getTime();
        return Math.max(0, (e - s) / (365.2425 * 24 * 60 * 60 * 1000));
    } catch {
        return 0;
    }
}

interface DashaTimelineSectionProps {
    periods: DashaPeriod[];
    yoginiPeriods?: DashaPeriod[];
    charaPeriods?: SignDashaPeriod[];
}

export function DashaTimelineSection({ periods, yoginiPeriods, charaPeriods }: DashaTimelineSectionProps) {
    const [dashaType, setDashaType] = useState<"vimshottari" | "yogini" | "chara">("vimshottari");
    const [expandedIndex, setExpandedIndex] = useState<number | null>(null);
    const [now] = useState(() => Date.now());

    const activePeriods = dashaType === "vimshottari" ? periods : (dashaType === "yogini" ? (yoginiPeriods ?? []) : (charaPeriods ?? []));
    const hasPeriods = Array.isArray(activePeriods) && activePeriods.length > 0;

    if (!hasPeriods && !yoginiPeriods?.length) {
        return (
            <div className="glass p-8 rounded-[2rem]">
                <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                    <Clock className="w-6 h-6 text-celestial-gold" />
                    다샤 타임라인
                </h5>
                <p className="text-white/50 text-sm">
                    다샤 타임라인을 계산할 수 없습니다.
                </p>
            </div>
        );
    }

    // Total duration for proportional widths
    const totalYears = activePeriods.reduce((sum, raw) => {
        const p = raw as any;
        return sum + yearsBetween(p.start_time ?? p.startTime, p.end_time ?? p.endTime);
    }, 0);

    // Find current Mahadasha
    const currentIdx = activePeriods.findIndex(raw => {
        const p = raw as any;
        const s = new Date(p.start_time ?? p.startTime).getTime();
        const e = new Date(p.end_time ?? p.endTime).getTime();
        return now >= s && now < e;
    });

    return (
        <div className="glass p-8 rounded-[2rem]">
            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 mb-6">
                <h5 className="text-xl font-bold text-white flex items-center gap-3">
                    <Clock className="w-6 h-6 text-celestial-gold" />
                    다샤 타임라인 ({dashaType === "vimshottari" ? "Vimshottari" : "Yogini"})
                </h5>
                
                <div className="flex bg-black/40 border border-white/20 rounded-xl overflow-hidden p-0.5">
                    <button
                        onClick={() => { setDashaType("vimshottari"); setExpandedIndex(null); }}
                        className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${dashaType === "vimshottari" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                    >
                        빔쇼타리
                    </button>
                    {yoginiPeriods && yoginiPeriods.length > 0 && (
                        <button
                            onClick={() => { setDashaType("yogini"); setExpandedIndex(null); }}
                            className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${dashaType === "yogini" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                        >
                            요기니
                        </button>
                    )}
                    {charaPeriods && charaPeriods.length > 0 && (
                        <button
                            onClick={() => { setDashaType("chara"); setExpandedIndex(null); }}
                            className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${dashaType === "chara" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                        >
                            제미니
                        </button>
                    )}
                </div>
            </div>

            {dashaType === "yogini" && (
                <p className="text-xs text-white/40 mb-4 bg-white/5 p-3 rounded-lg border border-white/5">
                    <Zap className="w-3 h-3 inline mr-1 text-celestial-gold" />
                    요기니 다샤(Yogini Dasha)는 36년 주기로 순환하며, 나크샤트라에 기반한 강력한 길흉 판단 도구입니다.
                </p>
            )}

            {dashaType === "chara" && (
                <p className="text-xs text-white/40 mb-4 bg-white/5 p-3 rounded-lg border border-white/5">
                    <Zap className="w-3 h-3 inline mr-1 text-celestial-purple" />
                    차라 다샤(Chara Dasha)는 사지(Signs)의 이동을 통해 영혼의 여정과 변화를 분석하는 제미니 체계의 핵심 도구입니다.
                </p>
            )}

            {/* Mahadasha 바 차트 */}
            <div className="flex w-full h-10 rounded-xl overflow-hidden border border-white/10 mb-4">
                {activePeriods.map((raw, i) => {
                    const p = raw as any;
                    const startTime = p.start_time ?? p.startTime;
                    const endTime = p.end_time ?? p.endTime;
                    const years = yearsBetween(startTime, endTime);
                    const pct = totalYears > 0 ? (years / totalYears) * 100 : 0;
                    const isCurrent = i === currentIdx;
                    
                    const lordOfRasi: Record<number, string> = {
                        1: "Mars", 2: "Venus", 3: "Mercury", 4: "Moon", 5: "Sun", 6: "Mercury",
                        7: "Venus", 8: "Mars", 9: "Jupiter", 10: "Saturn", 11: "Saturn", 12: "Jupiter"
                    };

                    let lord = "Saturn";
                    let label = "";
                    let title = "";

                    if (p.type === "planet") {
                        lord = p.lord;
                        label = p.name ? p.name.substring(0, 2) : p.lord.substring(0, 2);
                        title = `${p.name || p.lord}: ${formatDate(startTime)} ~ ${formatDate(endTime)}`;
                    } else if (p.type === "sign") {
                        lord = lordOfRasi[p.rasi] || "Saturn";
                        label = `S${p.rasi}`;
                        title = `Sign ${p.rasi}: ${formatDate(startTime)} ~ ${formatDate(endTime)}`;
                    }

                    const color = PLANET_BAR_COLORS[lord] ?? "#888";

                    return (
                        <button
                            key={`${dashaType}-${i}`}
                            type="button"
                            onClick={() => setExpandedIndex(expandedIndex === i ? null : i)}
                            className={`relative flex items-center justify-center text-[10px] font-bold text-white transition-all hover:brightness-125 ${isCurrent ? "ring-2 ring-white/40 ring-inset" : ""}`}
                            style={{
                                width: `${pct}%`,
                                backgroundColor: color,
                                opacity: isCurrent ? 1 : 0.65,
                                minWidth: pct > 2 ? undefined : "12px",
                            }}
                            title={`${title} (${years.toFixed(1)}년)`}
                        >
                            {pct > 5 && label}
                            {isCurrent && (
                                <span className="absolute -bottom-4 left-1/2 -translate-x-1/2 text-[8px] text-white/60">▲</span>
                            )}
                        </button>
                    );
                })}
            </div>

            {/* Mahadasha 카드 리스트 */}
            <div className="space-y-2 mt-6">
                {activePeriods.map((raw, i) => {
                    const p = raw as any;
                    const startTime = p.start_time ?? p.startTime;
                    const endTime = p.end_time ?? p.endTime;
                    const years = yearsBetween(startTime, endTime);
                    const isCurrent = i === currentIdx;
                    const isExpanded = expandedIndex === i;
                    
                    const lordOfRasi: Record<number, string> = {
                        1: "Mars", 2: "Venus", 3: "Mercury", 4: "Moon", 5: "Sun", 6: "Mercury",
                        7: "Venus", 8: "Mars", 9: "Jupiter", 10: "Saturn", 11: "Saturn", 12: "Jupiter"
                    };

                    let lord = "Saturn";
                    let displayLabel = "";

                    if (p.type === "planet") {
                        lord = p.lord;
                        displayLabel = p.name ? `${p.name} (${p.lord})` : p.lord;
                    } else if (p.type === "sign") {
                        lord = lordOfRasi[p.rasi] || "Saturn";
                        displayLabel = `사인 ${p.rasi} (Chara)`;
                    }

                    const colorClass = PLANET_COLORS[lord] ?? "bg-white/10 text-white/70 border-white/20";
                    const subDashas = p.sub_dashas ?? p.subDashas;

                    return (
                        <div key={`${dashaType}-list-${i}`}>
                            <button
                                type="button"
                                onClick={() => setExpandedIndex(isExpanded ? null : i)}
                                className={`w-full flex items-center gap-3 p-3 rounded-xl border transition-all text-left ${isCurrent ? `${colorClass} ring-1 ring-white/20` : "bg-white/[0.03] border-white/10 hover:bg-white/[0.06]"
                                    }`}
                            >
                                <span className={`text-xs px-2 py-0.5 rounded-lg border font-bold ${colorClass}`}>
                                    {displayLabel}
                                </span>
                                <span className="text-xs text-white/50 font-mono flex-1 text-center sm:text-left">
                                    {formatDate(startTime)} ~ {formatDate(endTime)}
                                </span>
                                <span className="text-xs text-white/30 hidden sm:inline">{years.toFixed(1)}년</span>
                                {p.type === "planet" && subDashas && subDashas.length > 0 && (
                                    isExpanded ? <ChevronUp className="w-4 h-4 text-white/30" /> : <ChevronDown className="w-4 h-4 text-white/30" />
                                )}
                                {isCurrent && (
                                    <span className="text-[10px] px-2 py-0.5 rounded-full bg-white/20 text-white font-bold">현재</span>
                                )}
                            </button>

                            {/* Antardasha 확장 */}
                            {isExpanded && p.type === "planet" && subDashas && subDashas.length > 0 && (
                                <div className="ml-6 mt-1 pl-4 border-l-2 border-white/10 space-y-1">
                                    {subDashas.map((rawSub: any, j: number) => {
                                        const sub = rawSub;
                                        const subStartTime = sub.start_time ?? sub.startTime;
                                        const subEndTime = sub.end_time ?? sub.endTime;
                                        const subYears = yearsBetween(subStartTime, subEndTime);
                                        const subNow = new Date(subStartTime).getTime();
                                        const subEnd = new Date(subEndTime).getTime();
                                        const isSubCurrent = now >= subNow && now < subEnd;
                                        const subLord = sub.lord || "Saturn";
                                        const subColor = PLANET_COLORS[subLord] ?? "bg-white/10 text-white/50 border-white/10";

                                        return (
                                            <div
                                                key={j}
                                                className={`flex items-center gap-2 p-2 rounded-lg text-xs ${isSubCurrent ? `${subColor} ring-1 ring-white/10` : "text-white/40"
                                                    }`}
                                            >
                                                <span className={`px-1.5 py-0.5 rounded border text-[10px] font-bold ${subColor}`}>
                                                    {sub.name ? `${sub.name} (${sub.lord})` : sub.lord}
                                                </span>
                                                <span className="font-mono text-white/30 flex-1">
                                                    {formatDate(subStartTime)} ~ {formatDate(subEndTime)}
                                                </span>
                                                <span className="text-white/20">{subYears.toFixed(1)}년</span>
                                                {isSubCurrent && (
                                                    <span className="text-[9px] px-1.5 py-0.5 rounded-full bg-white/20 text-white font-bold">현재</span>
                                                )}
                                            </div>
                                        );
                                    })}
                                </div>
                            )}
                        </div>
                    );
                })}
            </div>
        </div>
    );
}
