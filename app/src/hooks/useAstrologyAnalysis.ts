import { toast } from "sonner";
import { backendClient } from "../lib/backend";
import { useAppStore } from "../store/useAppStore";
import { getBirthValidationError } from "../utils/validation";
import { normalizeVedicResult } from "../utils/normalize";
import type { SajuAnalysisResult, VedicAnalysisResult, TransitResult } from "../types";
import type { RunAnalysisResult } from "../types/analysis";

export function useAstrologyAnalysis() {
  const store = useAppStore();
  const { birthData, isMale } = store;

  const runAnalysis = async (): Promise<RunAnalysisResult> => {
    const error = getBirthValidationError(birthData, "출생 정보");
    if (error) {
      toast.error(error);
      return { ok: false, partial: false, completed: [], failed: [] };
    }

    store.resetAnalysisState();
    
    const now = new Date();
    const nowIso = now.toISOString();

    const commonArgs = {
      year: birthData.year,
      month: birthData.month,
      day: birthData.day,
      hour: birthData.hour,
      minute: birthData.minute,
      isLunar: birthData.isLunar ?? false,
      isLeapMonth: birthData.isLeapMonth ?? false,
      lat: birthData.lat,
      lon: birthData.lon,
      timezone: birthData.timezone,
      unknownTime: birthData.unknownTime ?? false,
    };

    const sajuArgs = {
      ...commonArgs,
      isMale: isMale,
      useNightRatHour: birthData.useNightRatHour ?? false,
    };

    const transitArgs = {
      ...sajuArgs,
      nowUtc: nowIso,
    };

    const completed: Array<any> = [];
    const failed: Array<any> = [];

    // 1. Core Parallel Tasks (Vedic, Saju, AI Audit)
    store.setAnalysisTaskState("vedic", { status: "loading" });
    store.setAnalysisTaskState("saju", { status: "loading" });

    store.setAnalysisTaskState("transit", { status: "loading" });

    const tasks = [
      { key: "vedic", fn: () => backendClient.getVedicAnalysis(commonArgs) },
      { key: "saju", fn: () => backendClient.getSajuAnalysis(sajuArgs) },

      { key: "transit", fn: () => backendClient.getTransitAnalysis(transitArgs) },
    ];

    const results = await Promise.allSettled(tasks.map(t => t.fn()));

    let sajuData: SajuAnalysisResult | null = null;
    let vedicData: VedicAnalysisResult | null = null;
    let transitData: TransitResult | null = null;

    results.forEach((res, idx) => {
      const key = tasks[idx].key as "vedic" | "saju" | "transit";
      
      if (res.status === "fulfilled") {
        const val = res.value;
        completed.push(key);
        
        switch (key) {
          case "vedic": {
            const data = normalizeVedicResult(val);
            vedicData = data;
            store.setAnalysisTaskState("vedic", { status: "success", data });
            break;
          }
          case "saju": {
            const data = val as SajuAnalysisResult;
            sajuData = data;
            store.setAnalysisTaskState("saju", { status: "success", data });
            break;
          }
          case "transit": {
            const data = val as TransitResult;
            transitData = data;
            store.setAnalysisTaskState("transit", { status: "success", data });
            break;
          }

        }
      } else {
        const errMsg = String(res.reason);
        store.setAnalysisTaskState(key, { status: "error", error: errMsg });
        failed.push(key);
      }
    });

    // 2. Dependent Task: Destiny Tier (Requires Saju & Vedic)
    if (sajuData && vedicData) {
      store.setAnalysisTaskState("tier", { status: "loading" });
      try {
        const tier = await backendClient.getDestinyTier(sajuData, vedicData, transitData);
        store.setAnalysisTaskState("tier", { status: "success", data: tier });
        completed.push("tier");
      } catch (e) {
        store.setAnalysisTaskState("tier", { status: "error", error: String(e) });
        failed.push("tier");
      }
    } else {
      store.setAnalysisTaskState("tier", { status: "error", error: "Required data (Saju/Vedic) missing" });
      failed.push("tier");
    }



    const ok = completed.length > 0;
    const partial = failed.length > 0;

    if (!ok) {
      toast.error("분석에 실패했습니다. 입력 정보를 확인해주세요.");
    } else if (partial) {
      toast.warning(`일부 분석(${failed.join(", ")})에 실패했습니다.`);
    } else {
      toast.success("분석이 완료되었습니다.");
    }

    return { ok, partial, completed, failed };
  };

  const loading = Object.values(store.analysisState).some(t => t.status === "loading");

  return {
    runAnalysis,
    analysisState: store.analysisState,
    loading,
  };
}
