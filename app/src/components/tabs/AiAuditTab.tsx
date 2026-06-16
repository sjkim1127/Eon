import { motion } from "framer-motion";
import { AiAuditSection } from "../sections/ai/AiAuditSection";

interface AiAuditTabProps {
  sajuReport: any;
  birthYear?: number;
  birthMonth?: number;
  birthDay?: number;
  birthHour?: number;
  isMale?: boolean;
}

export function AiAuditTab({
  sajuReport,
  birthYear,
  birthMonth,
  birthDay,
  birthHour,
  isMale,
}: AiAuditTabProps) {
  return (
    <motion.div
      key="ai-audit-tab"
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      exit={{ opacity: 0, y: -20 }}
      className="space-y-8"
    >
      {birthYear && birthMonth && birthDay !== undefined && birthHour !== undefined ? (
        <AiAuditSection
          sajuReport={sajuReport}
          birthYear={birthYear}
          birthMonth={birthMonth}
          birthDay={birthDay}
          birthHour={birthHour}
          isMale={isMale ?? true}
        />
      ) : (
        <div className="glass p-8 rounded-[2rem] text-center text-white/50">
          생년월일시 정보가 올바르지 않거나 아직 분석되지 않았습니다.
        </div>
      )}
    </motion.div>
  );
}
