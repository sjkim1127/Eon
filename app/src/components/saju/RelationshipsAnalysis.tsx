import { Link2, Sparkles, AlertCircle, Layers } from "lucide-react";
import { REL_INFO, BRANCH_INFO } from "../../constants";
import { RelationshipAnalysis, RelationshipDetail } from "../../types/saju";

interface Props {
  relationships: RelationshipAnalysis;
}

export function RelationshipsAnalysis({ relationships }: Props) {
  if (!relationships) return null;

  const hasDetails = !!relationships?.mappedRelationships && relationships.mappedRelationships.length > 0;
  
  // 새 필드가 있으면 고해상도 카드 렌더링
  if (hasDetails) {
    return (
      <div className="glass p-8 rounded-[2rem] border border-white/10">
        <div className="flex items-center justify-between mb-8">
          <h5 className="text-2xl font-bold text-white flex items-center gap-3">
            <Link2 className="w-8 h-8 text-emerald-400" />
            합충형해 (合沖刑害) 분석
          </h5>
          <div className="flex gap-4 text-[10px] font-bold tracking-tighter uppercase">
            <span className="text-emerald-400/80 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20">Auspicious</span>
            <span className="text-rose-400/80 bg-rose-500/10 px-2 py-0.5 rounded border border-rose-500/20">Caution</span>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {relationships.mappedRelationships.map((rel: RelationshipDetail, i: number) => {
            const isAuspicious = rel.level === "Auspicious";
            return (
              <div
                key={i}
                className={`group relative p-6 rounded-2xl border transition-all duration-300 hover:scale-[1.01] ${
                  isAuspicious 
                    ? "bg-emerald-500/5 border-emerald-500/20 hover:bg-emerald-500/10 shadow-[0_0_20px_rgba(52,211,153,0.05)]" 
                    : "bg-rose-500/5 border-rose-500/20 hover:bg-rose-500/10 shadow-[0_0_20px_rgba(251,113,133,0.05)]"
                }`}
              >
                <div className="flex items-start justify-between mb-4">
                  <div className="flex-1">
                    <div className="flex items-center gap-2 mb-1">
                      <h6 className={`text-lg font-bold ${isAuspicious ? "text-emerald-400" : "text-rose-400"}`}>
                        {rel.name}
                      </h6>
                      <div className="flex gap-1">
                        {rel.positions.map((pos, idx) => (
                          <span key={idx} className="text-[10px] px-1.5 py-0.5 rounded bg-white/5 text-white/40 border border-white/5">
                            {pos}
                          </span>
                        ))}
                      </div>
                    </div>
                    <p className="text-sm font-bold text-white/80 leading-snug">
                      {rel.summary}
                    </p>
                  </div>
                  {isAuspicious ? (
                    <Sparkles className="w-5 h-5 text-emerald-400/40" />
                  ) : (
                    <AlertCircle className="w-5 h-5 text-rose-400/40" />
                  )}
                </div>

                <p className="text-xs text-white/50 mb-5 leading-relaxed font-medium">
                  {rel.description}
                </p>

                <div className="flex items-center justify-between mt-auto">
                  <div className="flex flex-wrap gap-1.5">
                    {rel.reasons.map((reason, idx) => (
                      <span
                        key={idx}
                        className="px-2 py-0.5 rounded-md bg-black/20 border border-white/5 text-[10px] text-white/40 font-semibold"
                      >
                        {reason}
                      </span>
                    ))}
                  </div>
                  {rel.transformedElement && (
                    <div className="flex items-center gap-1 px-2 py-0.5 rounded bg-white/10 border border-white/10">
                      <Layers className="w-3 h-3 text-white/40" />
                      <span className="text-[10px] text-white/60 font-bold">{rel.transformedElement}</span>
                    </div>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      </div>
    );
  }

  // 레거시 폴백 (하위 호환)
  const rel = relationships as any;
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
    { label: "천간합", color: "text-emerald-400 bg-emerald-500/15 border-emerald-500/30", items: (rel.stemCombinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "천간충", color: "text-red-400 bg-red-500/15 border-red-500/30", items: (rel.stemClashes ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "삼합", color: "text-amber-400 bg-amber-500/15 border-amber-500/30", items: (rel.tripleCombinations ?? []).map((r: any) => formatRel(r)) },
    { label: "방합", color: "text-amber-400 bg-amber-500/15 border-amber-500/30", items: (rel.seasonalCombinations ?? []).map((r: any) => formatRel(r)) },
    { label: "반합(진)", color: "text-yellow-400 bg-yellow-500/15 border-yellow-500/30", items: (rel.dominantSemiCombinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "육합", color: "text-green-400 bg-green-500/15 border-green-500/30", items: (rel.sixCombinations ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "지지충", color: "text-rose-400 bg-rose-500/15 border-rose-500/30", items: (rel.branchClashes ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "지지형", color: "text-orange-400 bg-orange-500/15 border-orange-500/30", items: (rel.branchPunishments ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "지지해", color: "text-pink-400 bg-pink-500/15 border-pink-500/30", items: (rel.branchHarms ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "지지파", color: "text-fuchsia-400 bg-fuchsia-500/15 border-fuchsia-500/30", items: (rel.branchDestructions ?? []).map((r: any) => `${formatRel(r[0])} (${r[1]}-${r[2]})`) },
    { label: "암합", color: "text-teal-400 bg-teal-500/15 border-teal-500/30", items: (rel.amCombinations ?? []).map((r: any) => `${formatRel(r[0]?.combination)} (${r[1]}-${r[2]})`) },
    { label: "명암합", color: "text-cyan-400 bg-cyan-500/15 border-cyan-500/30", items: (rel.myungAmCombinations ?? []).map((r: any) => `${formatRel(r[0]?.combination)} (${r[1]}-${r[2]})`) },
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
