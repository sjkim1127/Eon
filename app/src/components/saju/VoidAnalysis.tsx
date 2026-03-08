import { CircleOff } from "lucide-react";
import { BRANCH_INFO, TENGOD_INFO } from "../../constants";

interface Props {
  voidAnalysis: any;
}

export function VoidAnalysis({ voidAnalysis }: Props) {
  if (!voidAnalysis) return null;

  const branches: any[] = voidAnalysis.void_branches ?? [];
  const positions: string[] = voidAnalysis.void_positions ?? [];
  const tenGods: any[] = voidAnalysis.void_ten_gods ?? [];
  const xunGroup: string = voidAnalysis.xun_group ?? "";

  return (
    <div className="glass p-8 rounded-[2rem] border-violet-500/20 bg-violet-500/5">
      <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
        <CircleOff className="w-6 h-6 text-violet-400" />
        공망 (空亡) 분석
      </h5>
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
        <div>
          <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">공망 지지</p>
          <div className="flex gap-3">
            {branches.map((b: string, i: number) => {
              const info = BRANCH_INFO[b];
              return (
                <div key={i} className="px-5 py-3 rounded-xl bg-violet-500/15 border border-violet-500/30 text-center">
                  <p className="text-2xl font-black text-violet-300">{info?.hanja ?? b}</p>
                  <p className="text-xs text-violet-400/70 mt-1">{info?.hangul ?? ""}</p>
                </div>
              );
            })}
          </div>
          {xunGroup && (
            <p className="text-xs text-white/40 mt-3">순(旬) 그룹: <span className="text-violet-300 font-semibold">{xunGroup}</span></p>
          )}
        </div>
        <div>
          {positions.length > 0 ? (
            <>
              <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">원국 내 공망 발생</p>
              <div className="space-y-2">
                {positions.map((pos: string, i: number) => (
                  <div key={i} className="flex items-center gap-3 text-sm">
                    <span className="px-2.5 py-1 rounded-lg bg-violet-500/20 border border-violet-500/30 text-violet-300 font-bold">{pos}</span>
                    <span className="text-white/60">→</span>
                    <span className="text-white font-semibold">{TENGOD_INFO[tenGods[i]]?.hangul ?? tenGods[i] ?? ""}</span>
                    <span className="text-white/30 text-xs">공망</span>
                  </div>
                ))}
              </div>
            </>
          ) : (
            <div className="flex items-center h-full">
              <p className="text-sm text-white/40">원국 내에 공망이 없습니다.</p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
