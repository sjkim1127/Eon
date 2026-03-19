import { lazy, Suspense, useEffect, useState, useMemo } from "react";
import { Analytics } from "@vercel/analytics/react";
import { SpeedInsights } from "@vercel/speed-insights/react";
import { motion, AnimatePresence } from "framer-motion";
import { Compass, UserPlus, AlertCircle } from "lucide-react";
import { Routes, Route, Navigate, useLocation, useNavigate } from "react-router-dom";

import { useBirthForm, useAstrologyAnalysis, useCompatibility } from "./hooks";
import { useTabPrefetcher } from "./hooks/useTabPrefetcher";
import { useAppStore } from "./store/useAppStore";
import { ShootingStars, BirthDrawer, Sidebar, CompactBirthInfoBar, ExportActionButtons } from "./components/shared";
import type { TabId } from "./types";

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

function UnavailableTabFallback({ reason }: { reason: string }) {
  return (
    <div className="h-[40vh] flex flex-col items-center justify-center text-center p-8 glass rounded-[2rem]">
      <AlertCircle className="w-12 h-12 text-white/20 mb-4" />
      <h3 className="text-xl font-semibold text-white mb-2">분석 결과 없음</h3>
      <p className="text-brand-400 max-w-sm">{reason}</p>
    </div>
  );
}

function App() {
  const location = useLocation();
  const navigate = useNavigate();

  // Route -> TabId mapping
  const pathParts = location.pathname.split('/').filter(Boolean);
  const currentTab: TabId = (pathParts.length > 0 ? pathParts[0] : "overview") as TabId;

  const setActiveTab = (tab: TabId) => {
    useAppStore.getState().setActiveTab(tab); 
    navigate(`/${tab === 'overview' ? '' : tab}`);
  };

  const errorMessage = useAppStore(state => state.errorMessage);
  const analysisState = useAppStore(state => state.analysisState);

  const vedicData = analysisState.vedic.data;
  const sajuData = analysisState.saju.data;
  const transitData = analysisState.transit.data;
  const aiAuditData = analysisState.aiAudit.data;
  const tierData = analysisState.tier.data;

  const {
    birthData, setBirthData,
    selectedCity, handleCitySelect,
    isMale, setIsMale,
  } = useBirthForm();

  const {
    runAnalysis,
    loading,
  } = useAstrologyAnalysis();

  const { compReport } = useCompatibility();

  const isDST = sajuData?.meta.is_dst ?? false;

  const [formOpen, setFormOpen] = useState(true);

  const availability = useMemo(() => ({
    overview: analysisState.vedic.status === "success" && !!vedicData,
    saju: analysisState.saju.status === "success" && !!sajuData,
    vedic_charts: analysisState.vedic.status === "success" && !!vedicData && !birthData.unknown_time,
    strength: analysisState.saju.status === "success" && !!sajuData,
    transit: analysisState.transit.status === "success" && !!transitData,
    compatibility: true,
    destiny_tier: analysisState.tier.status === "success" && !!tierData,
    ai_audit: analysisState.aiAudit.status === "success" && !!aiAuditData,
  }), [analysisState, birthData.unknown_time, vedicData, sajuData, transitData, tierData, aiAuditData]);

  const hasAnyReport = useMemo(() => 
    Object.values(analysisState).some(s => s.status === "success" && s.data),
    [analysisState]
  );

  const { prefetchTab } = useTabPrefetcher(currentTab, {
    hasReport: !!(vedicData || sajuData),
    hasTransit: !!transitData,
    hasComp: !!compReport,
  });

  // 시간미상 가드
  useEffect(() => {
    if (birthData.unknown_time && currentTab === "vedic_charts") {
      setActiveTab("saju");
    }
  }, [birthData.unknown_time, currentTab]);

  return (
    <div className="h-screen w-full relative flex overflow-hidden">
      <ShootingStars />

      <Sidebar
        activeTab={currentTab}
        setActiveTab={setActiveTab}
        onTabHover={prefetchTab}
        unknownTime={birthData.unknown_time}
      />

      <main className="flex-1 p-4 md:p-10 pb-24 md:pb-10 overflow-y-auto z-10">
        <header className="mb-8">
          <div className="flex justify-between items-end mb-5">
            <div>
              <h2 className="text-2xl md:text-3xl font-bold text-white tracking-tight">역학적 인사이트</h2>
              <p className="text-sm text-brand-400 mt-1">사주명리 &amp; 베딕 점성학 통합 분석</p>
            </div>
          </div>

          {hasAnyReport ? (
            <CompactBirthInfoBar
              birthData={birthData}
              selectedCity={selectedCity}
              isMale={isMale}
              isDST={isDST}
              onEdit={() => setFormOpen(true)}
              actionSlot={
                <ExportActionButtons
                  sajuReport={sajuData}
                  report={vedicData}
                  transitReport={transitData}
                  compReport={compReport}
                  tierResult={tierData}
                />
              }
            />
          ) : (
            <button
              onClick={() => setFormOpen(true)}
              className="w-full py-5 rounded-2xl mb-5 border border-dashed border-white/15 hover:border-celestial-purple/40 hover:bg-celestial-purple/5 text-white/40 hover:text-white/70 transition-all text-sm font-medium flex items-center justify-center gap-2"
            >
              <UserPlus className="w-4 h-4" />
              출생 정보 입력하여 분석 시작
            </button>
          )}
          {sajuData?.meta?.corrected_time && (
            <span className="text-celestial-cyan/60">
              보정시: {sajuData.meta.corrected_time}
            </span>
          )}
          {sajuData?.meta?.is_dst && (
            <span className="text-amber-400/80">DST 적용됨</span>
          )}

          {errorMessage && (
            <p className="text-sm text-red-300/90 bg-red-500/10 border border-red-500/30 rounded-xl px-4 py-2 mb-2">
              {errorMessage}
            </p>
          )}
        </header>

        <AnimatePresence mode="wait">
          {!hasAnyReport && !loading ? (
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
              <Routes location={location} key={location.pathname}>
                <Route path="/" element={
                  availability.overview ? <OverviewTab report={vedicData!} /> : <UnavailableTabFallback reason="베딕 분석 결과가 필요합니다." />
                } />
                <Route path="/saju" element={
                  availability.saju ? <SajuTab sajuReport={sajuData!} unknownTime={birthData.unknown_time} /> : <UnavailableTabFallback reason="사주 분석 결과가 필요합니다." />
                } />
                <Route path="/vedic_charts" element={
                  availability.vedic_charts ? <VedicChartsTab report={vedicData!} /> : <UnavailableTabFallback reason="시간 미상인 경우 베딕 차트를 생성할 수 없습니다." />
                } />
                <Route path="/strength" element={
                  availability.strength ? <StrengthTab sajuReport={sajuData!} unknownTime={birthData.unknown_time} /> : <UnavailableTabFallback reason="사주 분석 결과가 필요합니다." />
                } />
                <Route path="/transit" element={
                  availability.transit ? <TransitTab transitReport={transitData!} transitError={null} /> : <UnavailableTabFallback reason="트랜짓 분석 결과가 필요합니다." />
                } />
                <Route path="/compatibility" element={<CompatibilityTab />} />
                <Route path="/destiny_tier" element={
                  availability.destiny_tier ? (
                    <DestinyTierTab
                      sajuReport={sajuData}
                      report={vedicData}
                      transitReport={transitData}
                      tierReport={tierData}
                      unknownTime={birthData.unknown_time}
                    />
                  ) : <UnavailableTabFallback reason="종합 등급 산출을 위해 사주와 베딕 분석이 모두 성공해야 합니다." />
                } />
                <Route path="/ai_audit" element={
                  availability.ai_audit ? <AiAuditTab aiAuditReport={aiAuditData!} /> : <UnavailableTabFallback reason="AI 진단 결과가 필요합니다." />
                } />
                <Route path="*" element={<Navigate to="/" replace />} />
              </Routes>
            </Suspense>
          )}
        </AnimatePresence>
      </main>

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
        sajuReport={sajuData}
      />

      <Analytics />
      <SpeedInsights />
    </div>
  );
}

export default App;
