import { Eye } from "lucide-react";
import type { AspectRelation } from "../../../types/vedic";

const SIGN_NAMES: Record<number, string> = {
    1: "Aries", 2: "Taurus", 3: "Gemini", 4: "Cancer",
    5: "Leo", 6: "Virgo", 7: "Libra", 8: "Scorpio",
    9: "Sagittarius", 10: "Capricorn", 11: "Aquarius", 12: "Pisces",
};

interface AspectsSectionProps {
    aspects: AspectRelation[];
}

export function AspectsSection({ aspects }: AspectsSectionProps) {
    if (!aspects || aspects.length === 0) {
        return (
            <div className="glass p-8 rounded-[2rem]">
                <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                    <Eye className="w-6 h-6 text-celestial-cyan" />
                    행성 시선 (Planetary Aspects / Drishti)
                </h5>
                <p className="text-white/50 text-sm">데이터를 불러올 수 없습니다. (배열이 비어있음)</p>
            </div>
        );
    }

    return (
        <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                <Eye className="w-6 h-6 text-celestial-cyan" />
                행성 시선 (Planetary Aspects / Drishti)
            </h5>
            <div className="overflow-x-auto">
                <table className="w-full text-sm">
                    <thead>
                        <tr className="border-b border-white/10">
                            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">시선 행성</th>
                            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">시선 대상 하우스</th>
                        </tr>
                    </thead>
                    <tbody className="divide-y divide-white/5">
                        {aspects.map((a, i) => (
                            <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                                <td className="py-3 pr-4 font-bold text-white whitespace-nowrap">
                                    {a.aspecting_planet}
                                </td>
                                <td className="py-3 pr-4">
                                    <div className="flex flex-wrap gap-1.5">
                                        {a.aspected_houses.map(h => (
                                            <span
                                                key={h}
                                                className="text-xs px-2 py-0.5 rounded-lg bg-celestial-cyan/10 text-celestial-cyan border border-celestial-cyan/20 font-semibold"
                                            >
                                                H{h} {SIGN_NAMES[h] ? `(${SIGN_NAMES[h]})` : ""}
                                            </span>
                                        ))}
                                    </div>
                                </td>
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
}
