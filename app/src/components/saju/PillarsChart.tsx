import { STEM_INFO, BRANCH_INFO, ELEMENT_INFO, STEM_TO_ELEMENT, BRANCH_TO_ELEMENT, TENGOD_INFO } from "../../constants";

interface Props {
  p: any;
  t: any;
  unknownTime: boolean;
}

export function PillarsChart({ p, t, unknownTime }: Props) {
  return (
    <div className="grid gap-2 md:gap-4 grid-cols-3 md:grid-cols-4">
      {[
        { label: "시주", pillar: p?.hour, isHour: true, tStem: t?.hour_stem, tBranch: t?.hour_branch },
        { label: "일주", pillar: p?.day, isHour: false, tStem: t?.day_stem, tBranch: t?.day_branch },
        { label: "월주", pillar: p?.month, isHour: false, tStem: t?.month_stem, tBranch: t?.month_branch },
        { label: "년주", pillar: p?.year, isHour: false, tStem: t?.year_stem, tBranch: t?.year_branch },
      ]
        .filter(({ isHour }) => !(isHour && unknownTime))
        .map(({ label, pillar, tStem, tBranch }) => {
          const stemEleKey = STEM_TO_ELEMENT[pillar?.stem] ?? "";
          const branchEleKey = BRANCH_TO_ELEMENT[pillar?.branch] ?? "";

          return (
            <div key={label} className="flex flex-col text-center p-3 rounded-2xl border transition-all bg-white/5 border-white/10">
              <p className="text-[11px] text-white/40 font-bold uppercase tracking-wider mb-2 flex items-center justify-center gap-1.5">
                {label}
              </p>

              {/* 천간 영역 */}
              <div className="flex-1 flex flex-col justify-center py-2 border-b border-white/5 relative group">
                <p className="text-[11px] text-white/50 mb-1.5 font-semibold">
                  {TENGOD_INFO[tStem]?.hangul || tStem || "—"}
                </p>
                <p className="text-2xl sm:text-3xl font-black text-celestial-gold mb-1.5 leading-none">
                  {STEM_INFO[pillar?.stem]?.hanja || "—"}
                </p>
                <p className="text-xs text-white/80 flex items-center justify-center gap-1">
                  {STEM_INFO[pillar?.stem]?.hangul || ""}
                  <span className="text-[10px] text-white/40 border border-white/10 px-1 py-0.5 rounded-sm">
                    {ELEMENT_INFO[stemEleKey]?.hangul || ""}
                  </span>
                </p>
              </div>

              {/* 지지 영역 */}
              <div className="flex-1 flex flex-col justify-center py-2 relative group mt-1">
                <p className="text-2xl sm:text-3xl font-black text-celestial-cyan mb-1.5 leading-none">
                  {BRANCH_INFO[pillar?.branch]?.hanja || "—"}
                </p>
                <p className="text-xs text-white/80 flex items-center justify-center gap-1 mb-1.5">
                  {BRANCH_INFO[pillar?.branch]?.hangul || ""}
                  <span className="text-[10px] text-white/40 border border-white/10 px-1 py-0.5 rounded-sm">
                    {ELEMENT_INFO[branchEleKey]?.hangul || ""}
                  </span>
                </p>
                <p className="text-[11px] text-white/50 font-semibold">
                  {TENGOD_INFO[tBranch]?.hangul || tBranch || "—"}
                </p>
              </div>
            </div>
          );
        })}
    </div>
  );
}
