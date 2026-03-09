import { toast } from "sonner";
import { backendClient } from "../lib/backend";
import { useAppStore } from "../store/useAppStore";
import { getBirthValidationError } from "../utils/validation";

export function useAstrologyAnalysis() {
  const store = useAppStore();

  const runAnalysis = async () => {
    const { birthData, isMale, setErrorMessage, setLoading, setReport, setSajuReport, setTransitReport, setAiAuditReport, setTransitError, setTierReport } = store;
    
    const now = new Date();
    const validationError = getBirthValidationError(birthData, "내 정보");
    if (validationError) {
      setErrorMessage(validationError);
      toast.error(validationError);
      return;
    }

    setLoading(true);
    setErrorMessage(null);
    setTransitError(null);
    try {
      const [vedicResult, sajuResult, transitResult, aiAuditResult] = await Promise.allSettled([
        backendClient.getVedicAnalysis({
          year: birthData.year, month: birthData.month, day: birthData.day,
          hour: birthData.hour, minute: birthData.minute,
          is_lunar: birthData.is_lunar ?? false, is_leap_month: birthData.is_leap_month ?? false,
          lat: birthData.lat, lon: birthData.lon,
          timezone: birthData.timezone,
        }),
        backendClient.getSajuAnalysis({
          year: birthData.year, month: birthData.month, day: birthData.day,
          hour: birthData.hour, minute: birthData.minute,
          is_lunar: birthData.is_lunar ?? false, is_leap_month: birthData.is_leap_month ?? false,
          is_male: isMale,
          use_night_rat_hour: birthData.use_night_rat_hour ?? false,
          lat: birthData.lat, lon: birthData.lon,
          timezone: birthData.timezone,
        }),
        backendClient.getTransitAnalysis({
          year: birthData.year, month: birthData.month, day: birthData.day,
          hour: birthData.hour, minute: birthData.minute,
          is_lunar: birthData.is_lunar ?? false, is_leap_month: birthData.is_leap_month ?? false,
          is_male: isMale,
          use_night_rat_hour: birthData.use_night_rat_hour ?? false,
          lat: birthData.lat, lon: birthData.lon,
          timezone: birthData.timezone,
          current_year: now.getFullYear(),
          current_month: now.getMonth() + 1,
          current_day: now.getDate(),
        }),
        backendClient.getAiAudit({
          year: birthData.year, month: birthData.month, day: birthData.day,
          hour: birthData.hour, minute: birthData.minute,
          is_lunar: birthData.is_lunar ?? false, is_leap_month: birthData.is_leap_month ?? false,
          is_male: isMale,
          use_night_rat_hour: birthData.use_night_rat_hour ?? false,
          lat: birthData.lat, lon: birthData.lon,
          timezone: birthData.timezone,
        }),
      ]);

      if (vedicResult.status === "fulfilled") {
        setReport(vedicResult.value);
      } else console.error("베딕 분석 실패:", vedicResult.reason);

      if (sajuResult.status === "fulfilled") setSajuReport(sajuResult.value);
      else console.error("사주 분석 실패:", sajuResult.reason);

      if (transitResult.status === "fulfilled") setTransitReport(transitResult.value);
      else {
        const errMsg = transitResult.reason instanceof Error ? transitResult.reason.message : String(transitResult.reason);
        console.error("운세 분석 실패:", transitResult.reason);
        setTransitError(errMsg);
      }

      if (aiAuditResult.status === "fulfilled") setAiAuditReport(aiAuditResult.value);
      else console.error("AI Audit 분석 실패:", aiAuditResult.reason);

      if (sajuResult.status === "fulfilled" && vedicResult.status === "fulfilled") {
        try {
          const tier = await backendClient.getDestinyTier(
            sajuResult.value,
            vedicResult.value,
            transitResult.status === "fulfilled" ? transitResult.value : null
          );
          setTierReport(tier);
        } catch (e) {
          console.error("Tier 분석 실패:", e);
        }
      }

      const allFailed = vedicResult.status === "rejected" && sajuResult.status === "rejected" && transitResult.status === "rejected";
      if (allFailed) {
        const message = "분석 중 오류가 발생했습니다.";
        setErrorMessage(message);
        toast.error("분석에 실패했습니다. 잠시 후 다시 시도해주세요.");
      } else {
        toast.success("분석이 완료되었습니다.");
      }
    } catch (e) {
      console.error(e);
      const message = e instanceof Error ? e.message : "분석 중 오류가 발생했습니다.";
      setErrorMessage(message);
      toast.error("분석에 실패했습니다. 잠시 후 다시 시도해주세요.");
    } finally {
      setLoading(false);
    }
  };

  return {
    runAnalysis,
    report: store.report,
    sajuReport: store.sajuReport,
    aiAuditReport: store.aiAuditReport,
    transitReport: store.transitReport,
    transitError: store.transitError,
    tierReport: store.tierReport,
    loading: store.loading,
  };
}
