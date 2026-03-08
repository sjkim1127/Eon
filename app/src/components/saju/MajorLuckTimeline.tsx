import { TrendingUp } from "lucide-react";
import { TENGOD_INFO } from "../../constants";
import { ganziDisplay, ganziHangul } from "../../utils";

interface Props {
  ml: any;
}

export function MajorLuckTimeline({ ml }: Props) {
  if (!ml || !ml.cycles) return null;

  return (
    <div className="glass p-8 rounded-[2rem]">
      <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
        <TrendingUp className="w-6 h-6 text-celestial-purple" />
        대운 (大運) 흐름
      </h5>
      <p className="text-sm text-white/50 mb-4">
        방향: {ml.direction === "Forward" ? "순행 ▶" : "역행 ◀"} | 시작 나이: {ml.start_age}세
      </p>
      <div className="grid grid-cols-2 sm:grid-cols-5 lg:grid-cols-10 gap-3">
        {ml.cycles.map((c: any, i: number) => (
          <div
            key={i}
            className="p-3 bg-white/5 rounded-xl border border-white/10 text-center hover:bg-white/10 transition-all"
          >
            <p className="text-xs text-white/40 mb-1">{c.start_age}~{c.end_age}세</p>
            <p className="text-lg font-bold text-white">{ganziDisplay(c.ganzi)}</p>
            <p className="text-xs text-white/30">{ganziHangul(c.ganzi)}</p>
            <p className="text-[10px] text-celestial-gold mt-1">{TENGOD_INFO[c.stem_god]?.hangul || c.stem_god || ""}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
