import { create } from 'zustand';
import type { BirthData, TabId, VedicAnalysisResult, SajuAnalysisResult, TransitResult } from '../types';
import type { AnalysisBundleState, AnalysisTaskState, CompatibilityOutput } from '../types/analysis';

export const DEFAULT_BIRTH: BirthData = {
  year: 1990, month: 1, day: 1, hour: 12, minute: 0,
  lat: 37.5665, lon: 126.978,
  is_lunar: false, is_leap_month: false,
  timezone: "Asia/Seoul",
  unknown_time: false,
};

export const DEFAULT_BIRTH2: BirthData = {
  year: 1990, month: 6, day: 15, hour: 10, minute: 0,
  lat: 37.5665, lon: 126.978,
  is_lunar: false, is_leap_month: false,
  timezone: "Asia/Seoul",
  unknown_time: false,
};

const INITIAL_TASK_STATE: AnalysisTaskState<any> = {
  status: "idle",
  data: null,
  error: null,
};

const INITIAL_ANALYSIS_STATE: AnalysisBundleState = {
  vedic: { ...INITIAL_TASK_STATE },
  saju: { ...INITIAL_TASK_STATE },
  transit: { ...INITIAL_TASK_STATE },
  aiAudit: { ...INITIAL_TASK_STATE },
  tier: { ...INITIAL_TASK_STATE },
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
  
  // Analysis Bundle (New)
  analysisState: AnalysisBundleState;
  setAnalysisTaskState: <K extends keyof AnalysisBundleState>(
    key: K,
    patch: Partial<AnalysisBundleState[K]>
  ) => void;
  resetAnalysisState: () => void;

  // Legacy Analysis State (maintained for compatibility)
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
  tierReport: any;
  setTierReport: (report: any) => void;
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
  compReport: CompatibilityOutput | null;
  setCompReport: (report: CompatibilityOutput | null) => void;
  compLoading: boolean;
  setCompLoading: (loading: boolean) => void;
}

export const useAppStore = create<AppState>((set) => ({
  // UI State
  activeTab: 'saju',
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

  // New Analysis State
  analysisState: INITIAL_ANALYSIS_STATE,
  setAnalysisTaskState: (key, patch) => set((state) => ({
    analysisState: {
      ...state.analysisState,
      [key]: { ...state.analysisState[key], ...patch }
    }
  })),
  resetAnalysisState: () => set({ analysisState: INITIAL_ANALYSIS_STATE }),

  // Legacy Analysis
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
