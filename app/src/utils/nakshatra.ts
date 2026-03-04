import { NAKSHATRA_DATA, SIGN_NAMES, SIGN_LORDS } from "../constants";
import type { NakshatraInfo } from "../types";

/**
 * 바르가별 유효 경도 (낙샤트라 계산용).
 * D1 경도가 파생 차트에서 어떤 사인에, 해당 사인 내 몇 도에 대응하는지 계산.
 * @param sidereal_deg D1 사이드리얼 경도 (0–360)
 * @param varga_rasi 파생 차트에서의 사인 번호 (1–12)
 * @param divisionCount 바르가 분할 수 (D3=3, D9=9 등)
 */
export function getVargaEffectiveLongitude(
  sidereal_deg: number,
  varga_rasi: number,
  divisionCount: number
): number {
  if (divisionCount <= 1) return ((sidereal_deg % 360) + 360) % 360;
  const deg = ((sidereal_deg % 360) + 360) % 360;
  const signDegree = deg % 30;
  const divisionSize = 30 / divisionCount;
  const degreeInDivision = signDegree % divisionSize;
  const scaledDegree = degreeInDivision * divisionCount;
  return (varga_rasi - 1) * 30 + scaledDegree;
}

/** 사이드리얼 degree → 낙샤트라 정보 */
export function getNakshatraInfo(sidereal_deg: number): NakshatraInfo {
  const deg = ((sidereal_deg % 360) + 360) % 360;
  const idx = Math.floor(deg / (360 / 27));
  const nak = NAKSHATRA_DATA[idx];
  const degInNak = deg - nak.start_deg;
  const pada = Math.floor(degInNak / (13.333 / 4)) + 1;
  const clampedPada = Math.min(4, Math.max(1, pada));
  const navamsaSign = ((idx * 4 + (clampedPada - 1)) % 12) + 1;
  const padaLord = SIGN_LORDS[navamsaSign];
  const purpose = nak.purpose; // BPHS nakshatra-level purpose
  const padaStartDeg = nak.start_deg + (clampedPada - 1) * (13.333 / 4);
  const padaEndDeg = padaStartDeg + (13.333 / 4);

  const fmtDeg = (d: number) => {
    const total = ((d % 360) + 360) % 360;
    const sign = Math.floor(total / 30) + 1;
    const degInSign = total % 30;
    const dd = Math.floor(degInSign);
    const mm = Math.round((degInSign - dd) * 60);
    return `${dd}°${String(mm).padStart(2, "0")}' ${SIGN_NAMES[sign]}`;
  };

  return {
    name: nak.name,
    pada: clampedPada,
    lord: nak.lord,
    padaLord,
    deity: nak.deity,
    purpose,
    range: `${fmtDeg(padaStartDeg)} – ${fmtDeg(padaEndDeg)}`,
  };
}
