import { ReactNode } from "react";
import { UserPlus } from "lucide-react";
import { ShootingStars, Sidebar, CompactBirthInfoBar, ExportActionButtons, BirthDrawer } from "../shared";
import { TabId } from "../../types";

interface AppLayoutProps {
  currentTab: TabId;
  setActiveTab: (tab: TabId) => void;
  prefetchTab: (tab: TabId) => void;
  birthData: any;
  setBirthData: any;
  selectedCity: any;
  handleCitySelect: any;
  isMale: any;
  setIsMale: any;
  isDST: boolean;
  sajuData: any;
  vedicData: any;
  transitData: any;
  tierData: any;
  hasAnyReport: boolean;
  formOpen: boolean;
  setFormOpen: (open: boolean) => void;
  loading: boolean;
  runAnalysis: any;
  errorMessage: string | null;
  children: ReactNode;
}

export function AppLayout({
  currentTab, setActiveTab, prefetchTab,
  birthData, setBirthData, selectedCity, handleCitySelect, isMale, setIsMale,
  isDST, sajuData, vedicData, transitData, tierData,
  hasAnyReport, formOpen, setFormOpen, loading, runAnalysis, errorMessage,
  children
}: AppLayoutProps) {
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
              meta={sajuData?.meta}
              onEdit={() => setFormOpen(true)}
              actionSlot={
                <ExportActionButtons
                  sajuReport={sajuData}
                  report={vedicData}
                  transitReport={transitData}
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
            <span className="text-amber-400/80 ml-2">DST 적용됨</span>
          )}

          {errorMessage && (
            <p className="text-sm text-red-300/90 bg-red-500/10 border border-red-500/30 rounded-xl px-4 py-2 mt-2">
              {errorMessage}
            </p>
          )}
        </header>

        {children}
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
    </div>
  );
}
