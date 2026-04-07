import { Heart, Clock } from "lucide-react";
import { cn } from "../../../utils";
import type { VedicAnalysisReport } from "../../../types/vedic";

interface VedicOverviewSectionProps {
    report: VedicAnalysisReport;
}

export function VedicOverviewSection({ report }: VedicOverviewSectionProps) {
    const primaryKarakas = report.primaryKarakas;
    const dashaFocus = report.dashaFocus ?? "";
    const overallStrengthScore = report.overallStrengthScore ?? 0;

    if (!primaryKarakas) return null;

    return (
        <div className="space-y-8">
            {/* ── Primary Karakas ────────────────────────────────────────── */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
                    <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
                        <Heart className="w-32 h-32" />
                    </div>
                    <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
                        영혼의 지표 (Atmakaraka)
                    </p>
                    <h4 className="text-3xl font-bold text-white mb-4">
                        {primaryKarakas.atmakaraka}
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
                        {primaryKarakas.amatyakaraka}
                    </h4>
                    <p className="text-sm text-white/60 leading-relaxed">
                        직업·사회적 역할에서 영혼을 보필하는 행성입니다.
                    </p>
                </div>

                <div className="glass p-8 rounded-[2rem] border-celestial-cyan/20 bg-celestial-cyan/5">
                    <p className="text-celestial-cyan/80 text-sm font-bold uppercase tracking-wider mb-2">
                        배우자 지표 (Darakaraka)
                    </p>
                    <h4 className="text-3xl font-bold text-white mb-4">
                        {primaryKarakas.darakaraka}
                    </h4>
                    <p className="text-sm text-white/60 leading-relaxed">
                        배우자·가깥은 파트너와의 관계 패턴을 나타내는 행성입니다.
                    </p>
                </div>
            </div>

            {/* ── Dasha Focus & Overall Strength ─────────────────────────── */}
            <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                <div className="glass p-10 rounded-[2.5rem] border-white/5 relative bg-gradient-to-br from-gray-900 via-gray-900 to-indigo-950/30">
                    <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                        <Clock className="w-6 h-6 text-celestial-purple" />
                        현재 시기의 핵심 과제 (Dasha Focus)
                    </h5>
                    <p className="text-lg text-white/90 leading-relaxed font-bold">
                        {dashaFocus}
                    </p>
                    <div className="mt-8 pt-8 border-t border-white/5 flex flex-wrap gap-4">
                        <div className="px-5 py-2 rounded-2xl bg-white/5 border border-white/10 text-xs text-white/40">
                            영혼의 목적 준수
                        </div>
                        <div className="px-5 py-2 rounded-2xl bg-white/5 border border-white/10 text-xs text-white/40">
                            사회적 과업 수행
                        </div>
                    </div>
                </div>

                <div className="flex flex-col justify-center gap-6">
                    <div className="flex items-end justify-between px-2">
                        <h6 className="text-sm font-bold text-white/40 uppercase tracking-widest">
                            인생 종합 에너지 지수
                        </h6>
                        <span className="text-5xl font-black text-white italic tracking-tighter">
                            {overallStrengthScore.toFixed(1)}<span className="text-sm not-italic text-white/30 ml-1">pts</span>
                        </span>
                    </div>

                    <div className="w-full bg-white/5 h-6 rounded-full overflow-hidden border border-white/10 p-1">
                        <div 
                            className={cn(
                                "h-full rounded-full transition-all duration-1000 bg-gradient-to-r",
                                overallStrengthScore >= 12 ? "from-emerald-500 to-celestial-cyan shadow-[0_0_20px_rgba(16,185,129,0.3)]" :
                                overallStrengthScore >= 9 ? "from-celestial-cyan to-celestial-purple" :
                                "from-orange-500 to-red-500 shadow-[0_0_20px_rgba(239,68,68,0.2)]"
                            )}
                            style={{ width: `${Math.min(100, (overallStrengthScore / 18) * 100)}%` }}
                        />
                    </div>

                    <div className="grid grid-cols-3 gap-2 px-2 mt-2">
                        <div className={cn("h-1 rounded-full", overallStrengthScore < 9 ? "bg-red-500/50" : "bg-white/10")} />
                        <div className={cn("h-1 rounded-full", overallStrengthScore >= 9 && overallStrengthScore < 12 ? "bg-celestial-cyan/50" : "bg-white/10")} />
                        <div className={cn("h-1 rounded-full", overallStrengthScore >= 12 ? "bg-emerald-500/50" : "bg-white/10")} />
                    </div>

                    <div className="flex justify-between items-center text-[10px] font-bold text-white/30 px-2 uppercase tracking-tight">
                        <span>Weak 0 ~ 9</span>
                        <span>Balanced 9 ~ 12</span>
                        <span>Powerful 12 ~ 18+</span>
                    </div>
                </div>
            </div>

            {/* ── Jaimini Secondary Karakas ────────────────────────────────── */}
            <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
                {[
                    { label: "형제 (Bhratrukaraka)", value: primaryKarakas.bhratrukaraka, color: "text-orange-400", bg: "bg-orange-500/5", border: "border-orange-500/10" },
                    { label: "어머니 (Matrukaraka)", value: primaryKarakas.matrukaraka, color: "text-emerald-400", bg: "bg-emerald-500/5", border: "border-emerald-500/10" },
                    { label: "아버지 (Pitrikaraka)", value: primaryKarakas.pitrikaraka, color: "text-blue-400", bg: "bg-blue-500/5", border: "border-blue-500/10" },
                    { label: "자녀 (Putrakaraka)", value: primaryKarakas.putrakaraka, color: "text-yellow-400", bg: "bg-yellow-500/5", border: "border-yellow-500/10" },
                    { label: "경쟁자 (Gnatikaraka)", value: primaryKarakas.gnatikaraka, color: "text-red-400", bg: "bg-red-500/5", border: "border-red-500/10" },
                ].filter(k => k.value).map((k, i) => (
                    <div key={i} className={cn("px-4 py-3 rounded-2xl border text-center transition-all", k.bg, k.border)}>
                        <p className="text-[10px] text-white/30 font-bold uppercase mb-1 tracking-tighter">{k.label}</p>
                        <p className={cn("text-sm font-black", k.color)}>{k.value}</p>
                    </div>
                ))}
            </div>
        </div>
    );
}
