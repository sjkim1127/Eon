import { toast } from "sonner";
import { backendClient } from "../lib/backend";
import { useAppStore } from "../store/useAppStore";
import { getBirthValidationError } from "../utils/validation";
import type { RunAnalysisResult } from "../types/analysis";

export function useCompatibility() {
  const store = useAppStore();

  const runCompatibilityAnalysis = async (): Promise<RunAnalysisResult> => {
    const { birthData, birthData2, isMale, isMale2, setErrorMessage, setCompLoading, setCompReport } = store;

    const firstValidation = getBirthValidationError(birthData, "내 정보");
    if (firstValidation) {
      setErrorMessage(firstValidation);
      toast.error(firstValidation);
      return { ok: false, partial: false, completed: [], failed: [] };
    }

    const secondValidation = getBirthValidationError(birthData2, "상대 정보");
    if (secondValidation) {
      setErrorMessage(secondValidation);
      toast.error(secondValidation);
      return { ok: false, partial: false, completed: [], failed: [] };
    }

    setCompLoading(true);
    setErrorMessage(null);
    try {
      const [saju, vedic] = await Promise.all([
        backendClient.getSajuCompatibility({
          year1: birthData.year, month1: birthData.month, day1: birthData.day,
          hour1: birthData.hour, minute1: birthData.minute,
          is_lunar1: birthData.is_lunar ?? false, is_leap_month1: birthData.is_leap_month ?? false,
          is_male1: isMale, lon1: birthData.lon, lat1: birthData.lat,
          use_night_rat_hour1: birthData.use_night_rat_hour ?? false,
          year2: birthData2.year, month2: birthData2.month, day2: birthData2.day,
          hour2: birthData2.hour, minute2: birthData2.minute,
          is_lunar2: birthData2.is_lunar ?? false, is_leap_month2: birthData2.is_leap_month ?? false,
          is_male2: isMale2, lon2: birthData2.lon, lat2: birthData2.lat,
          use_night_rat_hour2: birthData2.use_night_rat_hour ?? false,
          timezone1: birthData.timezone,
          timezone2: birthData2.timezone,
        }),
        backendClient.getVedicCompatibility({
          year1: birthData.year, month1: birthData.month, day1: birthData.day,
          hour1: birthData.hour, minute1: birthData.minute,
          is_lunar1: birthData.is_lunar ?? false, is_leap_month1: birthData.is_leap_month ?? false,
          is_male1: isMale, lon1: birthData.lon, lat1: birthData.lat,
          year2: birthData2.year, month2: birthData2.month, day2: birthData2.day,
          hour2: birthData2.hour, minute2: birthData2.minute,
          is_lunar2: birthData2.is_lunar ?? false, is_leap_month2: birthData2.is_leap_month ?? false,
          is_male2: isMale2, lon2: birthData2.lon, lat2: birthData2.lat,
          timezone1: birthData.timezone,
          timezone2: birthData2.timezone,
        }),
      ]);
      setCompReport({ saju, vedic });
      toast.success("궁합 분석이 완료되었습니다.");
      setCompLoading(false);
      return { ok: true, partial: false, completed: [] as any, failed: [] };
    } catch (e) {
      console.error(e);
      const message = e instanceof Error ? e.message : "궁합 분석 중 오류가 발생했습니다.";
      setErrorMessage(message);
      toast.error("궁합 분석에 실패했습니다. 입력값을 확인해주세요.");
      setCompLoading(false);
      return { ok: false, partial: false, completed: [], failed: [] };
    }
  };

  return {
    runCompatibilityAnalysis,
    compReport: store.compReport,
    compLoading: store.compLoading,
  };
}
