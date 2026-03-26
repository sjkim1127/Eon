import { useState } from "react";
import { ClipboardCopy, Check } from "lucide-react";
import { toast } from "sonner";
import {
    buildFullAnalysisMarkdown,
    buildSajuMarkdown,
    buildVedicMarkdown,
} from "../../utils";
import type { SajuAnalysisResult, VedicAnalysisResult, TransitResult } from "../../types";

import { TierResult } from "../../utils/tierScore";

interface ExportActionButtonsProps {
    sajuReport: SajuAnalysisResult | null;
    report: VedicAnalysisResult | null;
    transitReport: TransitResult | null;
    tierResult?: TierResult | null;
}

/** 클립보드에 텍스트를 복사하고 실패 시 toast.error를 표시합니다. */
async function copyToClipboard(text: string, errorMsg = "클립보드 복사에 실패했습니다."): Promise<boolean> {
    try {
        await navigator.clipboard.writeText(text);
        return true;
    } catch {
        toast.error(errorMsg);
        return false;
    }
}

/**
 * 마크다운 내보내기 버튼 그룹.
 *
 * 전체 / 사주 / 베딕 분석 결과를 클립보드로 복사합니다.
 * 각 버튼은 독립된 복사 완료 상태를 관리하며, 클립보드 API 오류 발생 시
 * sonner toast로 사용자에게 알립니다.
 */
export function ExportActionButtons({
    sajuReport,
    report,
    transitReport,
    tierResult,
}: ExportActionButtonsProps) {
    const [mdCopied, setMdCopied] = useState(false);
    const [sajuCopied, setSajuCopied] = useState(false);
    const [vedicCopied, setVedicCopied] = useState(false);

    const handleCopyAll = async () => {
        const md = buildFullAnalysisMarkdown(sajuReport ?? null, report ?? null, transitReport ?? null, tierResult);
        const ok = await copyToClipboard(md);
        if (ok) {
            setMdCopied(true);
            setTimeout(() => setMdCopied(false), 2500);
        }
    };

    const handleCopySaju = async () => {
        if (!sajuReport) return;
        const md = buildSajuMarkdown(sajuReport);
        const ok = await copyToClipboard(md);
        if (ok) {
            setSajuCopied(true);
            setTimeout(() => setSajuCopied(false), 2500);
        }
    };

    const handleCopyVedic = async () => {
        if (!report) return;
        const md = buildVedicMarkdown(report);
        const ok = await copyToClipboard(md);
        if (ok) {
            setVedicCopied(true);
            setTimeout(() => setVedicCopied(false), 2500);
        }
    };

    return (
        <>
            {/* 전체 복사 */}
            <button
                onClick={handleCopyAll}
                className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 text-white/50 hover:text-white transition-all font-medium"
            >
                {mdCopied ? <Check className="w-3 h-3 text-green-400" /> : <ClipboardCopy className="w-3 h-3" />}
                {mdCopied ? "복사됨!" : "전체 복사"}
            </button>

            {/* 사주 복사 */}
            {sajuReport && (
                <button
                    onClick={handleCopySaju}
                    className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-brand-500/20 bg-brand-500/10 hover:bg-brand-500/20 text-brand-300 transition-all font-medium"
                >
                    {sajuCopied ? <Check className="w-3 h-3 text-green-400" /> : <ClipboardCopy className="w-3 h-3" />}
                    {sajuCopied ? "사주 복사됨!" : "사주 복사"}
                </button>
            )}

            {/* 베딕 복사 */}
            {report && (
                <button
                    onClick={handleCopyVedic}
                    className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-celestial-purple/30 bg-celestial-purple/10 hover:bg-celestial-purple/20 text-indigo-300 transition-all font-medium"
                >
                    {vedicCopied ? <Check className="w-3 h-3 text-green-400" /> : <ClipboardCopy className="w-3 h-3" />}
                    {vedicCopied ? "베딕 복사됨!" : "베딕 복사"}
                </button>
            )}

        </>
    );
}
