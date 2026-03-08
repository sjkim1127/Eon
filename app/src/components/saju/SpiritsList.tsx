import { Shield } from "lucide-react";
import { SPIRIT_INFO, PILLAR_POS_INFO } from "../../constants";

interface Props {
  sp: any;
}

export function SpiritsList({ sp }: Props) {
  if (!sp?.markers || sp.markers.length === 0) return null;

  return (
    <div className="glass p-8 rounded-[2rem]">
      <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
        <Shield className="w-6 h-6 text-celestial-cyan" />
        신살 (神煞) 분석
      </h5>
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
        {sp.markers.map((m: any, i: number) => (
          <div
            key={i}
            className="p-4 bg-white/5 rounded-xl border border-white/10 text-center"
          >
            <p className="text-sm font-bold text-celestial-gold">{SPIRIT_INFO[m.marker]?.hangul || m.marker || "—"}</p>
            <p className="text-xs text-white/40 mt-1">{PILLAR_POS_INFO[m.position] || m.position || ""}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
