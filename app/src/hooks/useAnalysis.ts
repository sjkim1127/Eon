import { useState } from "react";
import {
  get_vedic_analysis,
  get_saju_analysis,
  get_transit_analysis,
  get_saju_compatibility,
  get_vedic_compatibility,
} from "../lib/api";
import { KOREAN_CITIES } from "../constants";
import { isKoreaDST } from "../utils";
import type { BirthData, TabId, VedicAnalysisResult } from "../types";

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

export function useAnalysis() {
  // ── 출생 정보 상태 ──
  const [birthData, setBirthData] = useState<BirthData>(DEFAULT_BIRTH);
  const [selectedCity, setSelectedCity] = useState("서울");
  const [isMale, setIsMale] = useState(true);

  // ── 분석 결과 ──
  const [report, setReport] = useState<VedicAnalysisResult | null>(null);
  const [sajuReport, setSajuReport] = useState<any>(null);
  const [transitReport, setTransitReport] = useState<any>(null);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState<TabId>("overview");

  // ── 궁합 상태 ──
  const [birthData2, setBirthData2] = useState<BirthData>(DEFAULT_BIRTH2);
  const [isMale2, setIsMale2] = useState(false);
  const [selectedCity2, setSelectedCity2] = useState("서울");
  const [compReport, setCompReport] = useState<any>(null);
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
    setLoading(true);
    try {
      const [vedic, saju, transit] = await Promise.all([
        get_vedic_analysis({ ...birthData }),
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
      setReport(vedic);
      setSajuReport(saju);
      setTransitReport(transit);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const runCompatibilityAnalysis = async () => {
    setCompLoading(true);
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
        }),
      ]);
      setCompReport({ saju, vedic });
    } catch (e) {
      console.error(e);
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
    report, sajuReport, transitReport,
    loading, runAnalysis,
    // 탭
    activeTab, setActiveTab,
    // 궁합
    birthData2, setBirthData2,
    selectedCity2, handleCityChange2,
    isMale2, setIsMale2,
    compReport, compLoading,
    runCompatibilityAnalysis,
  };
}
