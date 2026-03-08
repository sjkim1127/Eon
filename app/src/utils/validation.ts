import type { BirthData } from "../types";

const inRange = (value: number, min: number, max: number) => Number.isFinite(value) && value >= min && value <= max;

export const getBirthValidationError = (value: BirthData, label: string): string | null => {
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
