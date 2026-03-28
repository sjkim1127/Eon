import { BarChart3, Grid3x3 } from "lucide-react";
import { BavHeatmap } from "../../vedic/BavHeatmap";
import { SavScoreChart } from "../../vedic/SavScoreChart";
import { VimshopakaTable } from "../../vedic/VimshopakaTable";

interface VedicMetricsGridProps {
    bav: any[];
    savPoints: number[];
    vimshopaka: any[];
}

export function VedicMetricsGrid({ bav, savPoints, vimshopaka }: VedicMetricsGridProps) {
    return (
        <div className="space-y-8">
            {/* ── BAV Heatmap ────────────── */}
            {bav && bav.length > 0 && (
                <div className="glass p-8 rounded-[2rem]">
                    <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
                        <BarChart3 className="w-6 h-6 text-celestial-gold" />
                        빈나슈타카바르가 (BAV) — 행성×하우스 빈두 히트맵
                    </h5>
                    <p className="text-xs text-white/40 mb-6">
                        각 행성이 12개 하우스에 기여하는 빈두 포인트입니다. 녹색에 가까울수록 강한 하우스를 지원합니다.
                    </p>
                    <BavHeatmap bav={bav} savPoints={savPoints} />
                </div>
            )}

            {/* ── SAV Chart ────────────── */}
            {savPoints && savPoints.length === 12 && (
                <div className="glass p-8 rounded-[2rem]">
                    <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                        <Grid3x3 className="w-6 h-6 text-celestial-cyan" />
                        사르바아슈타카바르가 (SAV) — 12하우스 합산 점수
                    </h5>
                    <SavScoreChart points={savPoints} />
                </div>
            )}

            {/* ── Vimshopaka Table ────────────── */}
            {vimshopaka && vimshopaka.length > 0 && (
                <VimshopakaTable vimshopaka={vimshopaka} />
            )}
        </div>
    );
}
