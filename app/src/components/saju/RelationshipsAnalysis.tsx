import { Link2 } from "lucide-react";
import { REL_INFO, BRANCH_INFO } from "../../constants";

interface Props {
  relationships: any;
}

export function RelationshipsAnalysis({ relationships }: Props) {
  if (!relationships) return null;

  const rel = relationships as Record<string, unknown[] | undefined>;
  type RelGroup = { label: string; color: string; items: string[] };

  const formatRel = (r: any) => {
    if (typeof r === "string") return REL_INFO[r] || r;
    if (r && typeof r === "object") {
      if (r.SelfPunishment) return `${BRANCH_INFO[r.SelfPunishment]?.hangul || r.SelfPunishment}자형`;
      if (r.hangul) return r.hangul;
    }
    return JSON.stringify(r);
  };

  const groups: RelGroup[] = [
    { label: "천간합", color: "text-emerald-400 bg-emerald-500/15 border-emerald-500/30", items: (rel.stem_combinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "천간충", color: "text-red-400 bg-red-500/15 border-red-500/30", items: (rel.stem_clashes ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "삼합", color: "text-amber-400 bg-amber-500/15 border-amber-500/30", items: (rel.triple_combinations ?? []).map((r: any) => formatRel(r)) },
    { label: "방합", color: "text-amber-400 bg-amber-500/15 border-amber-500/30", items: (rel.seasonal_combinations ?? []).map((r: any) => formatRel(r)) },
    { label: "반합(진)", color: "text-yellow-400 bg-yellow-500/15 border-yellow-500/30", items: (rel.dominant_semi_combinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "육합", color: "text-green-400 bg-green-500/15 border-green-500/30", items: (rel.six_combinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "지지충", color: "text-rose-400 bg-rose-500/15 border-rose-500/30", items: (rel.branch_clashes ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "지지형", color: "text-orange-400 bg-orange-500/15 border-orange-500/30", items: (rel.branch_punishments ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "지지해", color: "text-pink-400 bg-pink-500/15 border-pink-500/30", items: (rel.branch_harms ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "지지파", color: "text-fuchsia-400 bg-fuchsia-500/15 border-fuchsia-500/30", items: (rel.branch_destructions ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "암합", color: "text-teal-400 bg-teal-500/15 border-teal-500/30", items: (rel.am_combinations ?? []).map((r: any) => `${formatRel(r[0]?.combination)} (${r[1]}-${r[2]})`) },
    { label: "명암합", color: "text-cyan-400 bg-cyan-500/15 border-cyan-500/30", items: (rel.myung_am_combinations ?? []).map((r: any) => `${formatRel(r[0]?.combination)} (${r[1]}-${r[2]})`) },
  ].filter(g => g.items.length > 0);

  if (groups.length === 0) return null;
  return (
    <div className="glass p-8 rounded-[2rem]">
      <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
        <Link2 className="w-6 h-6 text-emerald-400" />
        합충형해 (合沖刑害) 분석
      </h5>
      <div className="space-y-4">
        {groups.map((g) => (
          <div key={g.label}>
            <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">{g.label}</p>
            <div className="flex flex-wrap gap-2">
              {g.items.map((item: string, j: number) => (
                <span key={j} className={`text-xs px-3 py-1.5 rounded-full border font-semibold ${g.color}`}>
                  {item}
                </span>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
