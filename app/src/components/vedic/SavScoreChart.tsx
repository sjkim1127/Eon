interface Props {
  points: number[];
}

export function SavScoreChart({ points }: Props) {
  if (!points || points.length !== 12) return null;

  return (
    <div className="grid grid-cols-12 gap-2 items-end h-48">
      {points.map((pt: number, i: number) => {
        const maxPt = Math.max(...points, 1);
        const pct = (pt / maxPt) * 100;
        const isStrong = pt >= 28;
        const isWeak = pt < 25;
        return (
          <div key={i} className="flex flex-col items-center gap-1 h-full justify-end">
            <span className={`text-[10px] font-bold ${isStrong ? 'text-green-400' : isWeak ? 'text-red-400' : 'text-white/60'}`}>{pt}</span>
            <div
              className={`w-full rounded-t-lg transition-all ${isStrong ? 'bg-green-500/60' : isWeak ? 'bg-red-500/40' : 'bg-celestial-cyan/40'}`}
              style={{ height: `${pct}%`, minHeight: '4px' }}
            />
            <span className="text-[9px] text-white/30 font-bold">H{i + 1}</span>
          </div>
        );
      })}
    </div>
  );
}
