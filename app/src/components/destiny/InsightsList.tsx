import { Flame } from "lucide-react";
import { TierResult } from "../../utils/tierScore";
import { buildInsightBlocks } from "./destinyUtils";

interface Props {
  result: TierResult;
}

export function InsightsList({ result }: Props) {
  const blocks = buildInsightBlocks(result);

  return (
    <div className="glass p-6 rounded-2xl">
      <h5 className="text-lg font-bold text-white mb-5 flex items-center gap-2">
        <Flame className="w-5 h-5 text-celestial-gold" />
        종합 인사이트
      </h5>
      <div className="space-y-4">
        {blocks.map((block, i) => (
          <div key={i} className="flex gap-3 p-3.5 rounded-xl bg-white/5 border border-white/8">
            <span className="text-xl shrink-0 mt-0.5">{block.icon}</span>
            <div>
              <p className={`text-xs font-semibold mb-1 ${block.color}`}>{block.title}</p>
              <p className="text-sm text-white/80 leading-relaxed">{block.text}</p>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
