import { create } from 'zustand';
import type { BirthData, TabId, VedicAnalysisResult, SajuAnalysisResult, TransitResult, CompatibilityAudit, AshtaKutaResult } from '../types';
import type { TierResult } from '../utils/tierScore';

export const DEFAULT_BIRTH: BirthData = {
  year: 1990, month: 1, day: 1, hour: 12, minute: 0,
  lat: 37.5665, lon: 126.978,
  is_lunar: false, is_leap_month: false,
  timezone: "Asia/Seoul",
};

export const DEFAULT_BIRTH2: BirthData = {
  year: 1990, month: 6, day: 15, hour: 10, minute: 0,
  lat: 37.5665, lon: 126.978,
  is_lunar: false, is_leap_month: false,
  timezone: "Asia/Seoul",
};

interface AppState {
  // UI State
  activeTab: TabId;
  setActiveTab: (tab: TabId) => void;
  errorMessage: string | null;
  setErrorMessage: (msg: string | null) => void;

  // Person 1 State
  birthData: BirthData;
  setBirthData: (updater: BirthData | ((prev: BirthData) => BirthData)) => void;
  selectedCity: string;
  setSelectedCity: (city: string) => void;
  isMale: boolean;
  setIsMale: (val: boolean) => void;
  
  // Person 1 Analysis State
  report: VedicAnalysisResult | null;
  setReport: (report: VedicAnalysisResult | null) => void;
  sajuReport: SajuAnalysisResult | null;
  setSajuReport: (report: SajuAnalysisResult | null) => void;
  aiAuditReport: any;
  setAiAuditReport: (report: any) => void;
  transitReport: TransitResult | null;
  setTransitReport: (report: TransitResult | null) => void;
  transitError: string | null;
  setTransitError: (err: string | null) => void;
  tierReport: TierResult | null;
  setTierReport: (report: TierResult | null) => void;
  loading: boolean;
  setLoading: (loading: boolean) => void;

  // Person 2 State (Compatibility)
  birthData2: BirthData;
  setBirthData2: (updater: BirthData | ((prev: BirthData) => BirthData)) => void;
  selectedCity2: string;
  setSelectedCity2: (city: string) => void;
  isMale2: boolean;
  setIsMale2: (val: boolean) => void;

  // Compatibility Analysis State
  compReport: { saju: CompatibilityAudit; vedic: AshtaKutaResult } | null;
  setCompReport: (report: { saju: CompatibilityAudit; vedic: AshtaKutaResult } | null) => void;
  compLoading: boolean;
  setCompLoading: (loading: boolean) => void;
}

export const useAppStore = create<AppState>((set) => ({
  // UI State
  activeTab: 'overview',
  setActiveTab: (tab) => set({ activeTab: tab }),
  errorMessage: null,
  setErrorMessage: (msg) => set({ errorMessage: msg }),

  // Person 1
  birthData: DEFAULT_BIRTH,
  setBirthData: (updater) => set((state) => ({
    birthData: typeof updater === 'function' ? updater(state.birthData) : updater
  })),
  selectedCity: "서울",
  setSelectedCity: (city) => set({ selectedCity: city }),
  isMale: true,
  setIsMale: (val) => set({ isMale: val }),

  // Person 1 Analysis
  report: null,
  setReport: (report) => set({ report }),
  sajuReport: null,
  setSajuReport: (sajuReport) => set({ sajuReport }),
  aiAuditReport: null,
  setAiAuditReport: (aiAuditReport) => set({ aiAuditReport }),
  transitReport: null,
  setTransitReport: (transitReport) => set({ transitReport }),
  transitError: null,
  setTransitError: (transitError) => set({ transitError }),
  tierReport: null,
  setTierReport: (tierReport) => set({ tierReport }),
  loading: false,
  setLoading: (loading) => set({ loading }),

  // Person 2
  birthData2: DEFAULT_BIRTH2,
  setBirthData2: (updater) => set((state) => ({
    birthData2: typeof updater === 'function' ? updater(state.birthData2) : updater
  })),
  selectedCity2: "서울",
  setSelectedCity2: (city) => set({ selectedCity2: city }),
  isMale2: false,
  setIsMale2: (val) => set({ isMale2: val }),

  // Compatibility Analysis
  compReport: null,
  setCompReport: (compReport) => set({ compReport }),
  compLoading: false,
  setCompLoading: (compLoading) => set({ compLoading }),
}));
