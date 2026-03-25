import { Shield } from "lucide-react";
import { SPIRIT_INFO, PILLAR_POS_INFO } from "../../constants";

interface Props {
  sp: any;
}

export function SpiritsList({ sp }: Props) {
  if (!sp?.markers || sp.markers.length === 0) return null;

  const order = ["Year", "Month", "Day", "Hour"];
  
  const groups: Record<string, any[]> = {};
  for (const m of sp.markers) {
    const pos = m.position || "Unknown";
    if (!groups[pos]) groups[pos] = [];
    groups[pos].push(m);
  }

  const sortedPositions = Object.keys(groups).sort((a, b) => {
    const idxA = order.indexOf(a);
    const idxB = order.indexOf(b);
    if (idxA === -1 && idxB === -1) return a.localeCompare(b);
    if (idxA === -1) return 1;
    if (idxB === -1) return -1;
    return idxA - idxB;
  });

  return (
    <div className="glass p-8 rounded-[2rem]">
      <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
        <Shield className="w-6 h-6 text-celestial-cyan" />
        신살 (神煞) 분석
      </h5>
      <div className="space-y-6">
        {sortedPositions.map((pos) => (
          <div key={pos}>
            <div className="flex items-center gap-3 mb-3">
              <span className="px-3 py-1 rounded bg-white/10 text-white/80 text-sm font-bold tracking-wide">
                {PILLAR_POS_INFO[pos] || pos}
              </span>
              <div className="flex-1 h-px bg-white/10" />
            </div>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
              {groups[pos].map((m: any, i: number) => {
                const info = SPIRIT_INFO[m.marker];
                return (
                  <div
                    key={i}
                    className="p-4 bg-white/5 rounded-xl border border-white/10 text-center flex flex-col items-center justify-center transition-all hover:bg-white/10"
                  >
                    <p className="text-sm font-bold text-celestial-gold">
                      {info?.hangul || m.marker || "—"}
                    </p>
                    {info?.hanja && (
                      <p className="text-[10px] text-white/30 mt-1 font-medium">
                        {info.hanja}
                      </p>
                    )}
                  </div>
                );
              })}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
