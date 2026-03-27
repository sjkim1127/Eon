import { motion } from "framer-motion";
import { Info, HelpCircle } from "lucide-react";
import { DestinyComponent } from "../../types/analysis";

interface Props {
  components: DestinyComponent[];
}

export function HighFidelityBreakdown({ components }: Props) {
  if (!components || components.length === 0) return null;

  return (
    <div className="glass p-8 rounded-[2rem] border border-white/10 overflow-hidden relative">
      <div className="absolute top-0 right-0 p-8 opacity-10">
        <Info className="w-24 h-24 text-white" />
      </div>
      
      <div className="relative z-10">
        <div className="flex items-center gap-3 mb-8">
          <div className="p-3 rounded-2xl bg-celestial-purple/20 border border-celestial-purple/30">
            <HelpCircle className="w-6 h-6 text-celestial-purple" />
          </div>
          <div>
            <h3 className="text-2xl font-bold text-white">12-컴포넌트 정밀 분석</h3>
            <p className="text-white/40 text-sm">운명 티어를 결정짓는 12가지 핵심 지표의 상세 득점 현황입니다.</p>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {components.map((comp, i) => (
            <motion.div
              key={comp.key}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: i * 0.05 }}
              className="p-5 rounded-2xl bg-white/5 border border-white/10 hover:bg-white/10 transition-colors group"
            >
              <div className="flex justify-between items-start mb-3">
                <div>
                  <p className="text-white/40 text-[10px] uppercase tracking-wider font-bold mb-1">{comp.key.replace(/_/g, ' ')}</p>
                  <h4 className="text-white font-bold">{comp.label}</h4>
                </div>
                <div className="text-right">
                  <span className="text-xl font-black text-celestial-cyan">{Math.round(comp.score)}</span>
                  <span className="text-[10px] text-white/30 ml-1">/100</span>
                </div>
              </div>
              
              <div className="h-1.5 w-full bg-white/10 rounded-full overflow-hidden mb-3">
                <motion.div 
                  initial={{ width: 0 }}
                  animate={{ width: `${comp.score}%` }}
                  transition={{ duration: 1, delay: i * 0.1 }}
                  className="h-full bg-gradient-to-r from-celestial-purple to-celestial-cyan"
                />
              </div>

              <div className="space-y-1">
                {comp.reasons.map((reason, idx) => (
                  <p key={idx} className="text-[11px] text-white/50 leading-tight flex gap-1.5">
                    <span className="text-celestial-cyan/60">•</span>
                    {reason}
                  </p>
                ))}
              </div>
              
              <div className="mt-3 pt-3 border-t border-white/5 flex justify-between items-center opacity-0 group-hover:opacity-100 transition-opacity">
                <span className="text-[10px] text-white/30 text-xs">가중치: {(comp.weight * 100).toFixed(0)}%</span>
                <span className="text-[10px] text-celestial-purple font-bold">기여도: {Math.round(comp.score * comp.weight)}pt</span>
              </div>
            </motion.div>
          ))}
        </div>
      </div>
    </div>
  );
}
