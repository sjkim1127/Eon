import { TrendingUp } from "lucide-react";
import { TENGOD_INFO } from "../../constants";
import { ganziHangul } from "../../utils";

const STEM_TO_COLOR: Record<string, string> = {
  Jia: "text-emerald-400", Yi: "text-emerald-400",
  Bing: "text-rose-400", Ding: "text-rose-400",
  Wu: "text-amber-400", Ji: "text-amber-400",
  Geng: "text-white", Xin: "text-white",
  Ren: "text-celestial-cyan", Gui: "text-celestial-cyan",
};

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
      <div className="flex items-center justify-between gap-4 mb-4">
        <h6 className="text-[12px] font-bold text-white/40 uppercase tracking-widest">
          방향: {ml.direction === "Forward" ? "순행 ▶" : "역행 ◀"} | 시작 나이: {ml.startAge}세
        </h6>
      </div>
      <div className="grid grid-cols-2 sm:grid-cols-5 md:grid-cols-10 gap-3">
        {ml.cycles.map((c: any, i: number) => (
          <div
            key={i}
            className="group relative p-3 rounded-xl bg-white/5 border border-white/10 transition-all duration-300 hover:bg-celestial-gold/10 hover:border-celestial-gold/30"
          >
            <p className="text-xs text-white/40 mb-1">{c.startAge}~{c.endAge}세</p>
            <div className={`text-sm font-bold ${STEM_TO_COLOR[c.ganzi.stem] || 'text-white'}`}>
              {ganziHangul(c.ganzi)}
            </div>
            <p className="text-[10px] text-celestial-gold mt-1">{TENGOD_INFO[c.stemGod]?.hangul || c.stemGod || ""}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
