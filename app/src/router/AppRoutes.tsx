import { lazy, Suspense } from "react";
import { Routes, Route, Navigate, useLocation } from "react-router-dom";
import { AnimatePresence } from "framer-motion";

import { TabSkeleton, UnavailableTabFallback, EmptyStateFallback } from "../components/layout/Fallbacks";
import { getTabAvailability } from "../utils/analysis";

const SajuTab = lazy(() => import("../components/tabs/SajuTab").then((m) => ({ default: m.SajuTab })));
const VedicChartsTab = lazy(() => import("../components/tabs/VedicChartsTab").then((m) => ({ default: m.VedicChartsTab })));
const StrengthTab = lazy(() => import("../components/tabs/StrengthTab").then((m) => ({ default: m.StrengthTab })));
const TransitTab = lazy(() => import("../components/tabs/TransitTab").then((m) => ({ default: m.TransitTab })));
const SimulationTab = lazy(() => import("../components/tabs/SimulationTab").then((m) => ({ default: m.SimulationTab })));
const DestinyTierTab = lazy(() => import("../components/tabs/DestinyTierTab").then((m) => ({ default: m.DestinyTierTab })));
const AiAuditTab = lazy(() => import("../components/tabs/AiAuditTab").then((m) => ({ default: m.AiAuditTab })));

interface AppRoutesProps {
  hasAnyReport: boolean;
  loading: boolean;
  sajuData: any;
  vedicData: any;
  transitData: any;
  tierData: any;
  unknownTime: boolean;
  birthData?: {
    year: number;
    month: number;
    day: number;
    hour: number;
    isMale: boolean;
  };
}

export function AppRoutes({
  hasAnyReport, loading, sajuData, vedicData, transitData, tierData, unknownTime, birthData
}: AppRoutesProps) {
  const location = useLocation();

  const availability = getTabAvailability({
    sajuData,
    vedicData,
    transitData,
    tierData,
    unknownTime,
  });

  return (
    <AnimatePresence mode="wait">
      {!hasAnyReport && !loading ? (
        <EmptyStateFallback />
      ) : (
        <Suspense fallback={<TabSkeleton />}>
          <Routes location={location} key={location.pathname}>
            <Route path="/" element={
              availability.saju ? (
                <SajuTab
                  sajuReport={sajuData!}
                  unknownTime={unknownTime}
                />
              ) : <UnavailableTabFallback reason="사주 분석 결과가 필요합니다." />
            } />
            <Route path="/vedic_charts" element={
              availability.vedic_charts ? <VedicChartsTab report={vedicData!} /> : <UnavailableTabFallback reason="시간 미상인 경우 베딕 차트를 생성할 수 없습니다." />
            } />
            <Route path="/strength" element={
              availability.strength ? <StrengthTab sajuReport={sajuData!} unknownTime={unknownTime} /> : <UnavailableTabFallback reason="사주 분석 결과가 필요합니다." />
            } />
            <Route path="/transit" element={
              availability.transit ? <TransitTab transitReport={transitData!} transitError={null} /> : <UnavailableTabFallback reason="트랜짓 분석 결과가 필요합니다." />
            } />
            <Route path="/simulation" element={
              availability.simulation ? <SimulationTab sajuReport={sajuData!} vedicReport={vedicData!} /> : <UnavailableTabFallback reason="사주 분석 결과가 필요합니다." />
            } />
            <Route path="/destiny_tier" element={
              availability.destiny_tier ? (
                <DestinyTierTab
                  sajuReport={sajuData}
                  report={vedicData}
                  transitReport={transitData}
                  tierReport={tierData}
                  unknownTime={unknownTime}
                />
              ) : <UnavailableTabFallback reason="종합 등급 산출을 위해 사주와 베딕 분석이 모두 성공해야 합니다." />
            } />
            <Route path="/ai_audit" element={
              availability.ai_audit ? (
                <AiAuditTab
                  sajuReport={sajuData}
                  birthYear={birthData?.year}
                  birthMonth={birthData?.month}
                  birthDay={birthData?.day}
                  birthHour={birthData?.hour}
                  isMale={birthData?.isMale}
                />
              ) : <UnavailableTabFallback reason="사주 분석 결과가 필요합니다." />
            } />
            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </Suspense>
      )}
    </AnimatePresence>
  );
}
