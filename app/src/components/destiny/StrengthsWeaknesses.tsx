import { TrendingUp, AlertTriangle, ChevronRight } from "lucide-react";

interface Props {
  strengths: string[];
  weaknesses: string[];
}

export function StrengthsWeaknesses({ strengths, weaknesses }: Props) {
  return (
    <div className="glass p-6 rounded-2xl">
      <h5 className="text-lg font-bold text-white mb-4 flex items-center gap-2">
        <TrendingUp className="w-5 h-5 text-celestial-gold" />
        강점 · 약점
      </h5>
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div>
          <p className="text-xs text-white/40 font-bold uppercase mb-2">강점</p>
          <ul className="space-y-1.5">
            {strengths.length > 0 ? strengths.map((s, i) => (
              <li key={i} className="text-sm text-emerald-300 flex items-center gap-2">
                <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 shrink-0" />
                {s}
              </li>
            )) : (
              <li className="text-sm text-white/40">—</li>
            )}
          </ul>
        </div>
        <div>
          <p className="text-xs text-white/40 font-bold uppercase mb-2">약점 (참고)</p>
          <ul className="space-y-1.5">
            {weaknesses.length > 0 ? weaknesses.map((w, i) => (
              <li key={i} className="text-sm text-amber-300 flex items-center gap-2">
                <AlertTriangle className="w-3 h-3 text-amber-400 shrink-0" />
                {w}
              </li>
            )) : (
              <li className="text-sm text-white/40">특이사항 없음</li>
            )}
          </ul>
        </div>
      </div>
      <p className="text-xs text-white/30 mt-4 flex items-center gap-1">
        <ChevronRight className="w-3 h-3" />
        역량 및 기운 탭에서 주의 시점, 현재 운세 탭에서 세운·월운을 확인하세요.
      </p>
    </div>
  );
}
