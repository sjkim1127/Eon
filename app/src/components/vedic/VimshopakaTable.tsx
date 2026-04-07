interface Props {
  vimshopaka: [string, any][];
}

export function VimshopakaTable({ vimshopaka }: Props) {
  if (!vimshopaka || vimshopaka.length === 0) return null;

  return (
    <div className="overflow-x-auto">
      <table className="w-full text-sm">
        <thead>
          <tr className="border-b border-white/10">
            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">행성</th>
            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">6분할 점수</th>
            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4">16분할 점수</th>
            <th className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3">힘 그래프</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-white/5">
          {(vimshopaka || []).map(([planet, score]: [string, any], i: number) => {
            const shad = score?.shadvarga_score ?? score?.shadvargaScore ?? 0;
            const shod = score?.shodashavarga_score ?? score?.shodashavargaScore ?? 0;
            const avg = (shad + shod) / 2;
            const pct = (avg / 20) * 100;
            return (
              <tr key={i} className="hover:bg-white/[0.03] transition-colors">
                <td className="py-2.5 pr-4 font-bold text-white whitespace-nowrap">{planet}</td>
                <td className="py-2.5 pr-4 text-celestial-cyan font-mono">{shad.toFixed(1)}</td>
                <td className="py-2.5 pr-4 text-celestial-purple font-mono">{shod.toFixed(1)}</td>
                <td className="py-2.5 pr-4 w-48">
                  <div className="flex items-center gap-2">
                    <div className="flex-1 bg-white/10 h-2 rounded-full overflow-hidden">
                      <div
                        className={`h-full rounded-full transition-all ${avg >= 15 ? 'bg-green-500' : avg >= 10 ? 'bg-celestial-cyan' : avg >= 5 ? 'bg-yellow-500' : 'bg-red-500'}`}
                        style={{ width: `${pct}%` }}
                      />
                    </div>
                    <span className="text-xs text-white/50 font-mono w-8 text-right">{avg.toFixed(1)}</span>
                  </div>
                </td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </div>
  );
}
