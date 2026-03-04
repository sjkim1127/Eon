import {
    ResponsiveContainer,
    RadarChart,
    PolarGrid,
    PolarAngleAxis,
    PolarRadiusAxis,
    Radar,
    Tooltip,
} from "recharts";
import { Home } from "lucide-react";
import type { BhavaStrength } from "../../types/vedic";

const HOUSE_LABELS: Record<number, string> = {
    1: "1H 자아", 2: "2H 재물", 3: "3H 형제", 4: "4H 가정",
    5: "5H 자녀", 6: "6H 적", 7: "7H 배우자", 8: "8H 변형",
    9: "9H 행운", 10: "10H 직업", 11: "11H 이익", 12: "12H 해탈",
};

interface BhavaRadarSectionProps {
    strengths: BhavaStrength[];
}

export function BhavaRadarSection({ strengths }: BhavaRadarSectionProps) {
    if (!strengths || strengths.length === 0) return null;

    const chartData = strengths.map(s => ({
        house: HOUSE_LABELS[s.house] ?? `H${s.house}`,
        total: Math.round(s.total_score),
        lord: Math.round(s.lord_score),
        dig: Math.round(s.dig_score),
        drishti: Math.round(s.drishti_score),
    }));

    const getRatingColor = (score: number) =>
        score > 400 ? "text-celestial-gold" : score > 300 ? "text-green-400" : score > 200 ? "text-white/70" : "text-red-400";

    const getRatingLabel = (score: number) =>
        score > 400 ? "탁월" : score > 300 ? "강함" : score > 200 ? "보통" : "약함";

    return (
        <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                <Home className="w-6 h-6 text-celestial-purple" />
                12하우스 강도 분석 (Bhava Strength)
            </h5>

            <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
                {/* 레이더 차트 */}
                <div className="h-72 bg-white/5 rounded-2xl border border-white/10 p-3">
                    <ResponsiveContainer width="100%" height="100%">
                        <RadarChart data={chartData}>
                            <PolarGrid stroke="rgba(255,255,255,0.15)" />
                            <PolarAngleAxis
                                dataKey="house"
                                tick={{ fill: "rgba(255,255,255,0.65)", fontSize: 10 }}
                            />
                            <PolarRadiusAxis
                                angle={30}
                                tick={{ fill: "rgba(255,255,255,0.35)", fontSize: 9 }}
                                stroke="rgba(255,255,255,0.15)"
                            />
                            <Tooltip
                                contentStyle={{
                                    backgroundColor: "rgba(15,23,42,0.95)",
                                    border: "1px solid rgba(255,255,255,0.15)",
                                    borderRadius: "12px",
                                    padding: "8px 12px",
                                    fontSize: "12px",
                                }}
                            />
                            <Radar dataKey="total" stroke="#a78bfa" fill="#a78bfa" fillOpacity={0.35} />
                        </RadarChart>
                    </ResponsiveContainer>
                </div>

                {/* 하우스 강도 리스트 */}
                <div className="grid grid-cols-2 sm:grid-cols-3 gap-2">
                    {strengths.map(s => (
                        <div
                            key={s.house}
                            className="p-3 rounded-xl bg-white/5 border border-white/10 hover:bg-white/[0.07] transition-all"
                        >
                            <div className="flex items-center justify-between mb-1.5">
                                <span className="text-xs text-white/40 font-bold">{HOUSE_LABELS[s.house] ?? `H${s.house}`}</span>
                                <span className={`text-xs font-bold ${getRatingColor(s.total_score)}`}>
                                    {getRatingLabel(s.total_score)}
                                </span>
                            </div>
                            <p className="text-lg font-black text-white mb-1">{Math.round(s.total_score)}</p>
                            <div className="flex gap-2 text-[10px] text-white/40">
                                <span>lord:{Math.round(s.lord_score)}</span>
                                <span>dig:{Math.round(s.dig_score)}</span>
                                <span>dṛṣṭi:{Math.round(s.drishti_score)}</span>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
}
