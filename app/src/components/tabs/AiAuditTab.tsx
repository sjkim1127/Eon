import { motion } from "framer-motion";
import { Brain, TrendingUp, TrendingDown, FileCode } from "lucide-react";
import { isTauri } from "../../lib/backend";

export interface AiAuditReport {
  context_dump: string;
  peak_age: number;
  valley_age: number;
}

interface AiAuditTabProps {
  aiAuditReport: AiAuditReport | null;
}

export function AiAuditTab({ aiAuditReport }: AiAuditTabProps) {
  const isDesktop = isTauri();

  if (!isDesktop) {
    return (
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="glass p-12 rounded-[2rem] text-center"
      >
        <div className="w-20 h-20 rounded-full bg-white/5 flex items-center justify-center mx-auto mb-6">
          <Brain className="w-10 h-10 text-white/30" />
        </div>
        <h3 className="text-xl font-bold text-white mb-3">AI 감사 (데스크탑 전용)</h3>
        <p className="text-white/60 max-w-md mx-auto">
          AI 감사는 Tauri 데스크탑 앱에서만 사용 가능합니다. 웹 환경에서는 LLM 연동이 제한됩니다.
        </p>
        <p className="text-sm text-white/40 mt-4">
          <code className="px-2 py-1 rounded bg-white/10 font-mono">npm run tauri dev</code> 로 데스크탑 앱을 실행한 뒤 분석을 실행해보세요.
        </p>
      </motion.div>
    );
  }

  if (!aiAuditReport) {
    return (
      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        className="glass p-12 rounded-[2rem] text-center"
      >
        <div className="w-20 h-20 rounded-full bg-white/5 flex items-center justify-center mx-auto mb-6">
          <Brain className="w-10 h-10 text-white/30" />
        </div>
        <h3 className="text-xl font-bold text-white mb-3">AI 감사 결과</h3>
        <p className="text-white/60">
          출생 정보를 입력하고 통합 분석을 실행하면 AI 감사(코어덤프 형식 컨텍스트)가 여기에 표시됩니다.
        </p>
      </motion.div>
    );
  }

  const { context_dump, peak_age, valley_age } = aiAuditReport;

  return (
    <motion.div
      key="ai-audit-tab"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {/* Peak / Valley 요약 카드 */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="glass p-8 rounded-[2rem] border-celestial-gold/20 bg-celestial-gold/5">
          <p className="text-celestial-gold/80 text-sm font-bold uppercase tracking-wider mb-2 flex items-center gap-2">
            <TrendingUp className="w-4 h-4" />
            Peak Age (정점 나이)
          </p>
          <p className="text-4xl font-black text-white">{peak_age}세</p>
          <p className="text-sm text-white/50 mt-1">시뮬레이션 상 가장 유리한 시기</p>
        </div>
        <div className="glass p-8 rounded-[2rem] border-rose-500/20 bg-rose-500/5">
          <p className="text-rose-400/80 text-sm font-bold uppercase tracking-wider mb-2 flex items-center gap-2">
            <TrendingDown className="w-4 h-4" />
            Valley Age (저점 나이)
          </p>
          <p className="text-4xl font-black text-white">{valley_age}세</p>
          <p className="text-sm text-white/50 mt-1">시뮬레이션 상 주의가 필요한 시기</p>
        </div>
      </div>

      {/* 코어덤프 형식 컨텍스트 */}
      <div className="glass p-8 rounded-[2rem]">
        <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
          <FileCode className="w-6 h-6 text-celestial-cyan" />
          Destiny Context Dump (LLM 입력용)
        </h5>
        <pre className="p-5 rounded-xl bg-black/40 border border-white/10 font-mono text-xs text-white/90 whitespace-pre-wrap break-words overflow-x-auto max-h-[60vh] overflow-y-auto">
          {context_dump || "(없음)"}
        </pre>
      </div>
    </motion.div>
  );
}
