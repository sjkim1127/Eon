import { Heart, Clock } from "lucide-react";
import { cn } from "../../utils";
import type { VedicAnalysisReport } from "../../types/vedic";

interface VedicOverviewSectionProps {
    report: VedicAnalysisReport;
}

export function VedicOverviewSection({ report }: VedicOverviewSectionProps) {
    const { primary_karakas, dasha_focus, overall_strength_score } = report;

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
                        {primary_karakas.atmakaraka}
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
                        {primary_karakas.amatyakaraka}
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
                        {primary_karakas.darakaraka}
                    </h4>
                    <p className="text-sm text-white/60 leading-relaxed">
                        배우자·가깥은 파트너와의 관계 패턴을 나타내는 행성입니다.
                    </p>
                </div>
            </div>

            {/* ── Jaimini Secondary Karakas ────────────────────────────────── */}
            <div className="grid grid-cols-2 md:grid-cols-5 gap-4">
                {[
                    { label: "형제 (Bhratrukaraka)", value: primary_karakas.bhratrukaraka, color: "text-orange-400", bg: "bg-orange-500/5", border: "border-orange-500/10" },
                    { label: "어머니 (Matrukaraka)", value: primary_karakas.matrukaraka, color: "text-emerald-400", bg: "bg-emerald-500/5", border: "border-emerald-500/10" },
                    { label: "아버지 (Pitrikaraka)", value: primary_karakas.pitrikaraka, color: "text-blue-400", bg: "bg-blue-500/5", border: "border-blue-500/10" },
                    { label: "자녀 (Putrakaraka)", value: primary_karakas.putrakaraka, color: "text-yellow-400", bg: "bg-yellow-500/5", border: "border-yellow-500/10" },
                    { label: "경쟁자 (Gnatikaraka)", value: primary_karakas.gnatikaraka, color: "text-red-400", bg: "bg-red-500/5", border: "border-red-500/10" },
                ].filter(k => k.value).map((k, i) => (
                    <div key={i} className={cn("glass p-4 rounded-2xl border bg-white/5", k.bg, k.border)}>
                        <p className="text-[10px] text-white/40 font-bold uppercase mb-1">{k.label}</p>
                        <p className={cn("text-lg font-bold", k.color)}>{k.value}</p>
                    </div>
                ))}
            </div>

            {/* ── Dasha Focus & Overall Strength ──────────────────────────────── */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
                    <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
                        현재 대운 (Dasha)
                    </p>
                    <h4 className="text-3xl font-bold text-white mb-4">
                        {dasha_focus.replace("Current Major Period: ", "")}
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
                            {Math.round(overall_strength_score ?? 0)}
                        </h4>
                        <span className="text-white/20 font-bold">/ 600</span>
                    </div>
                    <div className="w-full bg-white/5 h-2 rounded-full overflow-hidden">
                        <div
                            className="bg-celestial-purple h-full rounded-full transition-all duration-1000"
                            style={{ width: `${((overall_strength_score ?? 0) / 600) * 100}%` }}
                        />
                    </div>
                </div>
            </div>
        </div>
    );
}
