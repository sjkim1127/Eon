/**
 * Vedic 낙샤트라·사이드리얼 위치 공통 포맷터
 *
 * UI(VedicChartsTab)와 마크다운 export(exportMarkdown.ts) 양쪽에서
 * 동일 함수를 사용하여 포맷 일관성을 보장합니다.
 */

import { SIGN_NAMES } from "../constants";

// ── 컬럼명 상수 (UI 테이블 헤더 & 마크다운 표 헤더 공통) ─────────────────

export const NAKSHATRA_TABLE_COLUMNS = {
  planet: "행성",
  position: "위치 (사이드리얼)",
  sign: "사인",
  house: "하우스",
  nakshatra: "낙샤트라 (파다)",
  padaRange: "파다 범위",
  nakshatraLord: "낙샤트라 로드",
  padaLord: "파다 로드",
  deity: "신 (Deity)",
  purpose: "목적 (Purpose)",
} as const;

/** D1 기본 8컬럼 (사인/하우스 제외) */
export const D1_COLUMNS = [
  NAKSHATRA_TABLE_COLUMNS.planet,
  NAKSHATRA_TABLE_COLUMNS.position,
  NAKSHATRA_TABLE_COLUMNS.nakshatra,
  NAKSHATRA_TABLE_COLUMNS.padaRange,
  NAKSHATRA_TABLE_COLUMNS.nakshatraLord,
  NAKSHATRA_TABLE_COLUMNS.padaLord,
  NAKSHATRA_TABLE_COLUMNS.deity,
  NAKSHATRA_TABLE_COLUMNS.purpose,
] as const;

/** Varga 확장 10컬럼 (사인·하우스 포함) */
export const VARGA_COLUMNS = [
  NAKSHATRA_TABLE_COLUMNS.planet,
  NAKSHATRA_TABLE_COLUMNS.position,
  NAKSHATRA_TABLE_COLUMNS.sign,
  NAKSHATRA_TABLE_COLUMNS.house,
  NAKSHATRA_TABLE_COLUMNS.nakshatra,
  NAKSHATRA_TABLE_COLUMNS.padaRange,
  NAKSHATRA_TABLE_COLUMNS.nakshatraLord,
  NAKSHATRA_TABLE_COLUMNS.padaLord,
  NAKSHATRA_TABLE_COLUMNS.deity,
  NAKSHATRA_TABLE_COLUMNS.purpose,
] as const;

// ── 포맷터 함수 ────────────────────────────────────────────────────────────

/**
 * 사이드리얼 경도(0–360) → "16°42' Sagittarius" 형식 문자열
 * Rust fmt_degree()와 동일 로직.
 */
export function formatSiderealPosition(sidereal_deg: number): string {
  const deg = ((sidereal_deg % 360) + 360) % 360;
  const sign = Math.floor(deg / 30) + 1;
  const d = deg % 30;
  const dd = Math.floor(d);
  const mm = Math.round((d - dd) * 60);
  return `${dd}°${String(mm).padStart(2, "0")}' ${SIGN_NAMES[sign] ?? "?"}`;
}

/**
 * 낙샤트라 이름 + 파다 → "Purva Ashadha (Pada 2)" 형식 문자열
 */
export function formatNakshatraWithPada(name: string, pada: number): string {
  return `${name} (Pada ${pada})`;
}

/**
 * 파다 범위 문자열 통과 함수 (향후 포맷 변환 대비 경유점)
 * 입력: Rust 또는 nakshatra.ts가 생성한 "16°40' Sagittarius – 19°53' Sagittarius" 형태
 */
export function formatPadaRange(range: string): string {
  return range;
}

// ── 마크다운 표 헬퍼 ───────────────────────────────────────────────────────

/**
 * VargaNakshatraReportRow 배열 → 마크다운 표 줄 배열 (헤더 포함)
 * showHouse=true 시 사인·하우스 컬럼 추가 (VARGA_COLUMNS 기준).
 */
export function buildNakshatraMarkdownRows(
  rows: {
    planet: string;
    positionStr: string;
    sign?: number;
    house?: number;
    nakshatraName: string;
    pada: number;
    padaRange: string;
    nakshatraLord: string;
    padaLord: string;
    deity: string;
    purpose: string;
    isRetrograde?: boolean;
    isCombust?: boolean;
  }[],
  showHouse = false,
): string[] {
  const lines: string[] = [];
  const head = showHouse 
    ? "| 행성 | 위치(사이드리얼) | 사인 | 하우스 | 낙샤트라(파다) | 파다 범위 | 낙샤트라 로드 | 파다 로드 | 신(Deity) | 목적 |" 
    : "| 행성 | 위치(사이드리얼) | 낙샤트라(파다) | 파다 범위 | 낙샤트라 로드 | 파다 로드 | 신(Deity) | 목적 |";
  const sep = showHouse 
    ? "|---|---|---|---|---|---|---|---|---|---|" 
    : "|---|---|---|---|---|---|---|---|";
  lines.push(head);
  lines.push(sep);

  for (const row of rows) {
    const flags = [row.isRetrograde ? "℞" : "", row.isCombust ? "☀" : ""]
      .filter(Boolean)
      .join(" ");
    const planet = flags ? `${row.planet} ${flags}` : row.planet;
    const signName = row.sign != null ? (SIGN_NAMES[row.sign] ?? "—") : "—";
    const houseStr = row.house != null ? `H${row.house}` : "—";

    if (showHouse) {
      lines.push(
        `| ${planet} | ${row.positionStr} | ${signName} | ${houseStr} | ${formatNakshatraWithPada(row.nakshatraName, row.pada)} | ${formatPadaRange(row.padaRange)} | ${row.nakshatraLord} | ${row.padaLord} | ${row.deity} | ${row.purpose} |`,
      );
    } else {
      lines.push(
        `| ${planet} | ${row.positionStr} | ${formatNakshatraWithPada(row.nakshatraName, row.pada)} | ${formatPadaRange(row.padaRange)} | ${row.nakshatraLord} | ${row.padaLord} | ${row.deity} | ${row.purpose} |`,
      );
    }
  }

  return lines;
}
