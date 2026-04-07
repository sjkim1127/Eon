import { useEffect, useState, useMemo } from "react";
import { Analytics } from "@vercel/analytics/react";
import { SpeedInsights } from "@vercel/speed-insights/react";
import { useLocation, useNavigate } from "react-router-dom";

import { useBirthForm, useAstrologyAnalysis } from "./hooks";
import { useTabPrefetcher } from "./hooks/useTabPrefetcher";
import { useAppStore } from "./store/useAppStore";
import type { TabId } from "./types";

import { AppLayout } from "./components/layout/AppLayout";
import { AppRoutes } from "./router/AppRoutes";

function App() {
  const location = useLocation();
  const navigate = useNavigate();

  // Route -> TabId mapping
  const pathParts = location.pathname.split('/').filter(Boolean);
  const currentTab: TabId = (pathParts.length > 0 ? pathParts[0] : "saju") as TabId;

  const setActiveTab = (tab: TabId) => {
    useAppStore.getState().setActiveTab(tab);
    navigate(`/${tab === 'saju' ? '' : tab}`);
  };

  const errorMessage = useAppStore(state => state.errorMessage);
  const analysisState = useAppStore(state => state.analysisState);

  const vedicData = analysisState.vedic.data;
  const sajuData = analysisState.saju.data;
  const transitData = analysisState.transit.data;
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

  const isDST = sajuData?.meta?.isDst ?? false;

  const [formOpen, setFormOpen] = useState(true);

  const hasAnyReport = useMemo(() =>
    Object.values(analysisState).some(s => s.status === "success" && s.data),
    [analysisState]
  );

  const { prefetchTab } = useTabPrefetcher(currentTab, {
    hasReport: !!(vedicData || sajuData),
    hasTransit: !!transitData,
  });

  // 시간미상 가드
  useEffect(() => {
    if (birthData.unknownTime && currentTab === "vedic_charts") {
      setActiveTab("saju");
    }
  }, [birthData.unknownTime, currentTab]);

  return (
    <>
      <AppLayout
        currentTab={currentTab}
        setActiveTab={setActiveTab}
        prefetchTab={prefetchTab}
        birthData={birthData}
        setBirthData={setBirthData}
        selectedCity={selectedCity}
        handleCitySelect={handleCitySelect}
        isMale={isMale}
        setIsMale={setIsMale}
        isDST={isDST}
        sajuData={sajuData}
        vedicData={vedicData}
        transitData={transitData}
        tierData={tierData}
        hasAnyReport={hasAnyReport}
        formOpen={formOpen}
        setFormOpen={setFormOpen}
        loading={loading}
        runAnalysis={runAnalysis}
        errorMessage={errorMessage}
      >
        <AppRoutes 
          hasAnyReport={hasAnyReport}
          loading={loading}
          sajuData={sajuData}
          vedicData={vedicData}
          transitData={transitData}
          tierData={tierData}
          unknownTime={!!birthData.unknownTime}
        />
        {sajuData?.meta?.correctedTime && (
          <div className="flex items-center gap-2 px-3 py-1 bg-violet-500/20 text-violet-300 rounded-full text-xs font-medium border border-violet-500/30">
            <span className="w-1.5 h-1.5 bg-violet-400 rounded-full animate-pulse" />
            보정됨 ({sajuData.meta.correctedTime})
          </div>
        )}
      </AppLayout>

      <Analytics />
      <SpeedInsights />
    </>
  );
}

export default App;
