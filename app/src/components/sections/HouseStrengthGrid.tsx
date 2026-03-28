import { cn } from "../../utils";

interface HouseStrengthGridProps {
    houseSummary: any[];
    bhavaStrengths: any[];
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
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
                {houseSummary.map((house: any) => {
                    const bhava = bhavaStrengths.find((b: any) => b.house === house.house);
                    return (
                        <div
                            key={house.house}
                            className="glass p-6 rounded-2xl text-center glass-hover cursor-help"
                        >
                            <p className="text-[10px] text-white/40 font-bold uppercase mb-1">House {house.house}</p>
                            <p className={cn("text-xs font-black mb-3",
                                house.rating === "Excellent" ? "text-emerald-400" :
                                    house.rating === "Strong" ? "text-celestial-cyan" :
                                        house.rating === "Average" ? "text-white/60" : "text-orange-400"
                            )}>
                                {ratingLabel(house.rating)}
                            </p>
                            <div className="flex flex-col gap-1.5 mt-4">
                                <div className="flex justify-between text-[9px]">
                                    <span className="text-white/30">로드</span>
                                    <span className="text-white/60 font-bold">{bhava?.lord_score?.toFixed(0)}</span>
                                </div>
                                <div className="flex justify-between text-[9px]">
                                    <span className="text-white/30">방위</span>
                                    <span className="text-white/60 font-bold">{bhava?.dig_score?.toFixed(0)}</span>
                                </div>
                                <div className="w-full bg-white/5 h-1 rounded-full overflow-hidden mt-1">
                                    <div
                                        className={cn("h-full rounded-full transition-all duration-1000",
                                            house.rating === "Excellent" ? "bg-emerald-500" :
                                                house.rating === "Strong" ? "bg-celestial-cyan" : "bg-white/20"
                                        )}
                                        style={{ width: `${(house.total_score / 600) * 100}%` }}
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
