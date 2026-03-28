import { useState } from "react";
import type { VedicPosition, VargaNakshatraReport } from "../../../types/vedic";
import { Grid3x3 } from "lucide-react";
import { VARGA_DEFS } from "../../../constants";
import { SouthIndianChart } from "../../charts/SouthIndianChart";
import { NorthIndianChart } from "../../charts/NorthIndianChart";
import { VargaNakshatraTable } from "./VargaNakshatraTable";
import { VargaSignPositionsTable } from "../../vedic/VargaSignPositionsTable";

interface VargaVisualizationSectionProps {
    planets: VedicPosition[];
    ascendant: VedicPosition;
    vargaNakshatraReportsMap?: Record<string, VargaNakshatraReport>;
}

export function VargaVisualizationSection({ planets, ascendant, vargaNakshatraReportsMap }: VargaVisualizationSectionProps) {
    const [selectedVargaId, setSelectedVargaId] = useState<string>("rasi");
    const [chartStyle, setChartStyle] = useState<"south" | "north">("south");

    const vargaDef = VARGA_DEFS.find((v) => v.id === selectedVargaId) || VARGA_DEFS[0];
    const lagnaRasi = ascendant[vargaDef.key] as number;

    return (
        <section className="glass p-10 rounded-[3rem] border-white/10 relative overflow-hidden">
            <div className="flex flex-col md:flex-row md:items-center justify-between gap-6 mb-10">
                <div>
                    <h5 className="text-2xl font-bold text-white mb-2 flex items-center gap-3">
                        <Grid3x3 className="w-7 h-7 text-celestial-gold" />
                        분할 차트 시각화 (Varga Charts)
                    </h5>
                    <p className="text-sm text-white/40">
                        인생의 특정 영역을 현미경처럼 확대하여 분석하는 독립적인 차트들입니다.
                    </p>
                </div>
                <div className="flex items-center gap-3 flex-wrap">
                    <div className="flex bg-black/40 border border-white/20 rounded-xl overflow-hidden p-0.5">
                        <button
                            onClick={() => setChartStyle("south")}
                            className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${chartStyle === "south" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                        >
                            남인도
                        </button>
                        <button
                            onClick={() => setChartStyle("north")}
                            className={`px-3 py-1.5 text-xs font-semibold rounded-lg transition-all ${chartStyle === "north" ? "bg-white/15 text-white" : "text-white/40 hover:text-white/70"}`}
                        >
                            북인도
                        </button>
                    </div>
                    <select
                        value={selectedVargaId}
                        onChange={(e) => setSelectedVargaId(e.target.value)}
                        className="bg-white/5 border border-white/10 text-white text-xs font-bold rounded-xl px-4 py-2 focus:ring-2 focus:ring-celestial-gold/50 outline-none transition-all cursor-pointer hover:bg-white/10"
                    >
                        {VARGA_DEFS.map((v) => (
                            <option key={v.id} value={v.id} className="bg-slate-900">
                                {v.label} - {v.name}
                            </option>
                        ))}
                    </select>
                </div>
            </div>

            <div className="space-y-4 mb-8">
                <p className="text-[11px] font-bold text-white/20 uppercase tracking-widest flex items-center gap-2">
                    <span className="w-8 h-px bg-white/10" />
                    현재 선택된 차트: {vargaDef.label} ({vargaDef.name})
                    <span className="text-white/30 hidden md:inline">|</span>
                    <span className="text-white/30">황금 테두리 = 라그나 · 오른쪽 숫자 = 하우스 번호</span>
                </p>
                <div className="flex flex-col md:flex-row gap-8 items-start">
                    {chartStyle === "south" ? (
                        <SouthIndianChart
                            lagnaRasi={lagnaRasi}
                            planetEntries={planets.map((p) => ({ name: p.planet, rasi: p[vargaDef.key] as number, retro: p.is_retrograde, deg: p.sidereal_deg }))}
                        />
                    ) : (
                        <NorthIndianChart
                            lagnaRasi={lagnaRasi}
                            planetEntries={planets.map((p) => ({ name: p.planet, rasi: p[vargaDef.key] as number, retro: p.is_retrograde, deg: p.sidereal_deg }))}
                        />
                    )}
                    <div className="flex-1 w-full overflow-x-auto">
                        <VargaNakshatraTable 
                            title={`${vargaDef.label} 낙샤트라 상세`} 
                            vargaLabel={vargaDef.label} 
                            rows={vargaNakshatraReportsMap?.[selectedVargaId]?.rows || []} 
                        />
                    </div>
                </div>
            </div>

            <VargaSignPositionsTable planets={planets} ascendant={ascendant} />
        </section>
    );
}
