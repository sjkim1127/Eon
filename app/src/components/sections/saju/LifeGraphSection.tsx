import { useState } from "react";
import {
    ResponsiveContainer,
    ComposedChart,
    Area,
    Line,
    CartesianGrid,
    XAxis,
    YAxis,
    Tooltip,
    ReferenceArea,
    ReferenceLine,
} from "recharts";
import { TrendingUp, Star, AlertTriangle, Moon } from "lucide-react";
import type { YearlyScore, GoldenTime } from "../../../types/saju";
import type { VedicAnalysisResult } from "../../../types/vedic";

interface LifeGraphSectionProps {
    timeline: YearlyScore[];
    goldenTime?: GoldenTime | null;
    simulationFrames?: any[];
    vedicReport?: VedicAnalysisResult | null;
}

export function LifeGraphSection({ timeline, goldenTime, simulationFrames, vedicReport }: LifeGraphSectionProps) {
    const [visibleLines, setVisibleLines] = useState({
        trend_ma: true,
        wealth: true,
        career: true,
        academic: true,
        health: true,
        volatility: true,
        vedic_dasha: true,
    });

    const toggleLine = (dataKey: string) => {
        setVisibleLines(prev => ({ ...prev, [dataKey as keyof typeof prev]: !prev[dataKey as keyof typeof prev] }));
    };

    const hasTimeline = timeline && timeline.length > 0;
    const hasFallback = !hasTimeline && simulationFrames && simulationFrames.length > 0;
    if (!hasTimeline && !hasFallback) return null;

    const chartData = hasTimeline
        ? timeline.map((f) => ({
            age: f.age,
            score: Number(f.total_score ?? 0),
            trend_ma: f.trend_ma_5yr != null ? Number(f.trend_ma_5yr) : null,
            wealth: Number(f.wealth_score ?? 0),
            career: Number(f.career_score ?? 0),
            academic: Number(f.academic_score ?? 0),
            health: Number(f.health_score ?? 0),
            volatility: Number(f.volatility_index ?? 0),
            isTransition: f.is_transition_period,
        }))
        : (simulationFrames ?? []).map((f: any) => ({
            age: f.age,
            score: Number(f.score ?? 0),
            trend_ma: Number(f.score ?? 0),
            wealth: 0, career: 0, academic: 0, health: 0, volatility: 0,
            isTransition: false,
        }));

    // Find transition periods for vertical markers
    const transitionAges = chartData.filter(d => d.isTransition).map(d => d.age);

    const getStatusColor = (score: number) =>
        score >= 70 ? "text-green-400" : score >= 40 ? "text-amber-400" : "text-red-400";

    const LEGEND_ITEMS = [
        { key: "trend_ma", label: "종합(MA)", color: "bg-cyan-400" },
        { key: "wealth", label: "재물운", color: "bg-amber-400" },
        { key: "career", label: "명예운", color: "bg-purple-400" },
        { key: "academic", label: "학업운", color: "bg-blue-400" },
        { key: "health", label: "건강운", color: "bg-emerald-400" },
        { key: "volatility", label: "교운기 변동", color: "bg-rose-400" },
    ];

    const VEDIC_PLANET_COLORS: Record<string, string> = {
        Sun: "#f97316", // orange
        Moon: "#94a3b8", // slate
        Mars: "#ef4444", // red
        Mercury: "#10b981", // emerald
        Jupiter: "#eab308", // yellow/gold
        Venus: "#ec4899", // pink
        Saturn: "#3b82f6", // blue
        Rahu: "#8b5cf6", // violet
        Ketu: "#6b7280", // grey
    };

    // Calculate Dasha periods in age-space
    const dashaPeriods = (vedicReport?.report as any)?.dasha_timeline ?? (vedicReport?.report as any)?.dashaTimeline ? (() => {
        const timeline = (vedicReport?.report as any)?.dasha_timeline ?? (vedicReport?.report as any)?.dashaTimeline ?? [];
        if (timeline.length === 0) return [];

        const birthTime = new Date(timeline[0].start_time).getTime();
        return timeline.map((p: any) => {
            const startAge = (new Date(p.start_time).getTime() - birthTime) / (365.2425 * 86400000);
            const endAge = (new Date(p.end_time).getTime() - birthTime) / (365.2425 * 86400000);
            return {
                lord: p.lord,
                start: Math.max(0, startAge),
                end: Math.min(100, endAge),
                color: VEDIC_PLANET_COLORS[p.lord] || "#888",
            };
        }).filter((p: any) => p.start < 100 && p.end > 0);
    })() : [];

    return (
        <div className="space-y-6">
            {/* 골든타임 요약 카드 */}
            {goldenTime && (
                <div className="glass p-6 rounded-[2rem] border-celestial-gold/20 bg-gradient-to-r from-celestial-gold/10 to-transparent relative overflow-hidden">
                    <div className="absolute top-0 right-0 p-4 opacity-10">
                        <Star className="w-32 h-32" />
                    </div>
                    <div className="relative z-10 flex flex-col sm:flex-row sm:items-center gap-4">
                        <div className="flex items-center gap-3">
                            <Star className="w-8 h-8 text-celestial-gold flex-shrink-0" />
                            <div>
                                <p className="text-xs text-celestial-gold/70 font-bold uppercase tracking-wider mb-1">Golden Time 🏆</p>
                                <p className="text-3xl font-black text-celestial-gold">
                                    {goldenTime.start_age}~{goldenTime.end_age}세
                                </p>
                            </div>
                        </div>
                        <div className="sm:ml-auto text-right">
                            <p className="text-xs text-white/40 mb-1">평균 점수</p>
                            <p className="text-2xl font-bold text-white">{goldenTime.average_score?.toFixed(1)}</p>
                        </div>
                    </div>
                    {goldenTime.description && (
                        <p className="text-sm text-white/60 mt-3 relative z-10">{goldenTime.description}</p>
                    )}
                </div>
            )}

            {/* 인생 곡선 그래프 */}
            <div className="glass p-8 rounded-[2rem]">
                <h5 className="text-xl font-bold text-white mb-6 flex items-center justify-between">
                    <div className="flex items-center gap-3">
                        <TrendingUp className="w-6 h-6 text-celestial-cyan" />
                        인생 흐름 그래프 (0~100세 {hasTimeline ? "다차원 점수" : "종합 점수"})
                    </div>
                </h5>

                <div className="h-72 w-full">
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
                                <linearGradient id="goldenTimeFill" x1="0" y1="0" x2="0" y2="1">
                                    <stop offset="0%" stopColor="#f59e0b" stopOpacity={0.25} />
                                    <stop offset="100%" stopColor="#f59e0b" stopOpacity={0.05} />
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
                                        return (
                                            <div className="bg-slate-900 border border-white/20 p-4 rounded-xl shadow-xl min-w-[200px]">
                                                <p className="font-bold text-white mb-3 pb-2 border-b border-white/10 flex justify-between">
                                                    <span>{label}세</span>
                                                    {data.volatility >= 50 && (
                                                        <span className="text-rose-400 text-xs px-2 py-0.5 rounded-full bg-rose-500/20 flex items-center gap-1">
                                                            <AlertTriangle className="w-3 h-3" />교운기
                                                        </span>
                                                    )}
                                                </p>
                                                <div className="space-y-2 text-xs">
                                                    <p className="flex justify-between items-center text-white/50">
                                                        <span>종합 (MA):</span>
                                                        <span className={`font-bold ${data.trend_ma != null ? getStatusColor(data.trend_ma) : getStatusColor(data.score)}`}>
                                                            {(data.trend_ma ?? data.score).toFixed(1)}점
                                                        </span>
                                                    </p>
                                                    {hasTimeline && (
                                                        <>
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
                                                        </>
                                                    )}
                                                    {visibleLines.vedic_dasha && dashaPeriods.length > 0 && (() => {
                                                        const currentDasha = dashaPeriods.find((p: any) => label >= p.start && label < p.end);
                                                        return currentDasha ? (
                                                            <p className="flex justify-between items-center mt-2 pt-2 border-t border-white/5">
                                                                <span className="text-white/40">Vedic Dasha:</span>
                                                                <span style={{ color: currentDasha.color }} className="font-bold">{currentDasha.lord}운</span>
                                                            </p>
                                                        ) : null;
                                                    })()}
                                                </div>
                                            </div>
                                        );
                                    }
                                    return null;
                                }}
                            />

                            {/* Golden Time 하이라이트 밴드 */}
                            {goldenTime && (
                                <ReferenceArea
                                    x1={goldenTime.start_age}
                                    x2={goldenTime.end_age}
                                    fill="url(#goldenTimeFill)"
                                    stroke="#f59e0b"
                                    strokeOpacity={0.3}
                                    strokeDasharray="4 4"
                                />
                            )}

                            {/* 교운기(전환기) 세로 점선 마커 */}
                            {transitionAges.map(age => (
                                <ReferenceLine
                                    key={`trans-${age}`}
                                    x={age}
                                    stroke="rgba(244,63,94,0.3)"
                                    strokeDasharray="2 4"
                                />
                            ))}

                            {/* 베딕 다샤(Mahadasha) 배경 밴드 */}
                            {visibleLines.vedic_dasha && dashaPeriods.map((p: any, i: number) => (
                                <ReferenceArea
                                    key={`dasha-${i}`}
                                    x1={p.start}
                                    x2={p.end}
                                    fill={p.color}
                                    fillOpacity={0.06}
                                    stroke="none"
                                />
                            ))}

                            {/* 메인 Area: 5년 이동평균 */}
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

                {/* 범례 토글 */}
                <div className="flex gap-4 mt-4 text-xs text-white/40 flex-wrap">
                    {LEGEND_ITEMS.map(item => (
                        <button
                            key={item.key}
                            type="button"
                            onClick={() => toggleLine(item.key)}
                            className={`flex items-center gap-1 transition-opacity ${visibleLines[item.key as keyof typeof visibleLines] ? "opacity-100 hover:opacity-80 text-white" : "opacity-30 hover:opacity-60"}`}
                        >
                            <span className={`w-2 h-2 rounded-full ${item.color} inline-block`} />
                            {item.label}
                        </button>
                    ))}
                    {vedicReport && (
                        <button
                            type="button"
                            onClick={() => toggleLine("vedic_dasha")}
                            className={`flex items-center gap-1 transition-opacity ${visibleLines.vedic_dasha ? "opacity-100 hover:opacity-80 text-celestial-gold" : "opacity-30 hover:opacity-60"}`}
                        >
                            <Moon className="w-3 h-3" />
                            베딕 다샤 오버레이
                        </button>
                    )}
                </div>
            </div>
        </div>
    );
}
