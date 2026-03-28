import { Sparkles, Users, Target } from "lucide-react";
import type { ArudhaPada } from "../../../types/vedic";

// ── Avasthas Types ──
interface PlanetAvastha {
    planet: string;
    baladi: string;  // "Bala" | "Kumara" | "Yuva" | "Vriddha" | "Mrita"
    jagradadi: string; // "Jagrat" | "Swapna" | "Sushupti"
    deeptaadi: string;
}

// ── Karaka Types ──
interface KarakaAssignment {
    planet: string;
    role: string;
    degree_in_rasi: number;
}

const BALADI_INFO: Record<string, { label: string; color: string; emoji: string }> = {
    Bala: { label: "유아", color: "bg-blue-500/15 text-blue-300 border-blue-500/20", emoji: "👶" },
    Kumara: { label: "성장", color: "bg-green-500/15 text-green-300 border-green-500/20", emoji: "🌱" },
    Yuva: { label: "청년", color: "bg-yellow-500/15 text-yellow-300 border-yellow-500/20", emoji: "💪" },
    Vriddha: { label: "노년", color: "bg-orange-500/15 text-orange-300 border-orange-500/20", emoji: "🧓" },
    Mrita: { label: "사망", color: "bg-red-500/15 text-red-300 border-red-500/20", emoji: "💀" },
};

const JAGRADADI_INFO: Record<string, { label: string; color: string }> = {
    Jagrat: { label: "깨어남 (Awake)", color: "text-green-400" },
    Swapna: { label: "꿈 (Dream)", color: "text-amber-400" },
    Sushupti: { label: "수면 (Sleep)", color: "text-red-400" },
};

const DEEPTAADI_INFO: Record<string, { label: string; color: string }> = {
    Deepta: { label: "환희 (Exalted)", color: "text-celestial-gold" },
    Svastha: { label: "안정 (Own Sign)", color: "text-green-400" },
    Mudita: { label: "기쁨 (Great Friend)", color: "text-emerald-400" },
    Shanta: { label: "평온 (Friend)", color: "text-blue-400" },
    Deena: { label: "평범 (Neutral)", color: "text-white/60" },
    Dukhita: { label: "슬픔 (Enemy)", color: "text-orange-400" },
    Vikala: { label: "불안 (Great Enemy)", color: "text-red-400" },
    Khala: { label: "사악 (Debilitated)", color: "text-red-600 font-bold" },
    Kopita: { label: "분노 (Combust)", color: "text-orange-600 font-bold" },
};

const KARAKA_ROLES: Record<string, { label: string; meaning: string }> = {
    Atmakaraka: { label: "AK", meaning: "영혼 (Soul)" },
    Amatyakaraka: { label: "AmK", meaning: "직업 (Career)" },
    Bhratrukaraka: { label: "BK", meaning: "형제 (Siblings)" },
    Matrukaraka: { label: "MK", meaning: "어머니 (Mother)" },
    Pitrikaraka: { label: "PiK", meaning: "아버지 (Father)" },
    Putrakaraka: { label: "PK", meaning: "자녀 (Children)" },
    Gnatikaraka: { label: "GK", meaning: "경쟁자 (Rivals)" },
    Darakaraka: { label: "DK", meaning: "배우자 (Spouse)" },
};

interface AvasthaKarakaSectionProps {
    avasthas: PlanetAvastha[];
    karakas: KarakaAssignment[];
    arudhaPadas?: ArudhaPada[];
}

export function AvasthaKarakaSection({ avasthas, karakas, arudhaPadas }: AvasthaKarakaSectionProps) {
    const hasAvasthas = avasthas && avasthas.length > 0;
    const hasKarakas = karakas && karakas.length > 0;

    return (
        <div className="space-y-8">
            <div className="glass p-8 rounded-[2rem]">
                {/* Karakas (Jaimini) */}
                {hasKarakas && (
                    <div className="mb-10">
                        <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
                            <Users className="w-6 h-6 text-celestial-gold" />
                            제미니 카라카 (Jaimini Karakas)
                        </h5>
                        <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
                            {karakas.map((k, i) => {
                                const roleInfo = KARAKA_ROLES[k.role] ?? { label: k.role.slice(0, 3), meaning: k.role };
                                return (
                                    <div key={i} className="p-3 rounded-xl bg-white/[0.03] border border-white/10 text-center">
                                        <p className="text-[10px] text-celestial-gold font-bold mb-1">{roleInfo.label}</p>
                                        <p className="text-white font-bold text-sm">{k.planet}</p>
                                        <p className="text-[10px] text-white/40 mt-1">{roleInfo.meaning}</p>
                                        <p className="text-[10px] text-white/25 font-mono">{k.degree_in_rasi.toFixed(2)}°</p>
                                    </div>
                                );
                            })}
                        </div>
                    </div>
                )}

                {/* Arudha Padas */}
                {arudhaPadas && arudhaPadas.length > 0 && (
                    <div className="mb-10">
                        <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
                            <Target className="w-6 h-6 text-celestial-cyan" />
                            아루다 파다 (Arudha Padas)
                        </h5>
                        <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-3">
                            {arudhaPadas.map((ap, i) => (
                                <div key={i} className="p-3 rounded-xl bg-white/[0.03] border border-white/10 text-center">
                                    <p className="text-[9px] text-white/40 font-bold uppercase mb-1">{ap.name.split(' (')[1]?.replace(')', '') || ap.name}</p>
                                    <p className="text-sm font-bold text-white">사인 {ap.rasi}</p>
                                    <p className="text-[9px] text-white/20 mt-1">H{ap.house} Arudha</p>
                                </div>
                            ))}
                        </div>
                    </div>
                )}

                {/* Avasthas (Planetary State) */}
                {hasAvasthas && (
                    <div>
                        <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
                            <Sparkles className="w-6 h-6 text-violet-400" />
                            행성 상태 (Avasthas)
                        </h5>
                        <div className="overflow-x-auto">
                            <table className="w-full text-sm">
                                <thead>
                                    <tr className="border-b border-white/10">
                                        <th className="text-left text-xs text-white/40 font-bold pb-3 pr-4">행성</th>
                                        <th className="text-left text-xs text-white/40 font-bold pb-3 pr-4">딥타디 (위엄)</th>
                                        <th className="text-left text-xs text-white/40 font-bold pb-3 pr-4">발라디 (연령)</th>
                                        <th className="text-left text-xs text-white/40 font-bold pb-3">자그라다디 (의식)</th>
                                    </tr>
                                </thead>
                                <tbody className="divide-y divide-white/5">
                                    {avasthas.map((a, i) => {
                                        const baladi = BALADI_INFO[a.baladi] ?? { label: a.baladi, color: "bg-white/10 text-white/50 border-white/10", emoji: "❓" };
                                        const jagradadi = JAGRADADI_INFO[a.jagradadi] ?? { label: a.jagradadi, color: "text-white/50" };
                                        const deeptaadi = DEEPTAADI_INFO[a.deeptaadi] ?? { label: a.deeptaadi, color: "text-white/50" };

                                        return (
                                            <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                                                <td className="py-2.5 pr-4 font-bold text-white">{a.planet}</td>
                                                <td className="py-2.5 pr-4">
                                                    <span className={`text-xs font-semibold ${deeptaadi.color}`}>{deeptaadi.label}</span>
                                                </td>
                                                <td className="py-2.5 pr-4">
                                                    <span className={`text-[11px] px-2 py-0.5 rounded-full border font-semibold ${baladi.color}`}>
                                                        {baladi.emoji} {baladi.label}
                                                    </span>
                                                </td>
                                                <td className="py-2.5">
                                                    <span className={`text-xs font-semibold ${jagradadi.color}`}>{jagradadi.label}</span>
                                                </td>
                                            </tr>
                                        );
                                    })}
                                </tbody>
                            </table>
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
}
