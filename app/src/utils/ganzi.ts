import { STEM_INFO, BRANCH_INFO } from "../constants";
import type { GanZi } from "../types";

/** GanZi 객체 → "甲子" 한자 형식 */
export function ganziDisplay(gz: GanZi | null | undefined): string {
  if (!gz) return "—";
  const s = STEM_INFO[gz.stem];
  const b = BRANCH_INFO[gz.branch];
  if (!s || !b) return "—";
  return `${s.hanja}${b.hanja}`;
}

/** GanZi 객체 → "갑자" 한글 형식 */
export function ganziHangul(gz: GanZi | null | undefined): string {
  if (!gz) return "";
  const s = STEM_INFO[gz.stem];
  const b = BRANCH_INFO[gz.branch];
  return `${s?.hangul ?? ""}${b?.hangul ?? ""}`;
}
