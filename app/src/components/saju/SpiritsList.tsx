import { Shield, Sparkles, AlertCircle, Info } from "lucide-react";
import { PILLAR_POS_INFO, SPIRIT_INFO } from "../../constants";
import { SpiritMarkerAnalysis } from "../../types/saju";

interface Props {
  sp: SpiritMarkerAnalysis;
}

export function SpiritsList({ sp }: Props) {
  // mappedMarkers가 있으면 이를 우선 사용, 없으면 markers 사용 (하위 호환)
  const hasDetails = !!sp?.mappedMarkers && sp.mappedMarkers.length > 0;
  const markersToRender = hasDetails ? sp.mappedMarkers : (sp?.markers || []);

  if (markersToRender.length === 0) return null;

  const order = ["Year", "Month", "Day", "Hour"];
  
  const groups: Record<string, any[]> = {};
  for (const m of markersToRender) {
    const pos = m.position || "Unknown";
    if (!groups[pos]) groups[pos] = [];
    groups[pos].push(m);
  }

  const sortedPositions = Object.keys(groups).sort((a, b) => {
    const idxA = order.indexOf(a);
    const idxB = order.indexOf(b);
    if (idxA === -1 && idxB === -1) return a.localeCompare(b);
    if (idxA === -1) return 1;
    if (idxB === -1) return -1;
    return idxA - idxB;
  });

  return (
    <div className="glass p-8 rounded-[2rem] border border-white/10">
      <div className="flex items-center justify-between mb-8">
        <h5 className="text-2xl font-bold text-white flex items-center gap-3">
          <Shield className="w-8 h-8 text-celestial-cyan" />
          신살 (神煞) 분석
        </h5>
        <div className="flex gap-4">
          <div className="flex items-center gap-1.5 text-xs text-celestial-gold/80 bg-celestial-gold/10 px-3 py-1 rounded-full border border-celestial-gold/20">
            <Sparkles className="w-3 h-3" />
            <span>길신</span>
          </div>
          <div className="flex items-center gap-1.5 text-xs text-rose-400/80 bg-rose-400/10 px-3 py-1 rounded-full border border-rose-400/20">
            <AlertCircle className="w-3 h-3" />
            <span>흉살</span>
          </div>
        </div>
      </div>

      <div className="space-y-10">
        {sortedPositions.map((pos) => (
          <div key={pos} className="relative">
            <div className="flex items-center gap-4 mb-6">
              <span className="px-4 py-1.5 rounded-lg bg-white/10 text-white font-bold tracking-widest text-sm shadow-inner">
                {PILLAR_POS_INFO[pos] || pos}
              </span>
              <div className="flex-1 h-px bg-gradient-to-r from-white/20 to-transparent" />
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {groups[pos].map((m: any, i: number) => {
                const isDetail = "summary" in m;
                const info = SPIRIT_INFO[m.marker];
                const level = m.level || (m.marker && isAuspicious(m.marker) ? "Auspicious" : "Caution");
                
                return (
                  <div
                    key={i}
                    className={`group relative p-5 rounded-2xl border transition-all duration-300 hover:scale-[1.02] ${
                      level === "Auspicious" 
                        ? "bg-celestial-gold/5 border-celestial-gold/20 hover:bg-celestial-gold/10" 
                        : "bg-white/5 border-white/10 hover:bg-white/10"
                    }`}
                  >
                    <div className="flex items-start justify-between mb-3">
                      <div>
                        <div className="flex items-center gap-2">
                          <h6 className={`text-lg font-bold ${level === "Auspicious" ? "text-celestial-gold" : "text-white/90"}`}>
                            {info?.hangul || m.marker || "—"}
                          </h6>
                          {info?.hanja && (
                            <span className="text-xs text-white/30 font-serif">
                              {info.hanja}
                            </span>
                          )}
                        </div>
                        {isDetail && (
                          <p className="text-sm text-white/60 mt-1 font-medium leading-relaxed">
                            {m.summary}
                          </p>
                        )}
                      </div>
                      {level === "Auspicious" ? (
                        <Sparkles className="w-5 h-5 text-celestial-gold/50" />
                      ) : (
                        <AlertCircle className="w-5 h-5 text-rose-400/30" />
                      )}
                    </div>

                    {isDetail && (
                      <>
                        <p className="text-xs text-white/40 mb-4 line-clamp-2 italic">
                          {m.description}
                        </p>
                        <div className="flex flex-wrap gap-1.5">
                          {m.reasons.map((reason: string, idx: number) => (
                            <span
                              key={idx}
                              className="px-2 py-0.5 rounded-md bg-white/5 border border-white/10 text-[10px] text-white/50 font-medium"
                            >
                              {reason}
                            </span>
                          ))}
                        </div>
                      </>
                    )}
                    
                    {!isDetail && info?.hangul && (
                      <div className="flex items-center gap-1.5 text-[10px] text-white/30 italic">
                        <Info className="w-3 h-3" />
                        <span>기본 해석 제공 중</span>
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

// 헬퍼 함수 (하위 호환용)
function isAuspicious(marker: string): boolean {
  const auspicious = [
    "Tianyi", "Wenchang", "Taiji", "Yuede", "Tiande", 
    "Zhenglu", "Jinyu", "Anlu", "Xuetang", "TianyiMedical", "Tianwen"
  ];
  return auspicious.includes(marker);
}
