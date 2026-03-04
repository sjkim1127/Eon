import { Globe } from "lucide-react";
import { SIGN_NAMES } from "../../constants";
import type { GocharaSummary, TransitPosition } from "../../types/vedic";

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
    if (!summary || !summary.transits || summary.transits.length === 0) return null;

    return (
        <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                <Globe className="w-6 h-6 text-celestial-cyan" />
                고차라 트랜싯 (Gochara — 현재 행성 위치)
            </h5>

            <div className="overflow-x-auto">
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
                        {summary.transits.map((t: TransitPosition, i: number) => {
                            const murti = MURTI_LABELS[t.murti] ?? MURTI_LABELS.Unknown;
                            const signName = SIGN_NAMES[t.current_rasi] ?? `#${t.current_rasi}`;

                            return (
                                <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                                    <td className="py-2.5 pr-4 font-bold text-white whitespace-nowrap">{t.planet}</td>
                                    <td className="py-2.5 pr-4 text-white/70">{signName}</td>
                                    <td className="py-2.5 pr-4 text-white/50 font-mono">H{t.house_from_moon}</td>
                                    <td className="py-2.5 pr-4">
                                        {t.is_benefic_transit ? (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-green-500/15 text-green-400 border border-green-500/20 font-semibold">
                                                길 吉
                                            </span>
                                        ) : (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-red-500/15 text-red-400 border border-red-500/20 font-semibold">
                                                흉 凶
                                            </span>
                                        )}
                                    </td>
                                    <td className="py-2.5 pr-4">
                                        {t.is_blocked ? (
                                            <span className="text-xs px-2 py-0.5 rounded-full bg-amber-500/15 text-amber-400 border border-amber-500/20 font-semibold">
                                                차단됨
                                            </span>
                                        ) : (
                                            <span className="text-xs text-white/20">—</span>
                                        )}
                                    </td>
                                    <td className="py-2.5">
                                        <span className={`text-xs font-semibold ${murti.color}`}>
                                            {murti.emoji} {murti.label}
                                        </span>
                                    </td>
                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            </div>
        </div>
    );
}
