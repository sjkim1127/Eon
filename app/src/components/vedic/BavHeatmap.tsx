interface Props {
  bav: any[];
  savPoints?: number[];
}

export function BavHeatmap({ bav, savPoints }: Props) {
  if (!bav || bav.length === 0) return null;

  return (
    <div className="overflow-x-auto">
      <table className="text-xs w-full">
        <thead>
          <tr className="border-b border-white/10">
            <th className="text-left text-white/40 font-bold pb-3 pr-4 whitespace-nowrap">행성</th>
            {Array.from({ length: 12 }, (_, i) => (
              <th key={i} className="text-center text-white/40 font-bold pb-3 px-1.5 whitespace-nowrap">H{i + 1}</th>
            ))}
            <th className="text-center text-white/40 font-bold pb-3 pl-3 whitespace-nowrap">Pinda</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-white/5">
          {(bav || []).map((entry) => {
            const pts = (entry.points || []) as number[];
            const maxPt = Math.max(...pts, 1);
            return (
              <tr key={entry.planet} className="hover:bg-white/[0.03] transition-colors">
                <td className="py-2 pr-4 font-bold text-white whitespace-nowrap">{entry.planet}</td>
                {pts.map((pt, hi) => {
                  const pct = pt / maxPt;
                  const bg = pt >= 5
                    ? `rgba(74,222,128,${0.15 + pct * 0.3})`
                    : pt >= 3
                      ? `rgba(250,204,21,${0.15 + pct * 0.25})`
                      : `rgba(239,68,68,${0.1 + pct * 0.2})`;
                  const textColor = pt >= 5 ? "text-green-300" : pt >= 3 ? "text-yellow-300" : pt === 0 ? "text-white/15" : "text-red-300";
                  return (
                    <td
                      key={hi}
                      className={`py-2 px-1.5 text-center font-mono font-bold rounded transition-all ${textColor}`}
                      style={{ background: bg }}
                      title={`${entry.planet} → H${hi + 1}: ${pt} bindu`}
                    >
                      {pt}
                    </td>
                  );
                })}
                <td className="py-2 pl-3 text-center font-mono text-white/50 text-[11px]">{entry.sodya_pinda}</td>
              </tr>
            );
          })}
          {/* 합산(SAV) 행 */}
          {savPoints && (
            <tr className="border-t border-white/20">
              <td className="py-2 pr-4 font-bold text-celestial-gold whitespace-nowrap">SAV</td>
              {savPoints.map((pt, hi) => {
                const isStrong = pt >= 28;
                const isWeak = pt < 25;
                return (
                  <td
                    key={hi}
                    className={`py-2 px-1.5 text-center font-mono font-bold ${isStrong ? "text-green-400" : isWeak ? "text-red-400" : "text-white/70"}`}
                  >
                    {pt}
                  </td>
                );
              })}
              <td className="py-2 pl-3 text-center text-white/25 text-[10px]">—</td>
            </tr>
          )}
        </tbody>
      </table>
    </div>
  );
}
