import { CircleOff, AlertCircle, Info, Sparkles } from "lucide-react";
import { BRANCH_INFO, TENGOD_INFO } from "../../constants";
import { VoidAnalysis as VoidType, VoidDetail } from "../../types/saju";

interface Props {
  voidAnalysis: VoidType;
}

export function VoidAnalysis({ voidAnalysis }: Props) {
  if (!voidAnalysis) return null;

  const branches = voidAnalysis.voidBranches ?? [];
  const xunGroup = voidAnalysis.xunGroup ?? "";
  const hasDetails = !!voidAnalysis.mappedVoids && voidAnalysis.mappedVoids.length > 0;

  return (
    <div className="glass p-8 rounded-[2rem] border-violet-500/20 bg-violet-500/5">
      <div className="flex items-center justify-between mb-8">
        <h5 className="text-2xl font-bold text-white flex items-center gap-3">
          <CircleOff className="w-8 h-8 text-violet-400" />
          공망 (空亡) 분석
        </h5>
        <div className="flex gap-2 text-[10px] font-bold tracking-tighter uppercase text-violet-400/80 bg-violet-500/10 px-3 py-1 rounded border border-violet-500/20">
          Void Analysis
        </div>
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-8">
        <div className="lg:col-span-1 p-6 rounded-2xl bg-black/20 border border-white/5 h-fit">
          <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-4">공망 지지 및 순(旬)</p>
          <div className="flex gap-4 mb-4">
            {branches.map((b: string, i: number) => {
              const info = BRANCH_INFO[b];
              return (
                <div key={i} className="flex-1 px-4 py-3 rounded-xl bg-violet-500/10 border border-violet-500/20 text-center shadow-inner">
                  <p className="text-3xl font-black text-violet-300">{info?.hanja ?? b}</p>
                  <p className="text-[10px] text-violet-400/70 mt-1 font-bold">{info?.hangul ?? ""}</p>
                </div>
              );
            })}
          </div>
          {xunGroup && (
            <div className="flex items-center gap-2 px-3 py-2 rounded-lg bg-white/5 border border-white/5">
              <Info className="w-3.5 h-3.5 text-violet-400/60" />
              <p className="text-xs text-white/50 font-medium">순(旬) 그룹: <span className="text-violet-300 font-bold">{xunGroup}</span></p>
            </div>
          )}
        </div>

        <div className="lg:col-span-2">
          {hasDetails ? (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {voidAnalysis.mappedVoids.map((v: VoidDetail, i: number) => (
                <div
                  key={i}
                  className="group relative p-5 rounded-2xl bg-violet-500/5 border border-violet-500/20 transition-all duration-300 hover:bg-violet-500/10 shadow-[0_0_20px_rgba(167,139,250,0.05)]"
                >
                  <div className="flex items-start justify-between mb-3">
                    <div className="flex-1">
                      <div className="flex items-center gap-2 mb-1">
                        <h6 className="text-base font-bold text-violet-300">
                          {v.position} {TENGOD_INFO[v.tenGod]?.hangul ?? v.tenGod}
                        </h6>
                        <span className="text-[10px] px-1.5 py-0.5 rounded bg-violet-500/20 text-violet-300 border border-violet-500/20 font-bold uppercase">
                          Void
                        </span>
                      </div>
                      <p className="text-xs font-bold text-white/80 leading-snug">
                        {v.summary}
                      </p>
                    </div>
                    <AlertCircle className="w-5 h-5 text-violet-400/40" />
                  </div>

                  <p className="text-[11px] text-white/50 mb-4 leading-relaxed font-medium">
                    {v.description}
                  </p>

                  <div className="flex flex-wrap gap-1.5 mt-auto">
                    {v.reasons.map((reason, idx) => (
                      <span
                        key={idx}
                        className="px-2 py-0.5 rounded-md bg-black/30 border border-white/5 text-[9px] text-white/40 font-semibold"
                      >
                        {reason}
                      </span>
                    ))}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center h-full min-h-[160px] p-8 rounded-2xl bg-black/10 border border-dashed border-white/10">
              <Sparkles className="w-8 h-8 text-white/10 mb-3" />
              <p className="text-sm text-white/30 font-bold tracking-tight text-center">원국 내에 공망이 없습니다.</p>
              <p className="text-[10px] text-white/20 mt-1 uppercase font-black">No Void Detected in Natal Chart</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
