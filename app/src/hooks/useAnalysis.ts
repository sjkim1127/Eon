import { useState } from "react";
import { toast } from "sonner";
import {
  get_vedic_analysis,
  get_saju_analysis,
  get_transit_analysis,
  get_saju_compatibility,
  get_vedic_compatibility,
} from "../lib/api";
import { KOREAN_CITIES } from "../constants";
import { isKoreaDST } from "../utils";
import type {
  BirthData,
  TabId,
  VedicAnalysisResult,
  SajuAnalysisResult,
  TransitResult,
  CompatibilityAudit,
  AshtaKutaResult,
} from "../types";

/** 기본 출생 데이터 */
const DEFAULT_BIRTH: BirthData = {
  year: 1990, month: 1, day: 1, hour: 12, minute: 0,
  lat: 37.5665, lon: 126.978,
};

/** 상대방 기본 출생 데이터 */
const DEFAULT_BIRTH2: BirthData = {
  year: 1990, month: 6, day: 15, hour: 10, minute: 0,
  lat: 37.5665, lon: 126.978,
};

const inRange = (value: number, min: number, max: number) => Number.isFinite(value) && value >= min && value <= max;

const getBirthValidationError = (value: BirthData, label: string): string | null => {
  if (!inRange(value.year, 1900, 2100)) return `${label}의 출생 연도는 1900~2100 사이여야 합니다.`;
  if (!inRange(value.month, 1, 12)) return `${label}의 출생 월은 1~12 사이여야 합니다.`;
  if (!inRange(value.day, 1, 31)) return `${label}의 출생 일은 1~31 사이여야 합니다.`;
  if (!inRange(value.hour, 0, 23)) return `${label}의 출생 시는 0~23 사이여야 합니다.`;
  if (!inRange(value.minute, 0, 59)) return `${label}의 출생 분은 0~59 사이여야 합니다.`;
  if (!inRange(value.lat, -90, 90)) return `${label}의 위도 값이 올바르지 않습니다.`;
  if (!inRange(value.lon, -180, 180)) return `${label}의 경도 값이 올바르지 않습니다.`;

  const daysInMonth = new Date(value.year, value.month, 0).getDate();
  if (value.day > daysInMonth) {
    return `${label}의 날짜가 실제 달력과 맞지 않습니다. (${value.month}월은 최대 ${daysInMonth}일)`;
  }

  return null;
};

export function useAnalysis() {
  // ── 출생 정보 상태 ──
  const [birthData, setBirthData] = useState<BirthData>(DEFAULT_BIRTH);
  const [selectedCity, setSelectedCity] = useState("서울");
  const [isMale, setIsMale] = useState(true);

  // ── 분석 결과 ──
  const [report, setReport] = useState<VedicAnalysisResult | null>(null);
  const [sajuReport, setSajuReport] = useState<SajuAnalysisResult | null>(null);
  const [transitReport, setTransitReport] = useState<TransitResult | null>(null);
  const [transitError, setTransitError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<TabId>("overview");
  const [errorMessage, setErrorMessage] = useState<string | null>(null);

  // ── 궁합 상태 ──
  const [birthData2, setBirthData2] = useState<BirthData>(DEFAULT_BIRTH2);
  const [isMale2, setIsMale2] = useState(false);
  const [selectedCity2, setSelectedCity2] = useState("서울");
  const [compReport, setCompReport] = useState<{ saju: CompatibilityAudit; vedic: AshtaKutaResult } | null>(null);
  const [compLoading, setCompLoading] = useState(false);

  // ── 파생 값 ──
  const isDST = isKoreaDST(birthData.year, birthData.month);

  // ── 핸들러 ──
  const handleCityChange = (cityName: string) => {
    const city = KOREAN_CITIES.find((c) => c.name === cityName);
    if (city) {
      setSelectedCity(cityName);
      setBirthData((prev) => ({ ...prev, lat: city.lat, lon: city.lon }));
    }
  };

  const handleCityChange2 = (cityName: string) => {
    const city = KOREAN_CITIES.find((c) => c.name === cityName);
    if (city) {
      setSelectedCity2(cityName);
      setBirthData2((prev) => ({ ...prev, lat: city.lat, lon: city.lon }));
    }
  };

  const runAnalysis = async () => {
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
      // 세 분석을 독립적으로 실행 — 하나가 실패해도 나머지 결과는 유지
      const [vedicResult, sajuResult, transitResult] = await Promise.allSettled([
        get_vedic_analysis({ ...birthData, timezone: "Asia/Seoul" }),
        get_saju_analysis({
          ...birthData,
          is_male: isMale,
          timezone: "Asia/Seoul",
        }),
        get_transit_analysis({
          ...birthData,
          is_male: isMale,
          timezone: "Asia/Seoul",
          current_year: now.getFullYear(),
          current_month: now.getMonth() + 1,
        }),
      ]);

      if (vedicResult.status === "fulfilled") setReport(vedicResult.value);
      else console.error("베딕 분석 실패:", vedicResult.reason);

      if (sajuResult.status === "fulfilled") setSajuReport(sajuResult.value);
      else console.error("사주 분석 실패:", sajuResult.reason);

      if (transitResult.status === "fulfilled") setTransitReport(transitResult.value);
      else {
        const errMsg = transitResult.reason instanceof Error ? transitResult.reason.message : String(transitResult.reason);
        console.error("운세 분석 실패:", transitResult.reason);
        setTransitError(errMsg);
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

  const runCompatibilityAnalysis = async () => {
    const firstValidation = getBirthValidationError(birthData, "내 정보");
    if (firstValidation) {
      setErrorMessage(firstValidation);
      toast.error(firstValidation);
      return;
    }

    const secondValidation = getBirthValidationError(birthData2, "상대 정보");
    if (secondValidation) {
      setErrorMessage(secondValidation);
      toast.error(secondValidation);
      return;
    }

    setCompLoading(true);
    setErrorMessage(null);
    try {
      const [saju, vedic] = await Promise.all([
        get_saju_compatibility({
          year1: birthData.year, month1: birthData.month, day1: birthData.day,
          hour1: birthData.hour, minute1: birthData.minute,
          is_male1: isMale, lon1: birthData.lon, lat1: birthData.lat,
          year2: birthData2.year, month2: birthData2.month, day2: birthData2.day,
          hour2: birthData2.hour, minute2: birthData2.minute,
          is_male2: isMale2, lon2: birthData2.lon, lat2: birthData2.lat,
          timezone: "Asia/Seoul",
        }),
        get_vedic_compatibility({
          year1: birthData.year, month1: birthData.month, day1: birthData.day,
          hour1: birthData.hour, minute1: birthData.minute,
          lat1: birthData.lat, lon1: birthData.lon,
          year2: birthData2.year, month2: birthData2.month, day2: birthData2.day,
          hour2: birthData2.hour, minute2: birthData2.minute,
          lat2: birthData2.lat, lon2: birthData2.lon,
          timezone: "Asia/Seoul",
        }),
      ]);
      setCompReport({ saju, vedic });
      toast.success("궁합 분석이 완료되었습니다.");
    } catch (e) {
      console.error(e);
      const message = e instanceof Error ? e.message : "궁합 분석 중 오류가 발생했습니다.";
      setErrorMessage(message);
      toast.error("궁합 분석에 실패했습니다. 입력값을 확인해주세요.");
    } finally {
      setCompLoading(false);
    }
  };

  return {
    // 1차 출생 정보
    birthData, setBirthData,
    selectedCity, handleCityChange,
    isMale, setIsMale,
    isDST,
    // 분석 결과
    report, sajuReport, transitReport, transitError,
    loading, runAnalysis,
    // 탭
    activeTab, setActiveTab,
    errorMessage,
    // 궁합
    birthData2, setBirthData2,
    selectedCity2, handleCityChange2,
    isMale2, setIsMale2,
    compReport, compLoading,
    runCompatibilityAnalysis,
  };
}
