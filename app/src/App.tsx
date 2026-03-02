import { Analytics } from "@vercel/analytics/react";
import { motion, AnimatePresence } from "framer-motion";
import { Compass } from "lucide-react";

import { useAnalysis } from "./hooks";
import { ShootingStars, BirthInputForm, Sidebar } from "./components/shared";
import {
  OverviewTab,
  SajuTab,
  VedicChartsTab,
  StrengthTab,
  TransitTab,
  CompatibilityTab,
} from "./components/tabs";

function App() {
  const {
    birthData, setBirthData,
    selectedCity, handleCityChange,
    isMale, setIsMale,
    isDST,
    report, sajuReport, transitReport,
    loading, runAnalysis,
    activeTab, setActiveTab,
    birthData2, setBirthData2,
    selectedCity2, handleCityChange2,
    isMale2, setIsMale2,
    compReport, compLoading,
    runCompatibilityAnalysis,
  } = useAnalysis();

  return (
    <div className="h-screen w-full relative flex overflow-hidden">
      <ShootingStars />

      {/* Sidebar */}
      <Sidebar activeTab={activeTab} setActiveTab={setActiveTab} />

      {/* Main Content */}
      <main className="flex-1 p-10 overflow-y-auto z-10">
        <header className="mb-8">
          <div className="flex justify-between items-end mb-6">
            <div>
              <h2 className="text-4xl font-bold text-white mb-2">천문(Celestial) 인사이트</h2>
              <p className="text-brand-400">사주명리 & 베딕 점성학 통합 엔진</p>
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
          ) : activeTab === "saju" ? (
            <SajuTab sajuReport={sajuReport} />
          ) : activeTab === "vedic_charts" ? (
            <VedicChartsTab report={report!} />
          ) : activeTab === "strength" ? (
            <StrengthTab sajuReport={sajuReport} />
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
        </AnimatePresence>
      </main>
      <Analytics />
    </div>
  );
}

export default App;
