import type { VargaNakshatraReportRow } from "../../../types/vedic";
import { SIGN_NAMES } from "../../../constants";
import { NAKSHATRA_TABLE_COLUMNS } from "../../../utils/vedicFormat";

interface VargaNakshatraTableProps {
  title: string;
  vargaLabel: string;
  rows: VargaNakshatraReportRow[];
  showHouse?: boolean;
  onCopyText?: () => void;
  copied?: boolean;
}

/** 재사용 가능한 바르가 낙샤트라 상세 테이블 (D1과 동일한 8컬럼) */
export function VargaNakshatraTable({
  title,
  vargaLabel,
  rows,
  showHouse = true,
  onCopyText,
  copied = false,
}: VargaNakshatraTableProps) {
  const purposeColor = (purpose: string) =>
    purpose === "Dharma"
      ? "text-celestial-gold"
      : purpose === "Artha"
        ? "text-green-400"
        : purpose === "Kama"
          ? "text-pink-400"
          : "text-blue-400";

  const columns: string[] = [
    NAKSHATRA_TABLE_COLUMNS.planet,
    NAKSHATRA_TABLE_COLUMNS.position,
    NAKSHATRA_TABLE_COLUMNS.nakshatra,
    NAKSHATRA_TABLE_COLUMNS.padaRange,
    NAKSHATRA_TABLE_COLUMNS.nakshatraLord,
    NAKSHATRA_TABLE_COLUMNS.padaLord,
    NAKSHATRA_TABLE_COLUMNS.deity,
    NAKSHATRA_TABLE_COLUMNS.purpose,
  ];

  if (showHouse) {
    columns.splice(2, 0, `${vargaLabel} 사인`, `${vargaLabel} 하우스`);
  }

  return (
    <div className="space-y-4">
      {(title || onCopyText) && (
        <div className="flex items-center justify-between">
          {title && <h6 className="text-lg font-bold text-white">{title}</h6>}
          {onCopyText && (
          <button
            type="button"
            onClick={onCopyText}
            className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-white/10 hover:bg-white/20 border border-white/20 text-xs text-white font-semibold"
          >
            {copied ? "복사됨!" : "복사"}
          </button>
        )}
        </div>
      )}
      <div className="overflow-x-auto">
        <table className="w-full text-sm">
          <thead>
            <tr className="border-b border-white/10">
              {(columns || []).map((h) => (
                <th
                  key={h}
                  className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4 whitespace-nowrap"
                >
                  {h}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y divide-white/5">
            {(rows || []).map((row, i) => {
              const isRetrograde = row.isRetrograde ?? false;
              const isCombust = row.isCombust ?? false;
              const positionStr = row.positionStr ?? "—";
              const nakshatraName = row.nakshatraName ?? "—";
              const padaRange = row.padaRange ?? "—";
              const nakshatraLord = row.nakshatraLord ?? "—";
              const padaLord = row.padaLord ?? "—";
              
              return (
                <tr key={i} className="hover:bg-white/3 transition-colors">
                  <td className="py-3 pr-4 font-bold text-white whitespace-nowrap">
                    {row.planet}
                    {isRetrograde && (
                      <span className="ml-1.5 text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/40">
                        ℞
                      </span>
                    )}
                    {isCombust && (
                      <span className="ml-1 text-[10px] px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 border border-orange-500/40">
                        ☀
                      </span>
                    )}
                  </td>
                  <td className="py-3 pr-4 text-white/70 font-mono text-xs whitespace-nowrap">
                    {positionStr}
                  </td>
                  {showHouse && (
                    <>
                      <td className="py-3 pr-4 text-celestial-cyan font-semibold whitespace-nowrap">
                        {SIGN_NAMES[row.sign] ?? "—"}
                      </td>
                      <td className="py-3 pr-4 text-white/70 whitespace-nowrap">
                        <span className="px-2 py-0.5 rounded bg-white/10 font-mono text-xs">
                          H{row.house}
                        </span>
                      </td>
                    </>
                  )}
                  <td className="py-3 pr-4 text-celestial-cyan font-semibold whitespace-nowrap">
                    {nakshatraName}
                    <span className="ml-1.5 text-[10px] text-white/40">(Pada {row.pada})</span>
                  </td>
                  <td className="py-3 pr-4 text-white/40 text-xs whitespace-nowrap">
                    {padaRange}
                  </td>
                  <td className="py-3 pr-4 text-white/70 whitespace-nowrap">{nakshatraLord}</td>
                  <td className="py-3 pr-4 text-white/70 whitespace-nowrap">{padaLord}</td>
                  <td className="py-3 pr-4 text-white/60 whitespace-nowrap">{row.deity}</td>
                  <td className="py-3 pr-4 whitespace-nowrap">
                    <span className={`text-xs font-bold ${purposeColor(row.purpose)}`}>
                      {row.purpose}
                    </span>
                  </td>
                </tr>
              );
            })}
          </tbody>
        </table>
      </div>
    </div>
  );
}
