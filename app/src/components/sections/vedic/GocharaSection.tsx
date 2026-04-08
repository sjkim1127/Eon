import { Globe, Sparkles } from "lucide-react";
import { SIGN_NAMES } from "../../../constants";
import type { GocharaSummary } from "../../../types/vedic";
import { cn } from "../../../utils";

const MURTI_LABELS: Record<string, { label: string; emoji: string; color: string }> = {
    Gold: { label: "금", emoji: "🥇", color: "text-yellow-300" },
    Silver: { label: "은", emoji: "🥈", color: "text-slate-200" },
    Copper: { label: "동", emoji: "🥉", color: "text-orange-400" },
    Iron: { label: "철", emoji: "⚙️", color: "text-gray-400" },
    Unknown: { label: "—", emoji: "", color: "text-white/30" },
};

interface GocharaSectionProps {
    summary: GocharaSummary | null;
}

export function GocharaSection({ summary }: GocharaSectionProps) {
    if (!summary || !summary.transits || summary.transits.length === 0) {
        return (
            <div className="glass p-8 rounded-[2rem]">
                <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                    <Globe className="w-6 h-6 text-celestial-cyan" />
                    고차라 트랜싯 (Gochara)
                </h5>
                <p className="text-white/50 text-sm">
                    {summary != null && typeof summary === "object"
                        ? "현재 트랜짓 데이터가 비어 있습니다. 출생 달 위치 기준으로 현재 행성 위치를 계산합니다."
                        : "데이터를 불러올 수 없습니다. 베딕 분석을 실행한 뒤 다시 확인해 주세요."}
                </p>
            </div>
        );
    }

    return (
        <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                <Globe className="w-6 h-6 text-celestial-cyan" />
                고차라 트랜싯 (Gochara — 현재 행성 위치)
            </h5>

            <div className="overflow-x-auto mb-10">
                <table className="w-full text-sm">
                    <thead>
                        <tr className="border-b border-white/10">
                            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">행성</th>
                            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">현재 라시</th>
                            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">Moon 기준</th>
                            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">길흉</th>
                            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">Vedha</th>
                            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3">무르띠</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y divide-white/5">
                        {summary.transits.map((t, i) => {
                            const murti = MURTI_LABELS[t.murti] ?? MURTI_LABELS.Unknown;
                            const signNum = Number(t.currentRasi ?? 0);
                            const houseNum = Number(t.houseFromMoon ?? 0);
                            const signName = signNum >= 1 && signNum <= 12 ? SIGN_NAMES[signNum] : "—";
                            return (
                                <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                                    <td className="py-2.5 pr-4 font-bold text-white whitespace-nowrap">{t.planet ?? "—"}</td>
                                    <td className="py-2.5 pr-4 text-white/70">{signName}</td>
                                    <td className="py-2.5 pr-4 text-white/50 font-mono">{houseNum ? `H${houseNum}` : "—"}</td>
                                    <td className="py-2.5 pr-4">
                                        {t.isBeneficTransit ? (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-green-500/15 text-green-400 border border-green-500/20 font-semibold">길 吉</span>
                                        ) : (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-red-500/15 text-red-400 border border-red-500/20 font-semibold">흉 凶</span>
                                        )}
                                    </td>
                                    <td className="py-2.5 pr-4">
                                        {t.isBlocked ? (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-amber-500/15 text-amber-400 border border-amber-500/20 font-semibold">차단됨</span>
                                        ) : (
                                            <span className="text-xs text-white/20">—</span>
                                        )}
                                    </td>
                                    <td className="py-2.5">
                                        <div className={cn("flex flex-col items-start gap-1 font-bold", murti.color)}>
                                            <span className="text-[10px] leading-tight text-white/40">{murti.emoji} MURTI</span>
                                            <span className="text-sm font-bold tracking-widest">{murti.label}</span>
                                        </div>
                                    </td>
                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {summary.transits.slice(0, 7).filter(t => t.summary).map((t, i) => (
                    <div key={i} className="p-6 bg-white/[0.03] rounded-2xl border border-white/10 hover:border-white/20 transition-all">
                        <div className="flex items-center justify-between mb-4">
                            <span className="font-bold text-white">{t.planet} 요약</span>
                            <span className={cn("text-xs font-bold px-2 py-1 rounded-lg", t.isBeneficTransit ? "bg-green-500/10 text-green-400" : "bg-red-500/10 text-red-400")}>
                                {t.isBeneficTransit ? "긍정적 주기" : "주의 필요 주기"}
                            </span>
                        </div>
                        <p className="text-sm text-white/70 leading-relaxed mb-4">{t.summary}</p>
                        {t.reasons && t.reasons.length > 0 && (
                            <div className="space-y-1.5 border-t border-white/5 pt-4">
                                {t.reasons.map((r, rIdx) => (
                                    <div key={rIdx} className="flex items-center gap-2 text-xs text-white/40">
                                        <Sparkles className="w-3 h-3 text-celestial-cyan/50" />
                                        <span>{r}</span>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
}
