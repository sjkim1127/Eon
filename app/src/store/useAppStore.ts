import { create } from 'zustand';
import type { BirthData, TabId } from '../types';
import type { AnalysisBundleState, AnalysisTaskState } from '../types/analysis';

export const DEFAULT_BIRTH: BirthData = {
  year: 1990, month: 1, day: 1, hour: 12, minute: 0,
  lat: 37.5665, lon: 126.978,
  isLunar: false, isLeapMonth: false,
  timezone: "Asia/Seoul",
  unknownTime: false,
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


}));
