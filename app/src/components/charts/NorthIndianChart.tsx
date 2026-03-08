import { PLANET_ABBR, formatDeg } from "./chartUtils";

const NORTH_HOUSE_CENTERS = [
  null,
  { x: 200, y: 90 },  // H1
  { x: 90, y: 50 },   // H2
  { x: 50, y: 90 },   // H3
  { x: 90, y: 200 },  // H4
  { x: 50, y: 310 },  // H5
  { x: 90, y: 350 },  // H6
  { x: 200, y: 310 }, // H7
  { x: 310, y: 350 }, // H8
  { x: 350, y: 310 }, // H9
  { x: 310, y: 200 }, // H10
  { x: 350, y: 90 },  // H11
  { x: 310, y: 50 },  // H12
];

const NORTH_SIGN_POS = [
  null,
  { x: 195, y: 175 }, // H1 bottom
  { x: 170, y: 25 },  // H2 
  { x: 25, y: 170 },  // H3 
  { x: 175, y: 195 }, // H4 right
  { x: 25, y: 235 },  // H5 
  { x: 170, y: 385 }, // H6 
  { x: 195, y: 235 }, // H7 top
  { x: 240, y: 385 }, // H8 
  { x: 385, y: 235 }, // H9 
  { x: 235, y: 195 }, // H10 left
  { x: 385, y: 170 }, // H11
  { x: 240, y: 25 },  // H12
];

export interface ChartProps {
  lagnaRasi: number;
  planetEntries: { name: string; rasi: number; retro: boolean; deg?: number }[];
}

export function NorthIndianChart({ lagnaRasi, planetEntries }: ChartProps) {
  const byHouse: Record<number, typeof planetEntries> = {};
  for (const p of planetEntries) {
    const houseNum = ((p.rasi - lagnaRasi + 12) % 12) + 1;
    if (!byHouse[houseNum]) byHouse[houseNum] = [];
    byHouse[houseNum].push(p);
  }

  return (
    <div className="relative w-full max-w-[280px] shrink-0 aspect-square">
      <svg viewBox="0 0 400 400" className="w-full h-full overflow-hidden rounded-lg bg-white/[0.02] border border-white/10">
        <g stroke="rgba(255,255,255,0.15)" strokeWidth="2" fill="none">
          <rect x="0" y="0" width="400" height="400" />
          <line x1="0" y1="0" x2="400" y2="400" />
          <line x1="400" y1="0" x2="0" y2="400" />
          <line x1="200" y1="0" x2="400" y2="200" />
          <line x1="400" y1="200" x2="200" y2="400" />
          <line x1="200" y1="400" x2="0" y2="200" />
          <line x1="0" y1="200" x2="200" y2="0" />
        </g>

        {/* Draw contents for all 12 houses */}
        {NORTH_HOUSE_CENTERS.slice(1).map((center, index) => {
          if (!center) return null;
          const houseNum = index + 1;
          const signNum = ((lagnaRasi + houseNum - 2) % 12) + 1;
          const pList = byHouse[houseNum] || [];
          const isLagna = houseNum === 1;
          const signPos = NORTH_SIGN_POS[houseNum] || { x: center.x, y: center.y };

          return (
            <g key={`h${houseNum}`}>
              {/* Sign Number */}
              <text x={signPos.x} y={signPos.y} fill="rgba(255,255,255,0.3)" fontSize="14" fontWeight="bold">
                {signNum}
              </text>

              {/* Planets */}
              <foreignObject x={center.x - 45} y={center.y - 40} width="90" height="80">
                <div className="w-full h-full flex flex-col items-center justify-center pointer-events-none">
                  <div className="flex flex-wrap items-center justify-center gap-x-2 gap-y-0.5">
                    {isLagna && <span className="text-[11px] font-bold text-celestial-gold">Lg</span>}
                    {pList.map((p, i) => {
                      const abbr = PLANET_ABBR[p.name] ?? p.name.substring(0, 2);
                      return (
                        <div key={i} className="flex items-center gap-1">
                          <span className={`text-[11px] font-bold leading-none ${p.retro ? "text-amber-300" : "text-white/80"}`}>
                            {abbr}{p.retro && "℞"}
                          </span>
                          {p.deg !== undefined && (
                            <span className="text-[9px] text-white/40 leading-none">{formatDeg(p.deg)}</span>
                          )}
                        </div>
                      );
                    })}
                  </div>
                </div>
              </foreignObject>
            </g>
          );
        })}
      </svg>
    </div>
  );
}
