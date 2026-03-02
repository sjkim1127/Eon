/** 출생 데이터 (lat/lon 포함) */
export interface BirthData {
  year: number;
  month: number;
  day: number;
  hour: number;
  minute: number;
  lat: number;
  lon: number;
  /** 태어난 시각을 모르는 경우. true이면 hour=12, minute=0 으로 대체하여 분석 */
  unknown_time?: boolean;
}

/** 한국 도시 */
export interface City {
  name: string;
  lat: number;
  lon: number;
}

/** 사이드바 탭 ID */
export type TabId =
  | "overview"
  | "saju"
  | "vedic_charts"
  | "strength"
  | "transit"
  | "compatibility";
