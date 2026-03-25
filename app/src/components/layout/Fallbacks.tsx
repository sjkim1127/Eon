import { AlertCircle, Compass } from "lucide-react";
import { motion } from "framer-motion";

export function TabSkeleton() {
  return (
    <div className="glass p-8 rounded-[2rem] animate-pulse">
      <div className="h-4 w-40 bg-white/10 rounded mb-4" />
      <div className="h-24 bg-white/5 rounded-xl mb-3" />
      <div className="h-24 bg-white/5 rounded-xl mb-3" />
      <div className="h-24 bg-white/5 rounded-xl" />
    </div>
  );
}

export function UnavailableTabFallback({ reason }: { reason: string }) {
  return (
    <div className="h-[40vh] flex flex-col items-center justify-center text-center p-8 glass rounded-[2rem]">
      <AlertCircle className="w-12 h-12 text-white/20 mb-4" />
      <h3 className="text-xl font-semibold text-white mb-2">분석 결과 없음</h3>
      <p className="text-brand-400 max-w-sm">{reason}</p>
    </div>
  );
}

export function EmptyStateFallback() {
  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      className="h-[40vh] flex flex-col items-center justify-center text-center"
    >
      <div className="w-24 h-24 rounded-full bg-white/5 flex items-center justify-center mb-6">
        <Compass className="w-12 h-12 text-white/20 animate-pulse" />
      </div>
      <h3 className="text-2xl font-semibold text-white mb-2">활성화된 차트 없음</h3>
      <p className="text-brand-400 max-w-sm">
        출생 정보를 입력하고 통합 분석 시작 버튼을 눌러주세요.
      </p>
    </motion.div>
  );
}
