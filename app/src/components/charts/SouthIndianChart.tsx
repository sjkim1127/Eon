import { PLANET_ABBR, formatDeg } from "./chartUtils";

const SOUTH_GRID: (number | null)[][] = [
  [12, 1, 2, 3],
  [11, null, null, 4],
  [10, null, null, 5],
  [9, 8, 7, 6],
];

const SIGN_ABBR = ["", "Ar", "Ta", "Ge", "Cn", "Le", "Vi", "Li", "Sc", "Sg", "Cp", "Aq", "Pi"];

export interface ChartProps {
  lagnaRasi: number;
  planetEntries: { name: string; rasi: number; retro: boolean; deg?: number }[];
}

export function SouthIndianChart({ lagnaRasi, planetEntries }: ChartProps) {
  const bySign: Record<number, string[]> = {};
  if (lagnaRasi != null && !bySign[lagnaRasi]) bySign[lagnaRasi] = [];
  if (lagnaRasi != null) bySign[lagnaRasi].unshift("Lg");

  const planetsToRender = planetEntries || [];
  for (const p of planetsToRender) {
    if (!bySign[p.rasi]) bySign[p.rasi] = [];
    const abbr = PLANET_ABBR[p.name] ?? p.name.substring(0, 2);
    bySign[p.rasi].push(p.retro ? `${abbr}\u211e` : abbr);
  }
  return (
    <div className="grid grid-cols-4 gap-1 w-full max-w-[280px] shrink-0">
      {SOUTH_GRID.flatMap((row, ri) =>
        row.map((signNum, ci) => {
          if (signNum === null) return <div key={`${ri}-${ci}`} className="aspect-square" />;
          const houseNum = ((signNum - lagnaRasi + 12) % 12) + 1;
          const isLagna = signNum === lagnaRasi;

          // Get the actual planet objects for this sign to show degrees
          const planetsInSign = planetEntries.filter(p => p.rasi === signNum);
          const hasLg = isLagna;
          return (
            <div
              key={`${ri}-${ci}`}
              className={`border min-h-[64px] rounded-lg p-1.5 flex flex-col ${isLagna ? "border-celestial-gold/50 bg-amber-500/[0.08]" : "border-white/10 bg-white/[0.03]"
                }`}
            >
              <div className="flex justify-between items-center mb-1">
                <span className="text-[9px] text-white/35 font-mono">{SIGN_ABBR[signNum]}</span>
                <span className="text-[9px] text-white/25 font-bold">{houseNum}</span>
              </div>
              <div className="flex flex-col gap-0.5 mt-1">
                {hasLg && (
                  <span className="text-[10px] font-bold leading-none text-celestial-gold">Lg</span>
                )}
                {planetsInSign.map((p, i) => {
                  const abbr = PLANET_ABBR[p.name] ?? p.name.substring(0, 2);
                  const isRetro = p.retro;
                  return (
                    <div key={i} className="flex items-center gap-1">
                      <span className={`text-[10px] font-bold leading-none ${isRetro ? "text-amber-300" : "text-white/80"}`}>
                        {abbr}{isRetro && "℞"}
                      </span>
                      {p.deg !== undefined && (
                        <span className="text-[8px] text-white/40 leading-none">{formatDeg(p.deg)}</span>
                      )}
                    </div>
                  );
                })}
              </div>
            </div>
          );
        })
      )}
    </div>
  );
}
