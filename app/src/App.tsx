import { lazy, Suspense, useEffect, useRef, useState } from "react";
import { Analytics } from "@vercel/analytics/react";
import { SpeedInsights } from "@vercel/speed-insights/react";
import { motion, AnimatePresence } from "framer-motion";
import { Compass, Calendar, UserPlus, Pencil, ClipboardCopy, Check } from "lucide-react";

import { useAnalysis } from "./hooks";
import { ShootingStars, BirthDrawer, Sidebar } from "./components/shared";
import { buildFullAnalysisMarkdown, buildSajuMarkdown, buildVedicMarkdown, buildCompatibilityMarkdown } from "./utils";
import type { TabId } from "./types";

const loadOverviewTab = () => import("./components/tabs/OverviewTab");
const loadSajuTab = () => import("./components/tabs/SajuTab");
const loadVedicChartsTab = () => import("./components/tabs/VedicChartsTab");
const loadStrengthTab = () => import("./components/tabs/StrengthTab");
const loadTransitTab = () => import("./components/tabs/TransitTab");
const loadCompatibilityTab = () => import("./components/tabs/CompatibilityTab");
const loadDestinyTierTab = () => import("./components/tabs/DestinyTierTab");
const loadAiAuditTab = () => import("./components/tabs/AiAuditTab");

const TAB_LOADERS: Record<TabId, () => Promise<unknown>> = {
  overview: loadOverviewTab,
  saju: loadSajuTab,
  vedic_charts: loadVedicChartsTab,
  strength: loadStrengthTab,
  transit: loadTransitTab,
  compatibility: loadCompatibilityTab,
  destiny_tier: loadDestinyTierTab,
  ai_audit: loadAiAuditTab,
};

const OverviewTab = lazy(() => loadOverviewTab().then((m) => ({ default: m.OverviewTab })));
const SajuTab = lazy(() => loadSajuTab().then((m) => ({ default: m.SajuTab })));
const VedicChartsTab = lazy(() => loadVedicChartsTab().then((m) => ({ default: m.VedicChartsTab })));
const StrengthTab = lazy(() => loadStrengthTab().then((m) => ({ default: m.StrengthTab })));
const TransitTab = lazy(() => loadTransitTab().then((m) => ({ default: m.TransitTab })));
const CompatibilityTab = lazy(() => loadCompatibilityTab().then((m) => ({ default: m.CompatibilityTab })));
const DestinyTierTab = lazy(() => loadDestinyTierTab().then((m) => ({ default: m.DestinyTierTab })));
const AiAuditTab = lazy(() => loadAiAuditTab().then((m) => ({ default: m.AiAuditTab })));

const TABS: TabId[] = ["overview", "saju", "vedic_charts", "strength", "transit", "compatibility", "destiny_tier", "ai_audit"];

const FALLBACK_NEXT_TABS: Record<TabId, TabId[]> = {
  overview: ["saju", "strength"],
  saju: ["strength", "transit"],
  vedic_charts: ["overview", "strength"],
  strength: ["transit", "saju"],
  transit: ["compatibility", "overview"],
  compatibility: ["destiny_tier", "overview"],
  destiny_tier: ["overview", "strength"],
  ai_audit: ["overview", "saju"],
};

const REPORT_READY_BONUS: Partial<Record<TabId, number>> = {
  saju: 4,
  strength: 4,
  vedic_charts: 2,
  transit: 2,
  destiny_tier: 5,
};

const TRANSIT_READY_BONUS: Partial<Record<TabId, number>> = {
  transit: 5,
  strength: 2,
};

const COMPATIBILITY_READY_BONUS: Partial<Record<TabId, number>> = {
  compatibility: 5,
  overview: 1,
};

function TabSkeleton() {
  return (
    <div className="glass p-8 rounded-[2rem] animate-pulse">
      <div className="h-4 w-40 bg-white/10 rounded mb-4" />
      <div className="h-24 bg-white/5 rounded-xl mb-3" />
      <div className="h-24 bg-white/5 rounded-xl mb-3" />
      <div className="h-24 bg-white/5 rounded-xl" />
    </div>
  );
}

function App() {
  const {
    birthData, setBirthData,
    selectedCity, handleCitySelect,
    isMale, setIsMale,
    isDST,
    report, sajuReport, transitReport, transitError,
    aiAuditReport,
    loading, runAnalysis,
    errorMessage,
    activeTab, setActiveTab,
    birthData2, setBirthData2,
    selectedCity2, handleCitySelect2,
    isMale2, setIsMale2,
    compReport, compLoading,
    runCompatibilityAnalysis,
  } = useAnalysis();

  // 드로어 상태: 첫 로드시 자동 오픈 (온보딩)
  const [formOpen, setFormOpen] = useState(true);
  const [mdCopied, setMdCopied] = useState(false);
  const [sajuCopied, setSajuCopied] = useState(false);
  const [vedicCopied, setVedicCopied] = useState(false);
  const [compCopied, setCompCopied] = useState(false);

  // 분석 완료 시 닫기는 BirthInputForm 내부 온클릭에서 처리됨

  const pad = (n: number) => String(n).padStart(2, "0");
  const hasReport = !!(report || sajuReport);
  const transitionRef = useRef<Record<TabId, Record<TabId, number>>>(
    TABS.reduce((acc, from) => {
      acc[from] = TABS.reduce((inner, to) => {
        inner[to] = 0;
        return inner;
      }, {} as Record<TabId, number>);
      return acc;
    }, {} as Record<TabId, Record<TabId, number>>)
  );
  const prevTabRef = useRef<TabId | null>(null);

  const prefetchTab = (tab: TabId) => {
    void TAB_LOADERS[tab]?.();
  };

  const getDomainWeightedTabs = (currentTab: TabId): TabId[] => {
    const transitions = transitionRef.current[currentTab];

    const scores = TABS.reduce((acc, tab) => {
      if (tab === currentTab) {
        return acc;
      }

      const learnedScore = transitions[tab] ?? 0;
      const fallbackScore = FALLBACK_NEXT_TABS[currentTab].includes(tab) ? 2 : 0;
      const reportScore = (report || sajuReport) ? (REPORT_READY_BONUS[tab] ?? 0) : 0;
      const transitScore = transitReport ? (TRANSIT_READY_BONUS[tab] ?? 0) : 0;
      const compatibilityScore = compReport ? (COMPATIBILITY_READY_BONUS[tab] ?? 0) : 0;

      acc[tab] = learnedScore + fallbackScore + reportScore + transitScore + compatibilityScore;
      return acc;
    }, {} as Partial<Record<TabId, number>>);

    return TABS
      .filter((tab) => tab !== currentTab)
      .sort((a, b) => (scores[b] ?? 0) - (scores[a] ?? 0))
      .slice(0, 2);
  };

  useEffect(() => {
    const preloadMostUsedTabs = () => {
      void loadOverviewTab();
      void loadSajuTab();
      void loadStrengthTab();
    };

    if (typeof window !== "undefined" && "requestIdleCallback" in window) {
      const idleId = window.requestIdleCallback(preloadMostUsedTabs, { timeout: 1200 });
      return () => window.cancelIdleCallback(idleId);
    }

    const timeoutId = setTimeout(preloadMostUsedTabs, 600);
    return () => clearTimeout(timeoutId);
  }, []);

  useEffect(() => {
    const prev = prevTabRef.current;
    if (prev && prev !== activeTab) {
      transitionRef.current[prev][activeTab] += 1;
    }
    prevTabRef.current = activeTab;

    const candidates = getDomainWeightedTabs(activeTab);
    for (const nextTab of candidates) {
      prefetchTab(nextTab);
    }
  }, [activeTab, report, sajuReport, transitReport, compReport]);

  useEffect(() => {
    if (birthData.unknown_time && activeTab === "vedic_charts") {
      setActiveTab("saju");
    }
  }, [birthData.unknown_time, activeTab]);

  return (
    <div className="h-screen w-full relative flex overflow-hidden">
      <ShootingStars />

      {/* Sidebar */}
      <Sidebar
        activeTab={activeTab}
        setActiveTab={setActiveTab}
        onTabHover={prefetchTab}
        unknownTime={birthData.unknown_time}
      />

      {/* Main Content */}
      <main className="flex-1 p-4 md:p-10 pb-24 md:pb-10 overflow-y-auto z-10">
        <header className="mb-8">
          <div className="flex justify-between items-end mb-5">
            <div>
              <h2 className="text-2xl md:text-3xl font-bold text-white tracking-tight">역학적 인사이트</h2>
              <p className="text-sm text-brand-400 mt-1">사주명리 & 베딕 점성학 통합 분석</p>
            </div>
          </div>

          {/* 드로어 트리거 */}
          {hasReport ? (
            /* compact 출생 요약 바 */
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
                <span className={isMale ? "text-celestial-cyan" : "text-pink-400"}>{isMale ? "남" : "여"}</span>
                {isDST && (
                  <span className="text-[10px] px-2 py-0.5 rounded-full bg-amber-500/20 text-amber-400 border border-amber-500/30">
                    DST
                  </span>
                )}
              </div>
              <button
                onClick={() => setFormOpen(true)}
                className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 text-white/50 hover:text-white transition-all font-medium"
              >
                <Pencil className="w-3 h-3" />
                수정
              </button>
              <div className="flex items-center gap-2">
                <button
                  onClick={async () => {
                    const md = buildFullAnalysisMarkdown(sajuReport ?? null, report ?? null, transitReport ?? null, compReport ?? null);
                    await navigator.clipboard.writeText(md);
                    setMdCopied(true);
                    setTimeout(() => setMdCopied(false), 2500);
                  }}
                  className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-white/10 bg-white/5 hover:bg-white/10 text-white/50 hover:text-white transition-all font-medium"
                >
                  {mdCopied ? <Check className="w-3 h-3 text-green-400" /> : <ClipboardCopy className="w-3 h-3" />}
                  {mdCopied ? "복사됨!" : "전체 복사"}
                </button>
                {sajuReport && (
                  <button
                    onClick={async () => {
                      const md = buildSajuMarkdown(sajuReport);
                      await navigator.clipboard.writeText(md);
                      setSajuCopied(true);
                      setTimeout(() => setSajuCopied(false), 2500);
                    }}
                    className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-brand-500/20 bg-brand-500/10 hover:bg-brand-500/20 text-brand-300 transition-all font-medium"
                  >
                    {sajuCopied ? <Check className="w-3 h-3 text-green-400" /> : <ClipboardCopy className="w-3 h-3" />}
                    {sajuCopied ? "사주 복사됨!" : "사주 복사"}
                  </button>
                )}
                {report && (
                  <button
                    onClick={async () => {
                      const md = buildVedicMarkdown(report);
                      await navigator.clipboard.writeText(md);
                      setVedicCopied(true);
                      setTimeout(() => setVedicCopied(false), 2500);
                    }}
                    className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-celestial-purple/30 bg-celestial-purple/10 hover:bg-celestial-purple/20 text-indigo-300 transition-all font-medium"
                  >
                    {vedicCopied ? <Check className="w-3 h-3 text-green-400" /> : <ClipboardCopy className="w-3 h-3" />}
                    {vedicCopied ? "베딕 복사됨!" : "베딕 복사"}
                  </button>
                )}
                {compReport && (
                  <button
                    onClick={async () => {
                      const md = buildCompatibilityMarkdown(compReport);
                      await navigator.clipboard.writeText(md);
                      setCompCopied(true);
                      setTimeout(() => setCompCopied(false), 2500);
                    }}
                    className="shrink-0 flex items-center gap-1.5 text-xs px-3 py-1.5 rounded-lg border border-pink-500/30 bg-pink-500/10 hover:bg-pink-500/20 text-pink-300 transition-all font-medium"
                  >
                    {compCopied ? <Check className="w-3 h-3 text-green-400" /> : <ClipboardCopy className="w-3 h-3" />}
                    {compCopied ? "궁합 복사됨!" : "궁합 복사"}
                  </button>
                )}
              </div>
            </div>
          ) : (
            /* 첫 진입 CTA */
            <button
              onClick={() => setFormOpen(true)}
              className="w-full py-5 rounded-2xl mb-5 border border-dashed border-white/15 hover:border-celestial-purple/40 hover:bg-celestial-purple/5 text-white/40 hover:text-white/70 transition-all text-sm font-medium flex items-center justify-center gap-2"
            >
              <UserPlus className="w-4 h-4" />
              출생 정보 입력하여 분석 시작
            </button>
          )}

          {errorMessage && (
            <p className="text-sm text-red-300/90 bg-red-500/10 border border-red-500/30 rounded-xl px-4 py-2 mb-2">
              {errorMessage}
            </p>
          )}
        </header>

        <AnimatePresence mode="wait">
          {!report && !sajuReport ? (
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
          ) : (
            <Suspense fallback={<TabSkeleton />}>
              {activeTab === "saju" ? (
                <SajuTab sajuReport={sajuReport} unknownTime={birthData.unknown_time} />
              ) : activeTab === "vedic_charts" ? (
                <VedicChartsTab report={report!} />
              ) : activeTab === "strength" ? (
                <StrengthTab sajuReport={sajuReport} unknownTime={birthData.unknown_time} />
              ) : activeTab === "transit" ? (
                <TransitTab transitReport={transitReport} transitError={transitError} />
              ) : activeTab === "compatibility" ? (
                <CompatibilityTab
                  birthData2={birthData2}
                  setBirthData2={setBirthData2}
                  isMale2={isMale2}
                  setIsMale2={setIsMale2}
                  selectedCity2={selectedCity2}
                  onCitySelect2={handleCitySelect2}
                  compReport={compReport}
                  compLoading={compLoading}
                  onRunCompatibility={runCompatibilityAnalysis}
                />
              ) : activeTab === "destiny_tier" ? (
                <DestinyTierTab
                  sajuReport={sajuReport}
                  report={report}
                  transitReport={transitReport}
                  unknownTime={birthData.unknown_time}
                />
              ) : activeTab === "ai_audit" ? (
                <AiAuditTab aiAuditReport={aiAuditReport} />
              ) : (
                <OverviewTab report={report!} />
              )}
            </Suspense>
          )}
        </AnimatePresence>
      </main>

      {/* 출생 정보 드로어 */}
      <BirthDrawer
        open={formOpen}
        onClose={() => setFormOpen(false)}
        birthData={birthData}
        setBirthData={setBirthData}
        selectedCity={selectedCity}
        onCitySelect={handleCitySelect}
        isMale={isMale}
        setIsMale={setIsMale}
        isDST={isDST}
        loading={loading}
        onAnalysis={runAnalysis}
        sajuReport={sajuReport}
      />

      <Analytics />
      <SpeedInsights />
    </div>
  );
}

export default App;
