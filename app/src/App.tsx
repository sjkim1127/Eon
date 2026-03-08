import { lazy, Suspense, useEffect, useState } from "react";
import { Analytics } from "@vercel/analytics/react";
import { SpeedInsights } from "@vercel/speed-insights/react";
import { motion, AnimatePresence } from "framer-motion";
import { Compass, UserPlus } from "lucide-react";

import { useAnalysis } from "./hooks";
import { useTabPrefetcher } from "./hooks/useTabPrefetcher";
import { ShootingStars, BirthDrawer, Sidebar, CompactBirthInfoBar, ExportActionButtons } from "./components/shared";


const OverviewTab = lazy(() => import("./components/tabs/OverviewTab").then((m) => ({ default: m.OverviewTab })));
const SajuTab = lazy(() => import("./components/tabs/SajuTab").then((m) => ({ default: m.SajuTab })));
const VedicChartsTab = lazy(() => import("./components/tabs/VedicChartsTab").then((m) => ({ default: m.VedicChartsTab })));
const StrengthTab = lazy(() => import("./components/tabs/StrengthTab").then((m) => ({ default: m.StrengthTab })));
const TransitTab = lazy(() => import("./components/tabs/TransitTab").then((m) => ({ default: m.TransitTab })));
const CompatibilityTab = lazy(() => import("./components/tabs/CompatibilityTab").then((m) => ({ default: m.CompatibilityTab })));
const DestinyTierTab = lazy(() => import("./components/tabs/DestinyTierTab").then((m) => ({ default: m.DestinyTierTab })));
const AiAuditTab = lazy(() => import("./components/tabs/AiAuditTab").then((m) => ({ default: m.AiAuditTab })));

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

  const hasReport = !!(report || sajuReport);

  // 마르코프 체인 기반 탭 프리패칭 훅
  const { prefetchTab } = useTabPrefetcher(activeTab, {
    hasReport,
    hasTransit: !!transitReport,
    hasComp: !!compReport,
  });

  // 시간미상 → vedic_charts 탭 자동 회피
  useEffect(() => {
    if (birthData.unknown_time && activeTab === "vedic_charts") {
      setActiveTab("saju");
    }
  }, [birthData.unknown_time, activeTab, setActiveTab]);

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
              <p className="text-sm text-brand-400 mt-1">사주명리 &amp; 베딕 점성학 통합 분석</p>
            </div>
          </div>

          {hasReport ? (
            <CompactBirthInfoBar
              birthData={birthData}
              selectedCity={selectedCity}
              isMale={isMale}
              isDST={isDST}
              onEdit={() => setFormOpen(true)}
              actionSlot={
                <ExportActionButtons
                  sajuReport={sajuReport}
                  report={report}
                  transitReport={transitReport}
                  compReport={compReport}
                />
              }
            />
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
