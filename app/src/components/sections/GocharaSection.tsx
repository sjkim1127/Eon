import { Globe, ShieldAlert, Sparkles, AlertCircle } from "lucide-react";
import { SIGN_NAMES } from "../../constants";
import type { GocharaSummary } from "../../types/vedic";
import { cn } from "../../utils";

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

/** 응답에서 gochara가 와도 transits가 배열이 아닐 수 있음 (WASM 직렬화 등) — 정규화 */
function normalizeSummary(summary: GocharaSummary | null): GocharaSummary | null {
    if (!summary || typeof summary !== "object") return null;
    const transits = Array.isArray(summary.transits) ? summary.transits : [];
    return { ...summary, transits };
}

export function GocharaSection({ summary }: GocharaSectionProps) {
    const normalized = normalizeSummary(summary);
    if (!normalized || normalized.transits.length === 0) {
        return (
            <div className="glass p-8 rounded-[2rem]">
                <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                    <Globe className="w-6 h-6 text-celestial-cyan" />
                    고차라 트랜싯 (Gochara)
                </h5>
                <p className="text-white/50 text-sm">
                    {summary != null && typeof summary === "object"
                        ? "현재 트랜짓 데이터가 비어 있습니다. 출생 달 위치 기준으로 현재 행성 위치를 계산합니다. (웹에서는 WASM 빌드 후 표시될 수 있습니다.)"
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
                    {/* ... (keep table as is, but we can make it more compact) */}
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
                        {normalized.transits.map((t, i) => {
                            const murti = MURTI_LABELS[t.murti] ?? MURTI_LABELS.Unknown;
                            const signName = SIGN_NAMES[t.current_rasi] ?? `#${t.current_rasi}`;
                            return (
                                <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                                    <td className="py-2.5 pr-4 font-bold text-white whitespace-nowrap">{t.planet}</td>
                                    <td className="py-2.5 pr-4 text-white/70">{signName}</td>
                                    <td className="py-2.5 pr-4 text-white/50 font-mono">H{t.house_from_moon}</td>
                                    <td className="py-2.5 pr-4">
                                        {t.is_benefic_transit ? (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-green-500/15 text-green-400 border border-green-500/20 font-semibold">길 吉</span>
                                        ) : (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-red-500/15 text-red-400 border border-red-500/20 font-semibold">흉 凶</span>
                                        )}
                                    </td>
                                    <td className="py-2.5 pr-4">
                                        {t.is_blocked ? (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-amber-500/15 text-amber-400 border border-amber-500/20 font-semibold">차단됨</span>
                                        ) : (
                                            <span className="text-xs text-white/20">—</span>
                                        )}
                                    </td>
                                    <td className="py-2.5">
                                        <span className={`text-xs font-semibold ${murti.color}`}>{murti.emoji} {murti.label}</span>
                                    </td>
                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                {normalized.transits.filter(t => t.summary).map((t, i) => (
                    <div key={i} className="p-6 rounded-2xl bg-white/[0.03] border border-white/10 glass-hover">
                        <div className="flex items-center justify-between mb-4">
                            <div className="flex items-center gap-3">
                                <div className={cn("p-2 rounded-lg", t.is_benefic_transit ? "bg-green-500/10" : "bg-red-500/10")}>
                                    {t.is_benefic_transit ? <Sparkles className="w-4 h-4 text-green-400" /> : <ShieldAlert className="w-4 h-4 text-red-400" />}
                                </div>
                                <div>
                                    <h6 className="text-white font-bold">{t.planet} 트랜짓</h6>
                                    <p className="text-[10px] text-white/40 uppercase font-bold">{t.summary}</p>
                                </div>
                            </div>
                            {t.is_blocked && (
                                <div className="flex items-center gap-1.5 px-2 py-1 rounded bg-amber-500/10 border border-amber-500/20">
                                    <AlertCircle className="w-3 h-3 text-amber-400" />
                                    <span className="text-[10px] font-bold text-amber-400 leading-none">VEDHA</span>
                                </div>
                            )}
                        </div>
                        <p className="text-sm text-white/70 leading-relaxed mb-4">
                            {t.description}
                        </p>
                        {t.reasons && t.reasons.length > 0 && (
                            <div className="space-y-2">
                                {t.reasons.map((reason, j) => (
                                    <div key={j} className="flex items-start gap-2 text-[11px] text-white/40">
                                        <div className="w-1 h-1 rounded-full bg-white/20 mt-1.5 flex-shrink-0" />
                                        <span>{reason}</span>
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
