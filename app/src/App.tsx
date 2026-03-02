import { lazy, Suspense, useEffect, useRef } from "react";
import { Analytics } from "@vercel/analytics/react";
import { motion, AnimatePresence } from "framer-motion";
import { Compass } from "lucide-react";

import { useAnalysis } from "./hooks";
import { ShootingStars, BirthInputForm, Sidebar } from "./components/shared";
import type { TabId } from "./types";

const loadOverviewTab = () => import("./components/tabs/OverviewTab");
const loadSajuTab = () => import("./components/tabs/SajuTab");
const loadVedicChartsTab = () => import("./components/tabs/VedicChartsTab");
const loadStrengthTab = () => import("./components/tabs/StrengthTab");
const loadTransitTab = () => import("./components/tabs/TransitTab");
const loadCompatibilityTab = () => import("./components/tabs/CompatibilityTab");

const OverviewTab = lazy(() => loadOverviewTab().then((m) => ({ default: m.OverviewTab })));
const SajuTab = lazy(() => loadSajuTab().then((m) => ({ default: m.SajuTab })));
const VedicChartsTab = lazy(() => loadVedicChartsTab().then((m) => ({ default: m.VedicChartsTab })));
const StrengthTab = lazy(() => loadStrengthTab().then((m) => ({ default: m.StrengthTab })));
const TransitTab = lazy(() => loadTransitTab().then((m) => ({ default: m.TransitTab })));
const CompatibilityTab = lazy(() => loadCompatibilityTab().then((m) => ({ default: m.CompatibilityTab })));

const TABS: TabId[] = ["overview", "saju", "vedic_charts", "strength", "transit", "compatibility"];

const FALLBACK_NEXT_TABS: Record<TabId, TabId[]> = {
  overview: ["saju", "strength"],
  saju: ["strength", "transit"],
  vedic_charts: ["overview", "strength"],
  strength: ["transit", "saju"],
  transit: ["compatibility", "overview"],
  compatibility: ["overview", "saju"],
};

const REPORT_READY_BONUS: Partial<Record<TabId, number>> = {
  saju: 4,
  strength: 4,
  vedic_charts: 2,
  transit: 2,
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
    selectedCity, handleCityChange,
    isMale, setIsMale,
    isDST,
    report, sajuReport, transitReport,
    loading, runAnalysis,
    errorMessage,
    activeTab, setActiveTab,
    birthData2, setBirthData2,
    selectedCity2, handleCityChange2,
    isMale2, setIsMale2,
    compReport, compLoading,
    runCompatibilityAnalysis,
  } = useAnalysis();
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
    if (tab === "overview") {
      void loadOverviewTab();
      return;
    }
    if (tab === "saju") {
      void loadSajuTab();
      return;
    }
    if (tab === "vedic_charts") {
      void loadVedicChartsTab();
      return;
    }
    if (tab === "strength") {
      void loadStrengthTab();
      return;
    }
    if (tab === "transit") {
      void loadTransitTab();
      return;
    }
    void loadCompatibilityTab();
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

  return (
    <div className="h-screen w-full relative flex overflow-hidden">
      <ShootingStars />

      {/* Sidebar */}
      <Sidebar activeTab={activeTab} setActiveTab={setActiveTab} onTabHover={prefetchTab} />

      {/* Main Content */}
      <main className="flex-1 p-4 md:p-10 pb-24 md:pb-10 overflow-y-auto z-10">
        <header className="mb-8">
          <div className="flex justify-between items-end mb-6">
            <div>
              <h2 className="text-2xl md:text-4xl font-bold text-white mb-2">천문 인사이트</h2>
              <p className="text-sm md:text-base text-brand-400">사주명리 & 베딕 점성학 통합 분석</p>
            </div>
          </div>

          {/* 출생 정보 입력 폼 */}
          <BirthInputForm
            birthData={birthData}
            setBirthData={setBirthData}
            selectedCity={selectedCity}
            onCityChange={handleCityChange}
            isMale={isMale}
            setIsMale={setIsMale}
            isDST={isDST}
            loading={loading}
            onAnalysis={runAnalysis}
            sajuReport={sajuReport}
          />

          {errorMessage && (
            <p className="text-sm text-red-300/90 bg-red-500/10 border border-red-500/30 rounded-xl px-4 py-2">
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
                <TransitTab transitReport={transitReport} />
              ) : activeTab === "compatibility" ? (
                <CompatibilityTab
                  birthData2={birthData2}
                  setBirthData2={setBirthData2}
                  isMale2={isMale2}
                  setIsMale2={setIsMale2}
                  selectedCity2={selectedCity2}
                  onCityChange2={handleCityChange2}
                  compReport={compReport}
                  compLoading={compLoading}
                  onRunCompatibility={runCompatibilityAnalysis}
                />
              ) : (
                <OverviewTab report={report!} />
              )}
            </Suspense>
          )}
        </AnimatePresence>
      </main>
      <Analytics />
    </div>
  );
}

export default App;
