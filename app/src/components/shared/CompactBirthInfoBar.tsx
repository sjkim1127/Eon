import { Calendar, Pencil } from "lucide-react";
import type { BirthData } from "../../types";

interface CompactBirthInfoBarProps {
    birthData: BirthData;
    selectedCity: string;
    isMale: boolean;
    isDST: boolean;
    onEdit: () => void;
    /** 복사 버튼 슬롯 — ExportActionButtons를 여기에 전달합니다 */
    actionSlot?: React.ReactNode;
}

const pad = (n: number) => String(n).padStart(2, "0");

/**
 * 분석 완료 후 헤더에 표시되는 출생 정보 요약 바.
 *
 * 출생 날짜/시간, 도시, 성별, DST 여부를 한 줄로 표시하고
 * '수정' 버튼과 내보내기 액션 버튼(actionSlot)을 우측에 배치합니다.
 */
export function CompactBirthInfoBar({
    birthData,
    selectedCity,
    isMale,
    isDST,
    onEdit,
    actionSlot,
}: CompactBirthInfoBarProps) {
    return (
        <div
            className="flex items-center gap-3 px-4 py-2.5 rounded-2xl mb-5"
            style={{ background: "rgba(255,255,255,0.04)", border: "1px solid rgba(255,255,255,0.08)" }}
        >
            <Calendar className="w-4 h-4 text-celestial-purple shrink-0" />
            <div className="flex items-center gap-2 text-sm text-white/60 flex-1 flex-wrap">
                <span className="font-mono text-white/80">
                    {birthData.year}.{pad(birthData.month)}.{pad(birthData.day)}
                </span>
                <span className="text-white/25">·</span>
                {birthData.unknown_time ? (
                    <span className="text-amber-400/70 text-xs">시간미상</span>
                ) : (
                    <span className="font-mono">{pad(birthData.hour)}:{pad(birthData.minute)}</span>
                )}
                <span className="text-white/25">·</span>
                <span>{selectedCity}</span>
                <span className="text-white/25">·</span>
                <span className={isMale ? "text-celestial-cyan" : "text-pink-400"}>
                    {isMale ? "남" : "여"}
                </span>
                {isDST && (
                    <span className="text-[10px] px-2 py-0.5 rounded-full bg-amber-500/20 text-amber-400 border border-amber-500/30">
                        DST
                    </span>
                )}
            </div>
            <button
                onClick={onEdit}
                className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 text-white/50 hover:text-white transition-all font-medium"
            >
                <Pencil className="w-3 h-3" />
                수정
            </button>
            {actionSlot && <div className="flex items-center gap-2">{actionSlot}</div>}
        </div>
    );
}
