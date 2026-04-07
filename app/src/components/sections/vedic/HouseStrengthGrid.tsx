import { cn } from "../../../utils";
import type { HouseSummary, BhavaStrength } from "../../../types/vedic";

interface HouseStrengthGridProps {
    houseSummary: HouseSummary[];
    bhavaStrengths: BhavaStrength[];
}

export function HouseStrengthGrid({ houseSummary, bhavaStrengths }: HouseStrengthGridProps) {
    const ratingLabel = (rating: string) => {
        if (rating === "Excellent") return "최상";
        if (rating === "Strong") return "강함";
        if (rating === "Average") return "보통";
        return "약함";
    };

    return (
        <section>
            <h5 className="text-xl font-bold text-white mb-6">하우스(Bhava)별 에너지 역량 상세</h5>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                {houseSummary.map((house) => {
                    const rawBhava = bhavaStrengths.find((b) => b.house === house.house);
                    const lordScore = rawBhava?.lordScore ?? 0;
                    const totalScore = house.totalScore ?? 0;
                    
                    return (
                        <div
                            key={house.house}
                            className="glass p-6 rounded-[2rem] glass-hover group border-white/5"
                        >
                            <div className="flex justify-between items-start mb-4">
                                <div>
                                    <p className="text-[10px] text-white/40 font-black uppercase tracking-widest mb-1">House {house.house}</p>
                                    <h6 className="text-sm font-bold text-white/90">{house.summary}</h6>
                                </div>
                                <div className={cn("px-3 py-1 rounded-full text-[10px] font-black",
                                    house.rating === "Excellent" ? "bg-emerald-500/20 text-emerald-400 border border-emerald-500/30" :
                                        house.rating === "Strong" ? "bg-celestial-cyan/20 text-celestial-cyan border border-celestial-cyan/30" :
                                            house.rating === "Average" ? "bg-white/10 text-white/60 border border-white/20" : 
                                            "bg-orange-500/20 text-orange-400 border border-orange-500/30"
                                )}>
                                    {ratingLabel(house.rating)}
                                </div>
                            </div>

                            <p className="text-[11px] text-white/50 leading-relaxed mb-4 line-clamp-2">
                                {house.description}
                            </p>

                            <div className="space-y-2 mb-4 bg-black/20 p-3 rounded-xl border border-white/5">
                                {house.reasons.map((reason, idx) => (
                                    <div key={idx} className="flex items-start gap-2 text-[10px] text-white/40">
                                        <div className="w-1 h-1 rounded-full bg-white/20 mt-1.5 shrink-0" />
                                        <span>{reason}</span>
                                    </div>
                                ))}
                            </div>

                            <div className="flex flex-col gap-2 pt-2 border-t border-white/5">
                                <div className="flex justify-between text-[10px]">
                                    <span className="text-white/30">로드 파워 (Lord)</span>
                                    <span className="text-white/70 font-mono italic">{lordScore.toFixed(1)}</span>
                                </div>
                                <div className="w-full bg-white/5 h-1.5 rounded-full overflow-hidden">
                                    <div
                                        className={cn("h-full rounded-full transition-all duration-1000",
                                            house.rating === "Excellent" ? "bg-emerald-500" :
                                                house.rating === "Strong" ? "bg-celestial-cyan" : "bg-white/20"
                                        )}
                                        style={{ width: `${Math.min(100, (totalScore / 600) * 100)}%` }}
                                    />
                                </div>
                            </div>
                        </div>
                    );
                })}
            </div>
        </section>
    );
}
