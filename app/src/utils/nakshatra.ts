import { NAKSHATRA_DATA, SIGN_NAMES, SIGN_LORDS, PURUSHARTHA } from "../constants";
import type { NakshatraInfo } from "../types";

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
  const purpose = PURUSHARTHA[(navamsaSign - 1) % 4];
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
