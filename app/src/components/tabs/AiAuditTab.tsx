import { motion } from "framer-motion";
import { Brain, TrendingUp, TrendingDown, FileCode, ExternalLink, Copy, Check } from "lucide-react";
import { useState } from "react";
import { toast } from "sonner";

export interface AiAuditReport {
  context_dump: string;
  peak_age: number;
  valley_age: number;
}

interface AiAuditTabProps {
  aiAuditReport: AiAuditReport | null;
}

const AI_STUDIO_URL = "https://aistudio.google.com/prompts/new_chat";

export function AiAuditTab({ aiAuditReport }: AiAuditTabProps) {
  const [copied, setCopied] = useState(false);

  const handleOpenAiStudio = async () => {
    if (!aiAuditReport?.context_dump) return;
    try {
      await navigator.clipboard.writeText(aiAuditReport.context_dump);
      setCopied(true);
      setTimeout(() => setCopied(false), 3000);
    } catch {
      toast.error("클립보드 복사에 실패했습니다. 아래 텍스트를 직접 복사해주세요.");
    }
    window.open(AI_STUDIO_URL, "_blank", "noopener,noreferrer");
  };

  const handleCopyOnly = async () => {
    if (!aiAuditReport?.context_dump) return;
    try {
      await navigator.clipboard.writeText(aiAuditReport.context_dump);
      setCopied(true);
      setTimeout(() => setCopied(false), 2500);
      toast.success("프롬프트가 클립보드에 복사됐습니다.");
    } catch {
      toast.error("클립보드 복사에 실패했습니다.");
    }
  };

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
        <h3 className="text-xl font-bold text-white mb-3">AI 분석 프롬프트</h3>
        <p className="text-white/60">
          출생 정보를 입력하고 통합 분석을 실행하면 AI 프롬프트가 생성됩니다.
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

      {/* Google AI Studio 연결 카드 */}
      <div className="glass p-8 rounded-[2rem] border-blue-500/20 bg-gradient-to-br from-blue-500/5 to-purple-500/5">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-6">
          <div>
            <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
              <Brain className="w-6 h-6 text-blue-400" />
              Google AI Studio에서 분석하기
            </h5>
            <p className="text-sm text-white/60 max-w-xl">
              운명 분석 컨텍스트를 클립보드에 복사하고 Google AI Studio를 엽니다.
              붙여넣기(<kbd className="px-1.5 py-0.5 rounded bg-white/10 text-xs font-mono">⌘V</kbd>) 후
              Gemini에게 분석을 요청하세요. <strong className="text-white/80">API 키 불필요.</strong>
            </p>
          </div>
          <div className="flex gap-3 shrink-0">
            <button
              onClick={handleCopyOnly}
              className="flex items-center gap-2 px-4 py-2.5 rounded-xl border border-white/20 bg-white/5 hover:bg-white/10 text-white text-sm font-medium transition-all"
            >
              {copied ? <Check className="w-4 h-4 text-green-400" /> : <Copy className="w-4 h-4" />}
              {copied ? "복사됨" : "복사만"}
            </button>
            <button
              onClick={handleOpenAiStudio}
              className="flex items-center gap-2 px-5 py-2.5 rounded-xl bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-400 hover:to-purple-500 text-white text-sm font-bold shadow-lg shadow-blue-500/20 transition-all hover:scale-[1.02] active:scale-[0.98]"
            >
              <ExternalLink className="w-4 h-4" />
              복사 후 AI Studio 열기
            </button>
          </div>
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
