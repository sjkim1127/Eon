/** 한국 DST(서머타임) 적용 기간 체크 */
export function isKoreaDST(year: number, month: number): boolean {
  // 1948-1951, 1955-1960, 1987-1988년 여름(5-9월) 기간
  if (
    (year >= 1948 && year <= 1951) ||
    (year >= 1955 && year <= 1960) ||
    (year >= 1987 && year <= 1988)
  ) {
    return month >= 5 && month <= 9;
  }
  return false;
}
