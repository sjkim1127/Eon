import { Zap } from "lucide-react";
import { cn } from "../../utils";
import type { Yoga } from "../../types/vedic";

interface VedicYogaHighlightSectionProps {
    yogas: Yoga[];
}

export function VedicYogaHighlightSection({ yogas }: VedicYogaHighlightSectionProps) {
    if (!yogas || yogas.length === 0) return null;

    return (
        <div className="flex gap-4 overflow-x-auto pb-4 snap-x">
            {yogas.map((yoga, idx) => {
                const isVeryHigh = yoga.quality === "VeryHigh";
                return (
                    <div
                        key={idx}
                        className={cn(
                            "min-width-[280px] flex-shrink-0 snap-center glass p-6 rounded-2xl border transition-all glass-hover",
                            isVeryHigh ? "border-celestial-gold/30 bg-celestial-gold/5" : "border-white/10 bg-white/5"
                        )}
                    >
                        <div className="flex items-center gap-2 mb-3">
                            <Zap className={cn("w-4 h-4", isVeryHigh ? "text-celestial-gold" : "text-brand-400")} />
                            <span className="text-sm font-bold text-white">{yoga.name}</span>
                            <span className="text-[10px] px-2 py-0.5 rounded-full bg-white/10 text-white/60">
                                {yoga.yoga_type}
                            </span>
                        </div>
                        <p className="text-xs text-white/60 line-clamp-2 leading-relaxed">
                            {yoga.description}
                        </p>
                    </div>
                );
            })}
        </div>
    );
}
