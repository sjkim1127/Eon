import type { NakshatraEntry, VargaDef } from "../types";

// ─── 베딕 정적 데이터 ────────────────────────────────────────────────────────

export const NAKSHATRA_DATA: NakshatraEntry[] = [
  { name: "Ashwini", lord: "Ketu", deity: "Ashwini Kumara", start_deg: 0, purpose: "Dharma" },
  { name: "Bharani", lord: "Venus", deity: "Yama", start_deg: 13.333, purpose: "Artha" },
  { name: "Krittika", lord: "Sun", deity: "Agni", start_deg: 26.667, purpose: "Kama" },
  { name: "Rohini", lord: "Moon", deity: "Brahma", start_deg: 40, purpose: "Moksha" },
  { name: "Mrigashira", lord: "Mars", deity: "Chandra", start_deg: 53.333, purpose: "Moksha" },
  { name: "Ardra", lord: "Rahu", deity: "Rudra", start_deg: 66.667, purpose: "Kama" },
  { name: "Punarvasu", lord: "Jupiter", deity: "Aditi", start_deg: 80, purpose: "Artha" },
  { name: "Pushya", lord: "Saturn", deity: "Brihaspati", start_deg: 93.333, purpose: "Dharma" },
  { name: "Ashlesha", lord: "Mercury", deity: "Nagas", start_deg: 106.667, purpose: "Dharma" },
  { name: "Magha", lord: "Ketu", deity: "Pitrs", start_deg: 120, purpose: "Artha" },
  { name: "Purva Phalguni", lord: "Venus", deity: "Bhaga", start_deg: 133.333, purpose: "Kama" },
  { name: "Uttara Phalguni", lord: "Sun", deity: "Aryaman", start_deg: 146.667, purpose: "Moksha" },
  { name: "Hasta", lord: "Moon", deity: "Savitr", start_deg: 160, purpose: "Moksha" },
  { name: "Chitra", lord: "Mars", deity: "Tvastr", start_deg: 173.333, purpose: "Kama" },
  { name: "Swati", lord: "Rahu", deity: "Vayu", start_deg: 186.667, purpose: "Artha" },
  { name: "Vishakha", lord: "Jupiter", deity: "Indra-Agni", start_deg: 200, purpose: "Dharma" },
  { name: "Anuradha", lord: "Saturn", deity: "Mitra", start_deg: 213.333, purpose: "Dharma" },
  { name: "Jyeshtha", lord: "Mercury", deity: "Indra", start_deg: 226.667, purpose: "Artha" },
  { name: "Mula", lord: "Ketu", deity: "Nritti", start_deg: 240, purpose: "Kama" },
  { name: "Purva Ashadha", lord: "Venus", deity: "Apas", start_deg: 253.333, purpose: "Moksha" },
  { name: "Uttara Ashadha", lord: "Sun", deity: "Vishvedevas", start_deg: 266.667, purpose: "Moksha" },
  { name: "Shravana", lord: "Moon", deity: "Vishnu", start_deg: 280, purpose: "Kama" },
  { name: "Dhanishta", lord: "Mars", deity: "Vasus", start_deg: 293.333, purpose: "Dharma" },  // Note: some sources say Artha
  { name: "Shatabhisha", lord: "Rahu", deity: "Varuna", start_deg: 306.667, purpose: "Dharma" },
  { name: "Purva Bhadrapada", lord: "Jupiter", deity: "Ajaikapada", start_deg: 320, purpose: "Artha" },
  { name: "Uttara Bhadrapada", lord: "Saturn", deity: "Ahirbudhnya", start_deg: 333.333, purpose: "Artha" },  // Note: some sources say Kama
  { name: "Revati", lord: "Mercury", deity: "Pushan", start_deg: 346.667, purpose: "Kama" },   // Note: some sources say Moksha
];

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
  { id: "hora", label: "D2", name: "Hora (재물)", key: "hora_rasi", divisionCount: 2 },
  { id: "drekkana", label: "D3", name: "Drekkana (형제)", key: "drekkana_rasi", divisionCount: 3 },
  { id: "chaturthamsha", label: "D4", name: "Chaturthamsha (부동산)", key: "chaturthamsha_rasi", divisionCount: 4 },
  { id: "panchamsa", label: "D5", name: "Panchamsa", key: "panchamsa_rasi", divisionCount: 5 },
  { id: "shashtamsa", label: "D6", name: "Shashtamsa (질병/적)", key: "shashtamsa_rasi", divisionCount: 6 },
  { id: "saptamsa", label: "D7", name: "Saptamsa (자녀)", key: "saptamsa_rasi", divisionCount: 7 },
  { id: "ashtamsa", label: "D8", name: "Ashtamsa", key: "ashtamsa_rasi", divisionCount: 8 },
  { id: "navamsa", label: "D9", name: "Navamsa (영혼/결혼)", key: "navamsa_rasi", divisionCount: 9 },
  { id: "dasamsa", label: "D10", name: "Dasamsa (직업)", key: "dasamsa_rasi", divisionCount: 10 },
  { id: "rudramsa", label: "D11", name: "Rudramsa", key: "rudramsa_rasi", divisionCount: 11 },
  { id: "dwadasamsa", label: "D12", name: "Dwadasamsa (부모)", key: "dwadasamsa_rasi", divisionCount: 12 },
  { id: "shodashamsa", label: "D16", name: "Shodashamsa (교통)", key: "shodashamsa_rasi", divisionCount: 16 },
  { id: "vimsamsa", label: "D20", name: "Vimsamsa (영적)", key: "vimsamsa_rasi", divisionCount: 20 },
  { id: "chaturvimshamsa", label: "D24", name: "Chaturvimshamsa (교육)", key: "chaturvimshamsa_rasi", divisionCount: 24 },
  { id: "saptavimsamsa", label: "D27", name: "Saptavimsamsa (체력)", key: "saptavimsamsa_rasi", divisionCount: 27 },
  { id: "trimsamsa", label: "D30", name: "Trimsamsa (재앙)", key: "trimsamsa_rasi", divisionCount: 30 },
  { id: "khavedamsa", label: "D40", name: "Khavedamsa", key: "khavedamsa_rasi", divisionCount: 40 },
  { id: "akshavedamsa", label: "D45", name: "Akshavedamsa", key: "akshavedamsa_rasi", divisionCount: 45 },
  { id: "shashtyamsa", label: "D60", name: "Shashtyamsa (카르마)", key: "shashtyamsa_rasi", divisionCount: 60 },
  { id: "navanavamsa", label: "D81", name: "Navanavamsa (D9 of D9)", key: "navanavamsa_rasi", divisionCount: 81 },
  { id: "ashtottaramsa", label: "D108", name: "Ashtottaramsa (D9 of D12)", key: "ashtottaramsa_rasi", divisionCount: 108 },
  { id: "dwadasdwadasamsa", label: "D144", name: "Dwadasdwadasamsa (D12 of D12)", key: "dwadasdwadasamsa_rasi", divisionCount: 144 },
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
