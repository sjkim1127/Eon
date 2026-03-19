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
  /** 음력 여부. 값이 참이면 음력으로 계산합니다. */
  is_lunar?: boolean;
  /** 윤달 여부. 음력 여부가 참일 때 이 값이 참이면 윤달로 계산합니다. */
  is_leap_month?: boolean;
  /** IANA 타임존 문자열 (예: "Asia/Seoul", "America/New_York") */
  timezone: string;
  /**
   * 야자시(夜子時) 적용 여부
   * - true: 23:00~23:59를 당일의 자시로 인정(일주 유지)
   * - false: 23:00~23:59를 다음날 자시로 간주(일주 변경, 기본값)
   */
  use_night_rat_hour?: boolean;
}

/** 한국 도시 */
export interface City {
  name: string;
  lat: number;
  lon: number;
  timezone: string;
}

/** 사이드바 탭 ID */
export type TabId =
  | "overview"
  | "saju"
  | "vedic_charts"
  | "strength"
  | "transit"
  | "compatibility"
  | "destiny_tier"
  | "ai_audit";



