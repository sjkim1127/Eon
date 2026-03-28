import { Star } from "lucide-react";
import type { VedicPanchanga } from "../../types/vedic";

interface VedicPanchangaSectionProps {
    panchanga: VedicPanchanga;
    nakshatraInfo: string;
}

export function VedicPanchangaSection({ panchanga, nakshatraInfo }: VedicPanchangaSectionProps) {
    return (
        <div className="glass p-10 rounded-[2.5rem]">
            <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
                <Star className="w-6 h-6 text-celestial-gold" />
                낙샤트라 청사진
            </h5>
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                <div className="p-6 bg-white/5 rounded-2xl border border-white/10">
                    <p className="text-[10px] text-white/40 font-bold uppercase mb-2">Birth Star Details</p>
                    <p className="text-lg text-white font-bold leading-tight">
                        {nakshatraInfo}
                    </p>
                </div>
                <div className="p-6 bg-celestial-gold/5 rounded-2xl border border-celestial-gold/10">
                    <p className="text-[10px] text-celestial-gold/60 font-bold uppercase mb-2">Lunar Phase & Day</p>
                    <p className="text-lg text-celestial-gold font-bold">
                        {panchanga.tithi_name} ({panchanga.is_day_birth ? "주간" : "야간"} 출생 · {panchanga.vara}요일)
                    </p>
                </div>
            </div>
        </div>
    );
}
