import { useState } from "react";
import { Clock, ChevronDown, ChevronUp } from "lucide-react";
import type { DashaPeriod } from "../../types/vedic";

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
}

export function DashaTimelineSection({ periods }: DashaTimelineSectionProps) {
    const [expandedIndex, setExpandedIndex] = useState<number | null>(null);
    const [now] = useState(() => Date.now());

    const hasPeriods = Array.isArray(periods) && periods.length > 0;

    if (!hasPeriods) {
        return (
            <div className="glass p-8 rounded-[2rem]">
                <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                    <Clock className="w-6 h-6 text-celestial-gold" />
                    다샤 타임라인 (Vimshottari Dasha)
                </h5>
                <p className="text-white/50 text-sm">
                    다샤 타임라인을 계산할 수 없습니다. 출생 차트에 달 위치가 필요하며, 베딕 분석이 완료되면 여기에 표시됩니다.
                </p>
            </div>
        );
    }

    // Total duration for proportional widths
    const totalYears = periods.reduce((sum, p) => sum + yearsBetween(p.start_time, p.end_time), 0);

    // Find current Mahadasha
    const currentIdx = periods.findIndex(p => {
        const s = new Date(p.start_time).getTime();
        const e = new Date(p.end_time).getTime();
        return now >= s && now < e;
    });

    return (
        <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                <Clock className="w-6 h-6 text-celestial-gold" />
                다샤 타임라인 (Vimshottari Dasha)
            </h5>

            {/* Mahadasha 바 차트 */}
            <div className="flex w-full h-10 rounded-xl overflow-hidden border border-white/10 mb-4">
                {periods.map((p, i) => {
                    const years = yearsBetween(p.start_time, p.end_time);
                    const pct = totalYears > 0 ? (years / totalYears) * 100 : 0;
                    const isCurrent = i === currentIdx;
                    const color = PLANET_BAR_COLORS[p.lord] ?? "#888";

                    return (
                        <button
                            key={i}
                            type="button"
                            onClick={() => setExpandedIndex(expandedIndex === i ? null : i)}
                            className={`relative flex items-center justify-center text-[10px] font-bold text-white transition-all hover:brightness-125 ${isCurrent ? "ring-2 ring-white/40 ring-inset" : ""}`}
                            style={{
                                width: `${pct}%`,
                                backgroundColor: color,
                                opacity: isCurrent ? 1 : 0.65,
                                minWidth: pct > 3 ? undefined : "18px",
                            }}
                            title={`${p.lord}: ${formatDate(p.start_time)} ~ ${formatDate(p.end_time)} (${years.toFixed(1)}년)`}
                        >
                            {pct > 5 && p.lord.substring(0, 2)}
                            {isCurrent && (
                                <span className="absolute -bottom-4 left-1/2 -translate-x-1/2 text-[8px] text-white/60">▲</span>
                            )}
                        </button>
                    );
                })}
            </div>

            {/* Mahadasha 카드 리스트 */}
            <div className="space-y-2 mt-6">
                {periods.map((p, i) => {
                    const years = yearsBetween(p.start_time, p.end_time);
                    const isCurrent = i === currentIdx;
                    const isExpanded = expandedIndex === i;
                    const colorClass = PLANET_COLORS[p.lord] ?? "bg-white/10 text-white/70 border-white/20";

                    return (
                        <div key={i}>
                            <button
                                type="button"
                                onClick={() => setExpandedIndex(isExpanded ? null : i)}
                                className={`w-full flex items-center gap-3 p-3 rounded-xl border transition-all text-left ${isCurrent ? `${colorClass} ring-1 ring-white/20` : "bg-white/[0.03] border-white/10 hover:bg-white/[0.06]"
                                    }`}
                            >
                                <span className={`text-xs px-2 py-0.5 rounded-lg border font-bold ${colorClass}`}>
                                    {p.lord}
                                </span>
                                <span className="text-xs text-white/50 font-mono flex-1">
                                    {formatDate(p.start_time)} ~ {formatDate(p.end_time)}
                                </span>
                                <span className="text-xs text-white/30">{years.toFixed(1)}년</span>
                                {p.sub_dashas.length > 0 && (
                                    isExpanded ? <ChevronUp className="w-4 h-4 text-white/30" /> : <ChevronDown className="w-4 h-4 text-white/30" />
                                )}
                                {isCurrent && (
                                    <span className="text-[10px] px-2 py-0.5 rounded-full bg-white/20 text-white font-bold">현재</span>
                                )}
                            </button>

                            {/* Antardasha 확장 */}
                            {isExpanded && p.sub_dashas.length > 0 && (
                                <div className="ml-6 mt-1 pl-4 border-l-2 border-white/10 space-y-1">
                                    {p.sub_dashas.map((sub, j) => {
                                        const subYears = yearsBetween(sub.start_time, sub.end_time);
                                        const subNow = new Date(sub.start_time).getTime();
                                        const subEnd = new Date(sub.end_time).getTime();
                                        const isSubCurrent = now >= subNow && now < subEnd;
                                        const subColor = PLANET_COLORS[sub.lord] ?? "bg-white/10 text-white/50 border-white/10";

                                        return (
                                            <div
                                                key={j}
                                                className={`flex items-center gap-2 p-2 rounded-lg text-xs ${isSubCurrent ? `${subColor} ring-1 ring-white/10` : "text-white/40"
                                                    }`}
                                            >
                                                <span className={`px-1.5 py-0.5 rounded border text-[10px] font-bold ${subColor}`}>
                                                    {sub.lord}
                                                </span>
                                                <span className="font-mono text-white/30 flex-1">
                                                    {formatDate(sub.start_time)} ~ {formatDate(sub.end_time)}
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
