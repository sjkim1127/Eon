import type { NakshatraEntry, VargaDef } from "../types";

// ─── 베딕 정적 데이터 ────────────────────────────────────────────────────────

export const NAKSHATRA_DATA: NakshatraEntry[] = [
  { name: "Ashwini", lord: "Ketu", deity: "Ashwini Kumara", startDeg: 0, purpose: "Dharma" },
  { name: "Bharani", lord: "Venus", deity: "Yama", startDeg: 13.333, purpose: "Artha" },
  { name: "Krittika", lord: "Sun", deity: "Agni", startDeg: 26.667, purpose: "Kama" },
  { name: "Rohini", lord: "Moon", deity: "Brahma", startDeg: 40, purpose: "Moksha" },
  { name: "Mrigashira", lord: "Mars", deity: "Chandra", startDeg: 53.333, purpose: "Moksha" },
  { name: "Ardra", lord: "Rahu", deity: "Rudra", startDeg: 66.667, purpose: "Kama" },
  { name: "Punarvasu", lord: "Jupiter", deity: "Aditi", startDeg: 80, purpose: "Artha" },
  { name: "Pushya", lord: "Saturn", deity: "Brihaspati", startDeg: 93.333, purpose: "Dharma" },
  { name: "Ashlesha", lord: "Mercury", deity: "Nagas", startDeg: 106.667, purpose: "Dharma" },
  { name: "Magha", lord: "Ketu", deity: "Pitrs", startDeg: 120, purpose: "Artha" },
  { name: "Purva Phalguni", lord: "Venus", deity: "Bhaga", startDeg: 133.333, purpose: "Kama" },
  { name: "Uttara Phalguni", lord: "Sun", deity: "Aryaman", startDeg: 146.667, purpose: "Moksha" },
  { name: "Hasta", lord: "Moon", deity: "Savitr", startDeg: 160, purpose: "Moksha" },
  { name: "Chitra", lord: "Mars", deity: "Tvastr", startDeg: 173.333, purpose: "Kama" },
  { name: "Swati", lord: "Rahu", deity: "Vayu", startDeg: 186.667, purpose: "Artha" },
  { name: "Vishakha", lord: "Jupiter", deity: "Indra-Agni", startDeg: 200, purpose: "Dharma" },
  { name: "Anuradha", lord: "Saturn", deity: "Mitra", startDeg: 213.333, purpose: "Dharma" },
  { name: "Jyeshtha", lord: "Mercury", deity: "Indra", startDeg: 226.667, purpose: "Artha" },
  { name: "Mula", lord: "Ketu", deity: "Nritti", startDeg: 240, purpose: "Kama" },
  { name: "Purva Ashadha", lord: "Venus", deity: "Apas", startDeg: 253.333, purpose: "Moksha" },
  { name: "Uttara Ashadha", lord: "Sun", deity: "Vishvedevas", startDeg: 266.667, purpose: "Moksha" },
  { name: "Shravana", lord: "Moon", deity: "Vishnu", startDeg: 280, purpose: "Kama" },
  { name: "Dhanishta", lord: "Mars", deity: "Vasus", startDeg: 293.333, purpose: "Dharma" },
  { name: "Shatabhisha", lord: "Rahu", deity: "Varuna", startDeg: 306.667, purpose: "Dharma" },
  { name: "Purva Bhadrapada", lord: "Jupiter", deity: "Ajaikapada", startDeg: 320, purpose: "Artha" },
  { name: "Uttara Bhadrapada", lord: "Saturn", deity: "Ahirbudhnya", startDeg: 333.333, purpose: "Artha" },
  { name: "Revati", lord: "Mercury", deity: "Pushan", startDeg: 346.667, purpose: "Kama" },];

export const SIGN_NAMES = [
  "", "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
  "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces",
];

export const SIGN_LORDS = [
  "", "Mars", "Venus", "Mercury", "Moon", "Sun", "Mercury",
  "Venus", "Mars", "Jupiter", "Saturn", "Saturn", "Jupiter",
];

export const PURUSHARTHA = ["Dharma", "Artha", "Kama", "Moksha"];

export const VARGA_DEFS: VargaDef[] = [
  { id: "rasi", label: "D1", name: "Rasi (원본 차트)", key: "rasi", divisionCount: 1 },
  { id: "hora", label: "D2", name: "Hora (재물)", key: "horaRasi", divisionCount: 2 },
  { id: "drekkana", label: "D3", name: "Drekkana (형제)", key: "drekkanaRasi", divisionCount: 3 },
  { id: "chaturthamsha", label: "D4", name: "Chaturthamsha (부동산)", key: "chaturthamshaRasi", divisionCount: 4 },
  { id: "panchamsa", label: "D5", name: "Panchamsa", key: "panchamsaRasi", divisionCount: 5 },
  { id: "shashtamsa", label: "D6", name: "Shashtamsa (질병/적)", key: "shashtamsaRasi", divisionCount: 6 },
  { id: "saptamsa", label: "D7", name: "Saptamsa (자녀)", key: "saptamsaRasi", divisionCount: 7 },
  { id: "ashtamsa", label: "D8", name: "Ashtamsa", key: "ashtamsaRasi", divisionCount: 8 },
  { id: "navamsa", label: "D9", name: "Navamsa (영혼/결혼)", key: "navamsaRasi", divisionCount: 9 },
  { id: "dasamsa", label: "D10", name: "Dasamsa (직업)", key: "dasamsaRasi", divisionCount: 10 },
  { id: "rudramsa", label: "D11", name: "Rudramsa", key: "rudramsaRasi", divisionCount: 11 },
  { id: "dwadasamsa", label: "D12", name: "Dwadasamsa (부모)", key: "dwadasamsaRasi", divisionCount: 12 },
  { id: "shodashamsa", label: "D16", name: "Shodashamsa (교통)", key: "shodashamsaRasi", divisionCount: 16 },
  { id: "vimsamsa", label: "D20", name: "Vimsamsa (영적)", key: "vimsamsaRasi", divisionCount: 20 },
  { id: "chaturvimshamsa", label: "D24", name: "Chaturvimshamsa (교육)", key: "chaturvimshamsaRasi", divisionCount: 24 },
  { id: "saptavimsamsa", label: "D27", name: "Saptavimsamsa (체력)", key: "saptavimsamsaRasi", divisionCount: 27 },
  { id: "trimsamsa", label: "D30", name: "Trimsamsa (재앙)", key: "trimsamsaRasi", divisionCount: 30 },
  { id: "khavedamsa", label: "D40", name: "Khavedamsa", key: "khavedamsaRasi", divisionCount: 40 },
  { id: "akshavedamsa", label: "D45", name: "Akshavedamsa", key: "akshavedamsaRasi", divisionCount: 45 },
  { id: "shashtyamsa", label: "D60", name: "Shashtyamsa (카르마)", key: "shashtyamsaRasi", divisionCount: 60 },
  { id: "navanavamsa", label: "D81", name: "Navanavamsa (D9 of D9)", key: "navanavamsaRasi", divisionCount: 81 },
  { id: "ashtottaramsa", label: "D108", name: "Ashtottaramsa (D9 of D12)", key: "ashtottaramsaRasi", divisionCount: 108 },
  { id: "dwadasdwadasamsa", label: "D144", name: "Dwadasdwadasamsa (D12 of D12)", key: "dwadasdwadasamsaRasi", divisionCount: 144 },
];

/** Ashta Kuta 항목 라벨 */
export const ASHTA_LABELS: Record<string, string> = {
  varna: "바르나 (계층)",
  vashya: "바쉬야 (지배)",
  tara: "타라 (별자리)",
  yoni: "요니 (본능)",
  maitri: "마이트리 (우정)",
  gana: "가나 (기질)",
  bhakoot: "바쿠트 (운명)",
  nadi: "나디 (신체)",
};

/** Ashta Kuta 항목별 최대 점수 */
export const ASHTA_MAX: Record<string, number> = {
  varna: 1, vashya: 2, tara: 3, yoni: 4, maitri: 5, gana: 6, bhakoot: 7, nadi: 8,
};
