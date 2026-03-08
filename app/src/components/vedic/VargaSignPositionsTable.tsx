import { SIGN_NAMES, VARGA_DEFS } from "../../constants";

interface Props {
  planets: any[];
  ascendant: any;
}

export function VargaSignPositionsTable({ planets, ascendant }: Props) {
  return (
    <div className="overflow-x-auto">
      <table className="text-xs">
        <thead>
          <tr className="border-b border-white/10">
            <th className="text-left text-white/40 font-bold uppercase tracking-wider pb-3 pr-3 whitespace-nowrap">행성</th>
            {VARGA_DEFS.map(v => (
              <th key={v.id} className="text-center text-white/40 font-bold pb-3 px-2 whitespace-nowrap">
                <span className="text-white/70">{v.label}</span>
              </th>
            ))}
          </tr>
          <tr className="border-b border-white/5">
            <th className="pb-2 pr-3"></th>
            {VARGA_DEFS.map(v => (
              <th key={v.id} className="text-center text-white/25 font-normal pb-2 px-2 whitespace-nowrap text-[10px]">
                {v.name.substring(0, 8)}
              </th>
            ))}
          </tr>
        </thead>
        <tbody className="divide-y divide-white/5">
          {[
            ...planets.map((p: any) => ({ name: p.planet, data: p, retro: p.is_retrograde, combust: p.is_combust })),
            ...(ascendant ? [{ name: "ASC", data: ascendant, retro: false, combust: false }] : []),
          ].map((row, i) => (
            <tr key={i} className="hover:bg-white/3 transition-colors">
              <td className="py-2 pr-3 font-bold text-white whitespace-nowrap">
                {row.name}
                {row.retro && <span className="ml-1 text-[9px] px-1 rounded bg-amber-500/20 text-amber-400">℞</span>}
                {row.combust && <span className="ml-0.5 text-[9px] px-1 rounded bg-orange-500/20 text-orange-400">☀</span>}
              </td>
              {VARGA_DEFS.map(v => {
                const signNum: number = row.data?.[v.key];
                const signName = SIGN_NAMES[signNum] ?? "—";
                return (
                  <td key={v.id} className="py-2 px-2 text-center whitespace-nowrap" title={`${row.name} in ${v.name}: ${signName}`}>
                    <span className="inline-block min-w-[24px] text-white/70 font-mono">
                      {signNum ?? "—"}
                    </span>
                    <br />
                    <span className="text-[9px] text-white/30">{signName.substring(0, 3)}</span>
                  </td>
                );
              })}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
