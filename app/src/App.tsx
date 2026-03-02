import { useState } from "react";
import { get_vedic_analysis, get_saju_analysis, get_transit_analysis, get_saju_compatibility, get_vedic_compatibility } from "./lib/api";
import { Analytics } from "@vercel/analytics/react";
import { motion, AnimatePresence } from "framer-motion";
import {
  Sparkles,
  Sun,
  Star,
  Compass,
  Clock,
  Calendar,
  LayoutDashboard,
  Zap,
  Shield,
  Heart,
  User,
  TrendingUp,
  Activity,
  MapPin,
  Copy,
  Check,
} from "lucide-react";
import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

// 한국 주요 도시 데이터 (eon-core::Location과 동일)
const KOREAN_CITIES = [
  { name: "서울", lat: 37.5665, lon: 126.9780 },
  { name: "안산", lat: 37.3219, lon: 126.8309 },
  { name: "인천", lat: 37.4563, lon: 126.7052 },
  { name: "부산", lat: 35.1796, lon: 129.0756 },
  { name: "대구", lat: 35.8714, lon: 128.6014 },
  { name: "대전", lat: 36.3504, lon: 127.3845 },
  { name: "광주", lat: 35.1595, lon: 126.8526 },
  { name: "제주", lat: 33.4996, lon: 126.5312 },
  { name: "울산", lat: 35.5384, lon: 129.3114 },
  { name: "수원", lat: 37.2636, lon: 127.0286 },
];

// ─── 베딕 정적 데이터 ────────────────────────────────────────────────────────
const NAKSHATRA_DATA: Array<{
  name: string;
  lord: string;
  deity: string;
  start_deg: number; // 황도 기준 시작도 (사이드리얼)
}> = [
  { name: "Ashwini",          lord: "Ketu",    deity: "Ashwini Kumara",  start_deg: 0 },
  { name: "Bharani",          lord: "Venus",   deity: "Yama",            start_deg: 13.333 },
  { name: "Krittika",         lord: "Sun",     deity: "Agni",            start_deg: 26.667 },
  { name: "Rohini",           lord: "Moon",    deity: "Brahma",          start_deg: 40 },
  { name: "Mrigashira",       lord: "Mars",    deity: "Chandra",         start_deg: 53.333 },
  { name: "Ardra",            lord: "Rahu",    deity: "Rudra",           start_deg: 66.667 },
  { name: "Punarvasu",        lord: "Jupiter", deity: "Aditi",           start_deg: 80 },
  { name: "Pushya",           lord: "Saturn",  deity: "Brihaspati",      start_deg: 93.333 },
  { name: "Ashlesha",         lord: "Mercury", deity: "Nagas",           start_deg: 106.667 },
  { name: "Magha",            lord: "Ketu",    deity: "Pitrs",           start_deg: 120 },
  { name: "Purva Phalguni",   lord: "Venus",   deity: "Bhaga",           start_deg: 133.333 },
  { name: "Uttara Phalguni",  lord: "Sun",     deity: "Aryaman",         start_deg: 146.667 },
  { name: "Hasta",            lord: "Moon",    deity: "Savitr",          start_deg: 160 },
  { name: "Chitra",           lord: "Mars",    deity: "Tvastr",          start_deg: 173.333 },
  { name: "Swati",            lord: "Rahu",    deity: "Vayu",            start_deg: 186.667 },
  { name: "Vishakha",         lord: "Jupiter", deity: "Indra-Agni",      start_deg: 200 },
  { name: "Anuradha",         lord: "Saturn",  deity: "Mitra",           start_deg: 213.333 },
  { name: "Jyeshtha",         lord: "Mercury", deity: "Indra",           start_deg: 226.667 },
  { name: "Mula",             lord: "Ketu",    deity: "Nritti",          start_deg: 240 },
  { name: "Purva Ashadha",    lord: "Venus",   deity: "Apas",            start_deg: 253.333 },
  { name: "Uttara Ashadha",   lord: "Sun",     deity: "Vishvedevas",     start_deg: 266.667 },
  { name: "Shravana",         lord: "Moon",    deity: "Vishnu",          start_deg: 280 },
  { name: "Dhanishta",        lord: "Mars",    deity: "Vasus",           start_deg: 293.333 },
  { name: "Shatabhisha",      lord: "Rahu",    deity: "Varuna",          start_deg: 306.667 },
  { name: "Purva Bhadrapada", lord: "Jupiter", deity: "Ajaikapada",      start_deg: 320 },
  { name: "Uttara Bhadrapada",lord: "Saturn",  deity: "Ahirbudhnya",     start_deg: 333.333 },
  { name: "Revati",           lord: "Mercury", deity: "Pushan",          start_deg: 346.667 },
];

const SIGN_NAMES = ["", "Aries", "Taurus", "Gemini", "Cancer", "Leo", "Virgo",
                       "Libra", "Scorpio", "Sagittarius", "Capricorn", "Aquarius", "Pisces"];
const SIGN_LORDS = ["", "Mars", "Venus", "Mercury", "Moon", "Sun", "Mercury",
                       "Venus", "Mars", "Jupiter", "Saturn", "Saturn", "Jupiter"];
const PURUSHARTHA = ["Dharma", "Artha", "Kama", "Moksha"];

// 사이드리얼 degree → 낙샤트라 정보
function getNakshatraInfo(sidereal_deg: number) {
  const deg = ((sidereal_deg % 360) + 360) % 360;
  const idx = Math.floor(deg / (360 / 27));          // 0-based nakshatra index
  const nak = NAKSHATRA_DATA[idx];
  const degInNak = deg - nak.start_deg;
  const pada = Math.floor(degInNak / (13.333 / 4)) + 1; // 1-4
  const clampedPada = Math.min(4, Math.max(1, pada));
  // navamsa sign: 낙샤트라 i의 pada p → navamsa = (i*4 + p-1) % 12
  const navamsaSign = ((idx * 4 + (clampedPada - 1)) % 12) + 1;
  const padaLord = SIGN_LORDS[navamsaSign];
  const purpose = PURUSHARTHA[(navamsaSign - 1) % 4];
  // pada 범위 표시용
  const padaStartDeg = nak.start_deg + (clampedPada - 1) * (13.333 / 4);
  const padaEndDeg   = padaStartDeg + (13.333 / 4);
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

const VARGA_DEFS = [
  { id: "rasi",              label: "D1",  name: "Rasi (원본 차트)",       key: "rasi" },
  { id: "hora",              label: "D2",  name: "Hora (재물)",           key: "hora_rasi" },
  { id: "drekkana",          label: "D3",  name: "Drekkana (형제)",       key: "drekkana_rasi" },
  { id: "chaturthamsha",     label: "D4",  name: "Chaturthamsha (부동산)",key: "chaturthamsha_rasi" },
  { id: "panchamsa",         label: "D5",  name: "Panchamsa",             key: "panchamsa_rasi" },
  { id: "saptamsa",          label: "D7",  name: "Saptamsa (자녀)",       key: "saptamsa_rasi" },
  { id: "ashtamsa",          label: "D8",  name: "Ashtamsa",              key: "ashtamsa_rasi" },
  { id: "navamsa",           label: "D9",  name: "Navamsa (영혼/결혼)",   key: "navamsa_rasi" },
  { id: "dasamsa",           label: "D10", name: "Dasamsa (직업)",        key: "dasamsa_rasi" },
  { id: "rudramsa",          label: "D11", name: "Rudramsa",              key: "rudramsa_rasi" },
  { id: "dwadasamsa",        label: "D12", name: "Dwadasamsa (부모)",     key: "dwadasamsa_rasi" },
  { id: "shodashamsa",       label: "D16", name: "Shodashamsa (교통)",    key: "shodashamsa_rasi" },
  { id: "vimsamsa",          label: "D20", name: "Vimsamsa (영적)",       key: "vimsamsa_rasi" },
  { id: "chaturvimshamsa",   label: "D24", name: "Chaturvimshamsa (교육)",key: "chaturvimshamsa_rasi" },
  { id: "saptavimsamsa",     label: "D27", name: "Saptavimsamsa (체력)",  key: "saptavimsamsa_rasi" },
  { id: "trimsamsa",         label: "D30", name: "Trimsamsa (재앙)",      key: "trimsamsa_rasi" },
  { id: "khavedamsa",        label: "D40", name: "Khavedamsa",            key: "khavedamsa_rasi" },
  { id: "akshavedamsa",      label: "D45", name: "Akshavedamsa",          key: "akshavedamsa_rasi" },
  { id: "shashtyamsa",       label: "D60", name: "Shashtyamsa (카르마)",  key: "shashtyamsa_rasi" },
  { id: "navanavamsa",       label: "D81", name: "Navanavamsa",           key: "navanavamsa_rasi" },
  { id: "ashtottaramsa",     label: "D108",name: "Ashtottaramsa",         key: "ashtottaramsa_rasi" },
  { id: "dwadasdwadasamsa",  label: "D144",name: "Dwadasdwadasamsa",      key: "dwadasdwadasamsa_rasi" },
];
// ──────────────────────────────────────────────────────────────────────────────

// 한국 DST(서머타임) 적용 기간 체크
function isKoreaDST(year: number, month: number): boolean {
  // 1948-1951, 1955-1960, 1987-1988년 여름(5-9월) 기간
  if ((year >= 1948 && year <= 1951) || (year >= 1955 && year <= 1960) || (year >= 1987 && year <= 1988)) {
    return month >= 5 && month <= 9;
  }
  return false;
}

interface VedicAnalysisReport {
  primary_karakas: {
    atmakaraka: string;
    amatyakaraka: string;
    darakaraka: string;
  };
  house_summary: Array<{
    house: number;
    rating: string;
    total_score: number;
  }>;
  dasha_focus: string;
  nakshatra_info: string;
  overall_strength_score: number;
  sade_sati: string;
  yogas: Array<{
    name: string;
    yoga_type: string;
    description: string;
    planets_involved: string[];
    quality: string | { Weak: string };
  }>;
}

interface VedicAnalysisResult {
  report: VedicAnalysisReport;
  chart: any; // Raw VedicChart data
}

function App() {
  const [birthData, setBirthData] = useState({
    year: 1990,
    month: 1,
    day: 1,
    hour: 12,
    minute: 0,
    lat: 37.5665,
    lon: 126.978,
  });
  const [selectedCity, setSelectedCity] = useState("서울");
  const [isMale, setIsMale] = useState(true);

  const [report, setReport] = useState<VedicAnalysisResult | null>(null);
  const [sajuReport, setSajuReport] = useState<any | null>(null);
  const [transitReport, setTransitReport] = useState<any | null>(null);
  const [loading, setLoading] = useState(false);
  const [activeTab, setActiveTab] = useState("overview");
  const [copied, setCopied] = useState(false);

  // 궁합 분석 상태
  const [birthData2, setBirthData2] = useState({ year: 1990, month: 6, day: 15, hour: 10, minute: 0, lat: 37.5665, lon: 126.978 });
  const [isMale2, setIsMale2] = useState(false);
  const [selectedCity2, setSelectedCity2] = useState("서울");
  const [compReport, setCompReport] = useState<any | null>(null);
  const [compLoading, setCompLoading] = useState(false);

  const handleCityChange2 = (cityName: string) => {
    const city = KOREAN_CITIES.find((c) => c.name === cityName);
    if (city) {
      setSelectedCity2(cityName);
      setBirthData2((prev) => ({ ...prev, lat: city.lat, lon: city.lon }));
    }
  };

  const runCompatibilityAnalysis = async () => {
    setCompLoading(true);
    try {
      const [saju, vedic] = await Promise.all([
        get_saju_compatibility({
          year1: birthData.year, month1: birthData.month, day1: birthData.day, hour1: birthData.hour, minute1: birthData.minute,
          is_male1: isMale, lon1: birthData.lon, lat1: birthData.lat,
          year2: birthData2.year, month2: birthData2.month, day2: birthData2.day, hour2: birthData2.hour, minute2: birthData2.minute,
          is_male2: isMale2, lon2: birthData2.lon, lat2: birthData2.lat,
          timezone: "Asia/Seoul",
        }),
        get_vedic_compatibility({
          year1: birthData.year, month1: birthData.month, day1: birthData.day, hour1: birthData.hour, minute1: birthData.minute, lat1: birthData.lat, lon1: birthData.lon,
          year2: birthData2.year, month2: birthData2.month, day2: birthData2.day, hour2: birthData2.hour, minute2: birthData2.minute, lat2: birthData2.lat, lon2: birthData2.lon,
        }),
      ]);
      setCompReport({ saju, vedic });
    } catch (e) {
      console.error(e);
    } finally {
      setCompLoading(false);
    }
  };

  const isDST = isKoreaDST(birthData.year, birthData.month);

  const handleCityChange = (cityName: string) => {
    const city = KOREAN_CITIES.find((c) => c.name === cityName);
    if (city) {
      setSelectedCity(cityName);
      setBirthData((prev) => ({ ...prev, lat: city.lat, lon: city.lon }));
    }
  };

  const runAnalysis = async () => {
    const now = new Date();
    setLoading(true);
    try {
      const [vedic, saju, transit] = await Promise.all([
        get_vedic_analysis({ ...birthData }),
        get_saju_analysis({
          ...birthData,
          is_male: isMale,
          timezone: "Asia/Seoul",
        }),
        get_transit_analysis({
          ...birthData,
          is_male: isMale,
          timezone: "Asia/Seoul",
          current_year: now.getFullYear(),
          current_month: now.getMonth() + 1,
        }),
      ]);
      setReport(vedic);
      setSajuReport(saju);
      setTransitReport(transit);
    } catch (e) {
      console.error(e);
    } finally {
      setLoading(false);
    }
  };

  const ShootingStars = () => {
    return (
      <div className="fixed inset-0 pointer-events-none overflow-hidden z-0">
        {[...Array(5)].map((_, i) => (
          <div
            key={i}
            className="shooting-star"
            style={{
              top: `${Math.random() * 50}%`,
              left: `${Math.random() * 50}%`,
              animationDelay: `${Math.random() * 10}s`,
            }}
          />
        ))}
      </div>
    );
  };

  const renderBirthInputForm = () => (
    <div className="glass p-6 rounded-[2rem] mb-8">
      <h5 className="text-lg font-bold text-white mb-5 flex items-center gap-3">
        <Calendar className="w-5 h-5 text-celestial-purple" />
        출생 정보 입력
        {isDST && (
          <span className="ml-auto text-xs px-3 py-1 rounded-full bg-amber-500/20 text-amber-400 border border-amber-500/30 font-semibold animate-pulse">
            ☀️ 서머타임 자동 적용
          </span>
        )}
      </h5>
      <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-6 gap-3 mb-4">
        {/* 년 */}
        <div>
          <label className="block text-xs text-white/40 mb-1.5 font-medium">년</label>
          <input
            type="number"
            value={birthData.year}
            onChange={(e) => setBirthData((prev) => ({ ...prev, year: Number(e.target.value) }))}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none focus:ring-1 focus:ring-celestial-purple/30 transition-all"
            min={1900}
            max={2100}
          />
        </div>
        {/* 월 */}
        <div>
          <label className="block text-xs text-white/40 mb-1.5 font-medium">월</label>
          <select
            value={birthData.month}
            onChange={(e) => setBirthData((prev) => ({ ...prev, month: Number(e.target.value) }))}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none appearance-none cursor-pointer"
          >
            {Array.from({ length: 12 }, (_, i) => (
              <option key={i + 1} value={i + 1} className="bg-gray-900">
                {i + 1}월
              </option>
            ))}
          </select>
        </div>
        {/* 일 */}
        <div>
          <label className="block text-xs text-white/40 mb-1.5 font-medium">일</label>
          <input
            type="number"
            value={birthData.day}
            onChange={(e) => setBirthData((prev) => ({ ...prev, day: Number(e.target.value) }))}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none focus:ring-1 focus:ring-celestial-purple/30 transition-all"
            min={1}
            max={31}
          />
        </div>
        {/* 시 */}
        <div>
          <label className="block text-xs text-white/40 mb-1.5 font-medium">시</label>
          <select
            value={birthData.hour}
            onChange={(e) => setBirthData((prev) => ({ ...prev, hour: Number(e.target.value) }))}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none appearance-none cursor-pointer"
          >
            {Array.from({ length: 24 }, (_, i) => (
              <option key={i} value={i} className="bg-gray-900">
                {String(i).padStart(2, "0")}시
              </option>
            ))}
          </select>
        </div>
        {/* 분 */}
        <div>
          <label className="block text-xs text-white/40 mb-1.5 font-medium">분</label>
          <input
            type="number"
            value={birthData.minute}
            onChange={(e) => setBirthData((prev) => ({ ...prev, minute: Number(e.target.value) }))}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none focus:ring-1 focus:ring-celestial-purple/30 transition-all"
            min={0}
            max={59}
          />
        </div>
        {/* 도시 */}
        <div>
          <label className="block text-xs text-white/40 mb-1.5 font-medium flex items-center gap-1">
            <MapPin className="w-3 h-3" /> 출생지
          </label>
          <select
            value={selectedCity}
            onChange={(e) => handleCityChange(e.target.value)}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none appearance-none cursor-pointer"
          >
            {KOREAN_CITIES.map((city) => (
              <option key={city.name} value={city.name} className="bg-gray-900">
                {city.name}
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* 하단: 성별 + 분석 실행 + 보정 정보 */}
      <div className="flex items-center gap-3 flex-wrap">
        <button
          onClick={() => setIsMale(!isMale)}
          className="glass px-4 py-2.5 rounded-xl flex items-center gap-2 hover:bg-white/10 transition-all text-sm"
        >
          <User className="w-4 h-4 text-celestial-cyan" />
          <span className="text-white font-semibold">{isMale ? "남" : "여"}</span>
        </button>

        <button
          onClick={runAnalysis}
          disabled={loading}
          className="bg-gradient-to-r from-celestial-purple to-brand-600 px-6 py-2.5 rounded-xl font-bold text-white text-sm shadow-lg shadow-indigo-500/20 hover:scale-105 active:scale-95 transition-all disabled:opacity-50"
        >
          {loading ? "계산 중..." : "통합 분석 시작"}
        </button>

        <div className="ml-auto text-xs text-white/30 flex items-center gap-4">
          <span>
            위도 {birthData.lat.toFixed(2)}° / 경도 {birthData.lon.toFixed(2)}°
          </span>
          {sajuReport?.corrected_time && (
            <span className="text-celestial-cyan/60">
              보정시: {sajuReport.corrected_time}
            </span>
          )}
          {sajuReport?.is_dst && (
            <span className="text-amber-400/80">DST 적용됨</span>
          )}
        </div>
      </div>
    </div>
  );

  const renderSajuResults = () => {
    if (!sajuReport || !sajuReport.report) return null;
    const reportData = sajuReport.report;
    const p = reportData.pillars;
    const s = reportData.strength;
    const y = reportData.yongshin;
    const st = reportData.structure;
    const sp = reportData.spirit_markers;
    const ml = reportData.major_luck;
    const gt = reportData.golden_time;

    return (
      <motion.div
        key="saju-results"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -20 }}
        className="space-y-8"
      >
        {/* 사주팔자 차트 */}
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Activity className="w-6 h-6 text-celestial-gold" />
            사주팔자 (四柱八字)
          </h5>
          <div className="grid grid-cols-4 gap-4">
            {[
              { label: "시주", pillar: p?.hour },
              { label: "일주", pillar: p?.day },
              { label: "월주", pillar: p?.month },
              { label: "년주", pillar: p?.year },
            ].map(({ label, pillar }) => (
              <div key={label} className="text-center p-4 bg-white/5 rounded-2xl border border-white/10">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">{label}</p>
                <p className="text-2xl font-bold text-celestial-gold mb-1">
                  {pillar?.stem?.hanja || "—"}
                </p>
                <p className="text-2xl font-bold text-celestial-cyan">
                  {pillar?.branch?.hanja || "—"}
                </p>
                <p className="text-xs text-white/30 mt-2">
                  {pillar?.stem?.hangul || ""} {pillar?.branch?.hangul || ""}
                </p>
              </div>
            ))}
          </div>
        </div>

        {/* 역량 + 용신 + 격국 */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          {/* 역량 분석 */}
          <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
            <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
              <Zap className="w-32 h-32" />
            </div>
            <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
              신강/신약 분석
            </p>
            <h4 className="text-3xl font-bold text-white mb-2">
              {s?.strength_type?.hangul || "—"}
            </h4>
            <div className="flex items-baseline gap-2 mb-4">
              <span className="text-5xl font-black text-gradient leading-none">
                {s?.strength_score != null ? Math.round(s.strength_score) : "—"}
              </span>
              <span className="text-white/20 font-bold">점</span>
            </div>
            <div className="space-y-1 text-xs text-white/50">
              <p>득령: {s?.deuk_ryeong?.acquired ? "✅" : "❌"}</p>
              <p>득지: {s?.deuk_ji?.acquired ? "✅" : "❌"}</p>
              <p>득시: {s?.deuk_si?.acquired ? "✅" : "❌"}</p>
              <p>득세: {s?.deuk_se?.acquired ? "✅" : "❌"}</p>
            </div>
          </div>

          {/* 용신 */}
          <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
            <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
              용신 (用神)
            </p>
            <h4 className="text-3xl font-bold text-white mb-4">
              {y?.primary?.hangul || "—"}
            </h4>
            <p className="text-sm text-white/60 mb-2">
              <span className="text-white/40">보조 용신:</span>{" "}
              {y?.assistant?.hangul || "—"}
            </p>
            <p className="text-xs text-white/40 leading-relaxed">
              용신은 사주의 균형을 맞추는 가장 필요한 오행입니다.
            </p>
          </div>

          {/* 격국 */}
          <div className="glass p-8 rounded-[2rem]">
            <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
              격국 (格局)
            </p>
            <h4 className="text-3xl font-bold text-white mb-4">
              {st?.structure?.hangul || "—"}
            </h4>
            <p className="text-sm text-white/60 leading-relaxed">
              {st?.reason || ""}
            </p>
          </div>
        </div>

        {/* 신살 */}
        {sp?.markers && sp.markers.length > 0 && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <Shield className="w-6 h-6 text-celestial-cyan" />
              신살 (神煞) 분석
            </h5>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-3">
              {sp.markers.map((m: any, i: number) => (
                <div
                  key={i}
                  className="p-4 bg-white/5 rounded-xl border border-white/10 text-center"
                >
                  <p className="text-sm font-bold text-celestial-gold">{m.marker?.hangul || "—"}</p>
                  <p className="text-xs text-white/40 mt-1">{m.position?.hangul || ""}</p>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* 대운 */}
        {ml && ml.cycles && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <TrendingUp className="w-6 h-6 text-celestial-purple" />
              대운 (大運) 흐름
            </h5>
            <p className="text-sm text-white/50 mb-4">
              방향: {ml.direction === "Forward" ? "순행 ▶" : "역행 ◀"} | 시작 나이: {ml.start_age}세
            </p>
            <div className="grid grid-cols-2 sm:grid-cols-5 lg:grid-cols-10 gap-3">
              {ml.cycles.map((c: any, i: number) => (
                <div
                  key={i}
                  className="p-3 bg-white/5 rounded-xl border border-white/10 text-center hover:bg-white/10 transition-all"
                >
                  <p className="text-xs text-white/40 mb-1">{c.start_age}~{c.end_age}세</p>
                  <p className="text-lg font-bold text-white">{c.ganzi?.hangul || "—"}</p>
                  <p className="text-[10px] text-celestial-gold mt-1">{c.stem_god || ""}</p>
                </div>
              ))}
            </div>
          </div>
        )}

        {/* 골든타임 */}
        {gt && (
          <div className="glass p-8 rounded-[2rem] border-celestial-gold/20 bg-celestial-gold/5">
            <h5 className="text-xl font-bold text-white mb-4 flex items-center gap-3">
              <Star className="w-6 h-6 text-celestial-gold" />
              골든타임 🏆
            </h5>
            <div className="flex items-baseline gap-4 mb-4">
              <span className="text-5xl font-black text-celestial-gold">
                {gt.start_age}~{gt.end_age}세
              </span>
              <span className="text-white/40">평균 점수: {gt.average_score?.toFixed(1)}</span>
            </div>
            <p className="text-sm text-white/60">{gt.description}</p>
          </div>
        )}

        {/* 인생 시뮬레이션 타임라인 (LifeFrame × 100) */}
        {reportData.simulation_frames && reportData.simulation_frames.length > 0 && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <TrendingUp className="w-6 h-6 text-celestial-cyan" />
              인생 시뮬레이션 (0~100세 System Score)
            </h5>
            <div className="flex items-end gap-0.5 h-24 w-full overflow-x-auto">
              {(reportData.simulation_frames as any[]).map((f: any, i: number) => {
                const score: number = f.score ?? 0;
                const heightPct = Math.max(4, Math.round(score));
                const color =
                  score >= 70
                    ? "bg-green-400"
                    : score >= 40
                    ? "bg-celestial-gold"
                    : "bg-red-400";
                return (
                  <div
                    key={i}
                    title={`${f.age}세: ${score.toFixed(1)}`}
                    className={`flex-1 min-w-[4px] rounded-sm ${color} opacity-80 hover:opacity-100 transition-opacity cursor-help`}
                    style={{ height: `${heightPct}%` }}
                  />
                );
              })}
            </div>
            <div className="flex justify-between text-xs text-white/30 mt-2">
              <span>0세</span>
              <span>25세</span>
              <span>50세</span>
              <span>75세</span>
              <span>100세</span>
            </div>
            <div className="flex gap-4 mt-3 text-xs text-white/40">
              <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-green-400 inline-block" />70+ 좋음</span>
              <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-celestial-gold inline-block" />40~70 보통</span>
              <span className="flex items-center gap-1"><span className="w-2 h-2 rounded-full bg-red-400 inline-block" />~40 주의</span>
            </div>
          </div>
        )}
      </motion.div>
    );
  };

  const renderVedicCharts = () => {
    if (!report || !report.chart || !report.chart.planets) return null;
    const planets: any[] = report.chart.planets;
    const ascendant: any = report.chart.ascendant;
    const panchanga = report.chart.panchanga;

    // ── 복사 텍스트 생성 헬퍼 ──────────────────────────────────────────
    const fmtPosition = (sidereal_deg: number) => {
      const deg = ((sidereal_deg % 360) + 360) % 360;
      const sign = Math.floor(deg / 30) + 1;
      const d = deg % 30;
      const dd = Math.floor(d);
      const mm = Math.floor((d - dd) * 60);
      return `${dd}°${String(mm).padStart(2, "0")}' ${SIGN_NAMES[sign]}`;
    };

    const buildD1ReportText = () => {
      const header = ["Planet", "Position", "Nakshatra (Pada)", "Nak Lord", "Pada Lord", "Deity", "Purpose"]
        .join("\t");
      const sep = "─".repeat(120);

      const allPoints = [
        ...planets.map((p: any) => ({ name: p.planet, deg: p.sidereal_deg, retro: p.is_retrograde, combust: p.is_combust })),
        { name: "ASC", deg: ascendant?.sidereal_deg, retro: false, combust: false },
      ].filter(x => x.deg != null);

      const rows = allPoints.map(p => {
        const ni = getNakshatraInfo(p.deg);
        const flags = [p.retro ? "℞" : "", p.combust ? "☀" : ""].filter(Boolean).join(" ");
        return [
          `${p.name}${flags ? " " + flags : ""}`,
          fmtPosition(p.deg),
          `${ni.name} (Pada ${ni.pada})`,
          ni.lord,
          ni.padaLord,
          ni.deity,
          ni.purpose,
        ].join("\t");
      });

      return [sep, "D1 Nakshatra Report", sep, header, ...rows, sep].join("\n");
    };

    const buildVargaTableText = () => {
      const header = ["Planet", ...VARGA_DEFS.map(v => v.label)].join("\t");
      const allPoints = [
        ...planets.map((p: any) => ({ name: p.planet, data: p })),
        { name: "ASC", data: ascendant },
      ].filter(x => x.data != null);

      const rows = allPoints.map(({ name, data }) => {
        const signs = VARGA_DEFS.map(v => SIGN_NAMES[data[v.key]] ?? "-");
        return [name, ...signs].join("\t");
      });

      return ["D1–D144 Sign Positions", header, ...rows].join("\n");
    };

    const copyReport = async () => {
      const text = [buildD1ReportText(), "", buildVargaTableText()].join("\n");
      await navigator.clipboard.writeText(text);
      setCopied(true);
      setTimeout(() => setCopied(false), 2500);
    };

    // ── D1 낙샤트라 상세 테이블 데이터 ─────────────────────────────────
    const d1Rows = [
      ...planets.map((p: any) => ({
        name: p.planet as string,
        sidereal_deg: p.sidereal_deg as number,
        retro: p.is_retrograde as boolean,
        combust: p.is_combust as boolean,
      })),
      ...(ascendant ? [{ name: "ASC", sidereal_deg: ascendant.sidereal_deg, retro: false, combust: false }] : []),
    ].filter(x => x.sidereal_deg != null);

    return (
      <motion.div
        key="vedic-charts"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -20 }}
        className="space-y-8"
      >
        {/* Panchanga 섹션 */}
        {panchanga && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <Calendar className="w-6 h-6 text-celestial-gold" />
              판창가 (Panchanga) — 출생 시각의 천문 요소
            </h5>
            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-5 gap-4">
              {[
                { label: "바라 (요일)", value: panchanga.vara },
                { label: "티티 (음력일)", value: `${panchanga.tithi} — ${panchanga.tithi_name}` },
                { label: "낙샤트라 번호", value: `No. ${panchanga.nakshatra}` },
                { label: "요가 (Nitya)", value: `No. ${panchanga.yoga}` },
                { label: "카라나", value: panchanga.karana_name },
              ].map(({ label, value }) => (
                <div key={label} className="p-4 bg-white/5 rounded-xl border border-white/10 text-center">
                  <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">{label}</p>
                  <p className="text-sm font-semibold text-white leading-snug">{value ?? "—"}</p>
                </div>
              ))}
            </div>
            <div className="grid grid-cols-2 gap-4 mt-4">
              <div className="p-4 bg-white/5 rounded-xl border border-white/10">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">일주 천주 (Day Lord / Hour Lord)</p>
                <p className="text-sm text-white">{panchanga.day_lord} / {panchanga.hour_lord}</p>
              </div>
              <div className="p-4 bg-white/5 rounded-xl border border-white/10">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">출생 시간대</p>
                <p className="text-sm text-white">{panchanga.is_day_birth ? "☀️ 주간 출생" : "🌙 야간 출생"}</p>
              </div>
            </div>
          </div>
        )}

        {/* ── D1 낙샤트라 상세 테이블 ─────────────────────────────────── */}
        <div className="glass p-8 rounded-[2rem]">
          <div className="flex items-center justify-between mb-6">
            <h5 className="text-xl font-bold text-white flex items-center gap-3">
              <Star className="w-6 h-6 text-celestial-gold" />
              D1 낙샤트라 리포트 (전체 행성)
            </h5>
            <button
              onClick={copyReport}
              className="flex items-center gap-2 px-4 py-2 rounded-xl bg-white/10 hover:bg-white/20 border border-white/20 text-sm text-white font-semibold transition-all"
            >
              {copied ? <Check className="w-4 h-4 text-green-400" /> : <Copy className="w-4 h-4" />}
              {copied ? "복사됨!" : "리포트 복사 (D1+분할)"}
            </button>
          </div>

          <div className="overflow-x-auto">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-white/10">
                  {["행성", "위치 (사이드리얼)", "낙샤트라 (파다)", "파다 범위", "낙샤트라 로드", "파다 로드", "신 (Deity)", "목적 (Purpose)"].map(h => (
                    <th key={h} className="text-left text-xs text-white/40 font-bold uppercase tracking-wider pb-3 pr-4 whitespace-nowrap">{h}</th>
                  ))}
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                {d1Rows.map((row, i) => {
                  const ni = getNakshatraInfo(row.sidereal_deg);
                  const purposeColor =
                    ni.purpose === "Dharma" ? "text-celestial-gold"
                    : ni.purpose === "Artha" ? "text-green-400"
                    : ni.purpose === "Kama" ? "text-pink-400"
                    : "text-blue-400";
                  return (
                    <tr key={i} className="hover:bg-white/3 transition-colors">
                      <td className="py-3 pr-4 font-bold text-white whitespace-nowrap">
                        {row.name}
                        {row.retro && <span className="ml-1.5 text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/40">℞</span>}
                        {row.combust && <span className="ml-1 text-[10px] px-1.5 py-0.5 rounded bg-orange-500/20 text-orange-400 border border-orange-500/40">☀</span>}
                      </td>
                      <td className="py-3 pr-4 text-white/70 font-mono text-xs whitespace-nowrap">{fmtPosition(row.sidereal_deg)}</td>
                      <td className="py-3 pr-4 text-celestial-cyan font-semibold whitespace-nowrap">
                        {ni.name}
                        <span className="ml-1.5 text-[10px] text-white/40">(Pada {ni.pada})</span>
                      </td>
                      <td className="py-3 pr-4 text-white/40 text-xs whitespace-nowrap">{ni.range}</td>
                      <td className="py-3 pr-4 text-white/70 whitespace-nowrap">{ni.lord}</td>
                      <td className="py-3 pr-4 text-white/70 whitespace-nowrap">{ni.padaLord}</td>
                      <td className="py-3 pr-4 text-white/60 whitespace-nowrap">{ni.deity}</td>
                      <td className="py-3 pr-4 whitespace-nowrap">
                        <span className={`text-xs font-bold ${purposeColor}`}>{ni.purpose}</span>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        </div>

        {/* ── D1-D144 분할 차트 사인 위치 테이블 ──────────────────────── */}
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-2 flex items-center gap-3">
            <Star className="w-6 h-6 text-celestial-purple" />
            D1 – D144 사인 포지션 (전체 분할 차트)
          </h5>
          <p className="text-xs text-white/40 mb-6">각 셀 = 해당 분할 차트에서의 사인 번호 (1=Aries … 12=Pisces)</p>

          <div className="flex gap-4 text-xs text-white/40 mb-4">
            <span className="flex items-center gap-1"><span className="px-1.5 rounded bg-amber-500/20 text-amber-400 border border-amber-500/40 font-bold">℞</span> 역행</span>
            <span className="flex items-center gap-1"><span className="px-1.5 rounded bg-orange-500/20 text-orange-400 border border-orange-500/40 font-bold">☀</span> 소각</span>
          </div>

          <div className="overflow-x-auto">
            <table className="text-xs">
              <thead>
                <tr className="border-b border-white/10">
                  <th className="text-left text-white/40 font-bold uppercase tracking-wider pb-3 pr-3 whitespace-nowrap">행성</th>
                  {VARGA_DEFS.map(v => (
                    <th key={v.id} className="text-center text-white/40 font-bold pb-3 px-2 whitespace-nowrap">
                      <span className="text-white/70">{v.label}</span>
                    </th>
                  ))}
                </tr>
                <tr className="border-b border-white/5">
                  <th className="pb-2 pr-3"></th>
                  {VARGA_DEFS.map(v => (
                    <th key={v.id} className="text-center text-white/25 font-normal pb-2 px-2 whitespace-nowrap text-[10px]">
                      {v.name.substring(0, 8)}
                    </th>
                  ))}
                </tr>
              </thead>
              <tbody className="divide-y divide-white/5">
                {[
                  ...planets.map((p: any) => ({ name: p.planet, data: p, retro: p.is_retrograde, combust: p.is_combust })),
                  ...(ascendant ? [{ name: "ASC", data: ascendant, retro: false, combust: false }] : []),
                ].map((row, i) => (
                  <tr key={i} className="hover:bg-white/3 transition-colors">
                    <td className="py-2 pr-3 font-bold text-white whitespace-nowrap">
                      {row.name}
                      {row.retro && <span className="ml-1 text-[9px] px-1 rounded bg-amber-500/20 text-amber-400">℞</span>}
                      {row.combust && <span className="ml-0.5 text-[9px] px-1 rounded bg-orange-500/20 text-orange-400">☀</span>}
                    </td>
                    {VARGA_DEFS.map(v => {
                      const signNum: number = row.data?.[v.key];
                      const signName = SIGN_NAMES[signNum] ?? "—";
                      return (
                        <td key={v.id} className="py-2 px-2 text-center whitespace-nowrap" title={`${row.name} in ${v.name}: ${signName}`}>
                          <span className="inline-block min-w-[24px] text-white/70 font-mono">
                            {signNum ?? "—"}
                          </span>
                          <br />
                          <span className="text-[9px] text-white/30">{signName.substring(0, 3)}</span>
                        </td>
                      );
                    })}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </motion.div>
    );
  };

  const renderStrengthTab = () => {
    if (!sajuReport) return null;
    const entropy = sajuReport.entropy;
    const topology = sajuReport.qi_topology;
    const lints: any[] = sajuReport.lints ?? [];
    const loadDiag: any[] = sajuReport.load_diagnostics ?? [];
    const crashCount: number = sajuReport.crash_count ?? 0;

    const ELEMENT_COLORS: Record<string, string> = {
      Wood: "text-green-400  border-green-500/30 bg-green-500/10",
      Fire: "text-red-400    border-red-500/30   bg-red-500/10",
      Earth: "text-yellow-400 border-yellow-500/30 bg-yellow-500/10",
      Metal: "text-gray-300  border-gray-400/30  bg-gray-400/10",
      Water: "text-blue-400  border-blue-500/30  bg-blue-500/10",
    };

    return (
      <motion.div
        key="strength-tab"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -20 }}
        className="space-y-8"
      >
        {/* 오행 네트워크 (Qi Topology) */}
        {topology?.nodes && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <Activity className="w-6 h-6 text-celestial-cyan" />
              기(氣) 위상 분석 (Qi Topology)
            </h5>
            <div className="grid grid-cols-5 gap-4">
              {topology.nodes.map((node: any, i: number) => {
                const colorClass = ELEMENT_COLORS[node.element?.english] ?? "text-white/60";
                const pct = Math.round(node.output * 100);
                return (
                  <div key={i} className={`p-4 rounded-2xl border text-center ${colorClass}`}>
                    <p className="text-xs font-bold uppercase tracking-wider mb-2">{node.element?.hangul ?? node.element}</p>
                    <p className="text-3xl font-black mb-1">{pct}%</p>
                    <div className="w-full bg-white/10 h-1.5 rounded-full overflow-hidden">
                      <div className="h-full rounded-full bg-current transition-all" style={{ width: `${pct}%` }} />
                    </div>
                    {(topology?.bottleneck?.hangul && node.element?.hangul === topology.bottleneck.hangul) && (
                      <span className="mt-2 inline-block text-[10px] px-2 py-0.5 rounded-full bg-red-500/20 text-red-400 border border-red-500/40">병목</span>
                    )}
                  </div>
                );
              })}
            </div>
          </div>
        )}

        {/* Destiny Entropy */}
        {entropy && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <Zap className="w-6 h-6 text-celestial-gold" />
              운명 엔트로피 (Destiny Entropy / DIE)
            </h5>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
              <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">복잡도 등급</p>
                <p className="text-4xl font-black text-celestial-gold">{entropy.level ?? "—"}</p>
              </div>
              <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">Shannon Entropy</p>
                <p className="text-4xl font-black text-white">{entropy.score?.toFixed(3) ?? "—"}</p>
              </div>
              <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">취약점 (Fuzzer)</p>
                <p className={`text-4xl font-black ${crashCount > 5 ? "text-red-400" : "text-green-400"}`}>{crashCount}개</p>
              </div>
            </div>
          </div>
        )}

        {/* Destiny Linter */}
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Shield className="w-6 h-6 text-celestial-purple" />
            Destiny Linter (사주 진단)
          </h5>
          {lints.length === 0 ? (
            <p className="text-green-400 font-semibold">✅ No issues found. Perfect structure!</p>
          ) : (
            <div className="space-y-3">
              {lints.map((lint: any, i: number) => (
                <div key={i} className={`p-4 rounded-xl border flex gap-3 items-start ${
                  lint.severity === "Error"
                    ? "bg-red-500/10 border-red-500/30"
                    : lint.severity === "Warning"
                    ? "bg-yellow-500/10 border-yellow-500/30"
                    : "bg-blue-500/10 border-blue-500/30"
                }`}>
                  <span className={`text-xs font-black px-2 py-1 rounded shrink-0 ${
                    lint.severity === "Error" ? "bg-red-500/30 text-red-400"
                    : lint.severity === "Warning" ? "bg-yellow-500/30 text-yellow-400"
                    : "bg-blue-500/30 text-blue-400"
                  }`}>{lint.severity?.toUpperCase()}</span>
                  <div>
                    <p className="text-sm font-bold text-white">[{lint.code}] {lint.message}</p>
                    <p className="text-xs text-white/50 mt-1">└ {lint.advice}</p>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>

        {/* Load Balancer 상위 진단 */}
        {loadDiag.length > 0 && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <TrendingUp className="w-6 h-6 text-brand-400" />
              카르마 부하 진단 (KarmaLoadBalancer)
            </h5>
            <div className="space-y-3">
              {loadDiag.slice(0, 8).map((d: any, i: number) => (
                <div key={i} className={`p-4 rounded-xl border flex gap-4 items-center ${
                  d.status === "SystemDown" ? "bg-red-500/10 border-red-500/30"
                  : d.status === "Overloaded" ? "bg-orange-500/10 border-orange-500/30"
                  : "bg-white/5 border-white/10"
                }`}>
                  <span className="text-2xl">
                    {d.status === "SystemDown" ? "🚫" : d.status === "Overloaded" ? "🔥" : "ℹ️"}
                  </span>
                  <div className="flex-1">
                    <p className="text-sm font-bold text-white">[{d.age}세] {d.reason}</p>
                    <p className="text-xs text-white/50 mt-0.5">▶ {d.strategy}</p>
                  </div>
                  <span className="text-xs text-white/30 shrink-0">{d.status}</span>
                </div>
              ))}
            </div>
          </div>
        )}
      </motion.div>
    );
  };

  const renderTransitTab = () => {
    if (!transitReport) return null;
    const yr = transitReport.yearly_luck;
    const mo = transitReport.monthly_luck;
    const frame = transitReport.current_frame;
    const nearby: any[] = transitReport.nearby_diagnostics ?? [];
    const age: number = transitReport.current_age ?? 0;

    const scoreColor = (s: number) =>
      s >= 70 ? "text-green-400" : s >= 40 ? "text-yellow-400" : "text-red-400";

    return (
      <motion.div
        key="transit-tab"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -20 }}
        className="space-y-8"
      >
        {/* 세운 / 월운 카드 */}
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          <div className="glass p-8 rounded-[2rem]">
            <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-3">세운 (年運) — {yr?.year}년</p>
            <h4 className="text-4xl font-black text-celestial-gold mb-3">{yr?.ganzi?.hangul ?? "—"}</h4>
            <div className="space-y-1 text-sm text-white/60 mb-4">
              <p>천간 십성: <span className="text-white font-semibold">{yr?.stem_god?.hangul ?? yr?.stem_god ?? "—"}</span></p>
              <p>지지 십성: <span className="text-white font-semibold">{yr?.branch_god?.hangul ?? yr?.branch_god ?? "—"}</span></p>
              {yr?.twelve_stage && (
                <p>12운성: <span className="text-celestial-cyan font-semibold">{yr.twelve_stage}</span></p>
              )}
              {yr?.special_events?.length > 0 && (
                <p className="text-red-400 font-semibold">⚠️ {yr.special_events.join(" / ")}</p>
              )}
            </div>
            {yr?.influence?.relations_with_natal?.length > 0 && (
              <div className="flex flex-wrap gap-1.5 mt-3">
                {yr.influence.relations_with_natal.map((rel: string, i: number) => (
                  <span key={i} className="text-xs px-2.5 py-1 rounded-full bg-celestial-gold/15 text-celestial-gold border border-celestial-gold/30 font-medium">{rel}</span>
                ))}
              </div>
            )}
          </div>
          <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
            <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-3">월운 (月運) — {mo?.month}월</p>
            <h4 className="text-4xl font-black text-white mb-3">{mo?.ganzi?.hangul ?? "—"}</h4>
            <div className="space-y-1 text-sm text-white/60 mb-4">
              <p>천간 십성: <span className="text-white font-semibold">{mo?.stem_god?.hangul ?? mo?.stem_god ?? "—"}</span></p>
              <p>지지 십성: <span className="text-white font-semibold">{mo?.branch_god?.hangul ?? mo?.branch_god ?? "—"}</span></p>
              {mo?.twelve_stage && (
                <p>12운성: <span className="text-celestial-cyan font-semibold">{mo.twelve_stage}</span></p>
              )}
            </div>
            {mo?.influence?.relations_with_natal?.length > 0 && (
              <div className="flex flex-wrap gap-1.5 mt-3">
                {mo.influence.relations_with_natal.map((rel: string, i: number) => (
                  <span key={i} className="text-xs px-2.5 py-1 rounded-full bg-celestial-purple/15 text-celestial-purple border border-celestial-purple/30 font-medium">{rel}</span>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* 현재 나이 LifeFrame */}
        {frame && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <Activity className="w-6 h-6 text-celestial-cyan" />
              현재 시스템 상태 ({age}세 LifeFrame)
            </h5>
            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-6">
              <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">System Score</p>
                <p className={`text-5xl font-black ${scoreColor(frame.score ?? 0)}`}>
                  {frame.score?.toFixed(1) ?? "—"}
                </p>
              </div>
              <div className="p-6 bg-white/5 rounded-2xl border border-white/10 text-center">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">현재 간지</p>
                <p className="text-4xl font-black text-white">{frame.ganzi?.hangul ?? "—"}</p>
              </div>
              <div className="p-6 bg-white/5 rounded-2xl border border-white/10">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-2">Tags</p>
                <div className="flex flex-wrap gap-1">
                  {(frame.tags ?? []).map((tag: string, i: number) => (
                    <span key={i} className="text-xs px-2 py-1 rounded-full bg-celestial-purple/20 text-celestial-purple border border-celestial-purple/30">{tag}</span>
                  ))}
                </div>
              </div>
            </div>
            {/* ESIL Trace */}
            {frame.esil_trace && (
              <div className="bg-black/40 rounded-xl p-4 font-mono text-xs border border-white/10 overflow-x-auto">
                <p className="text-white/40 mb-2">// ESIL TRACE</p>
                {frame.esil_trace.split("; ").map((line: string, i: number) => (
                  <p key={i} className={line.includes("irq") || line.includes("panic") ? "text-red-400" : "text-green-300/70"}>
                    {line}
                  </p>
                ))}
              </div>
            )}

            {/* QiRegisters 미니 바 차트 */}
            {frame.register_state && (
              <div className="mt-4">
                <p className="text-xs text-white/40 font-bold uppercase tracking-wider mb-3">오행 레지스터 (QiRegisters)</p>
                <div className="grid grid-cols-5 gap-3">
                  {[
                    { label: "木", key: "r0_wood", color: "bg-green-400" },
                    { label: "火", key: "r1_fire", color: "bg-red-400" },
                    { label: "土", key: "r2_earth", color: "bg-yellow-400" },
                    { label: "金", key: "r3_metal", color: "bg-gray-300" },
                    { label: "水", key: "r4_water", color: "bg-blue-400" },
                  ].map(({ label, key, color }) => {
                    const val: number = frame.register_state[key] ?? 0;
                    const pct = Math.min(100, Math.round(Math.abs(val) * 4));
                    return (
                      <div key={key} className="text-center">
                        <div className="h-16 bg-white/5 rounded-lg flex items-end overflow-hidden mb-1">
                          <div className={`w-full ${color} rounded-lg opacity-80 transition-all`} style={{ height: `${pct}%` }} />
                        </div>
                        <p className="text-xs text-white/40">{label}</p>
                        <p className="text-[10px] text-white/30">{val.toFixed(1)}</p>
                      </div>
                    );
                  })}
                </div>
              </div>
            )}
          </div>
        )}

        {/* 주변 부하 진단 */}
        {nearby.length > 0 && (
          <div className="glass p-8 rounded-[2rem]">
            <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
              <Shield className="w-6 h-6 text-brand-400" />
              전후 운세 부하 ({age - 3}~{age + 5}세)
            </h5>
            <div className="space-y-3">
              {nearby.map((d: any, i: number) => (
                <div key={i} className={`p-4 rounded-xl border flex gap-4 items-center ${
                  d.status === "SystemDown" ? "bg-red-500/10 border-red-500/30"
                  : d.status === "Overloaded" ? "bg-orange-500/10 border-orange-500/30"
                  : "bg-white/5 border-white/10"
                } ${d.age === age ? "ring-2 ring-celestial-purple" : ""}`}>
                  <span className="text-2xl">{d.status === "SystemDown" ? "🚫" : d.status === "Overloaded" ? "🔥" : "ℹ️"}</span>
                  <div className="flex-1">
                    <p className="text-sm font-bold text-white">
                      {d.age === age && <span className="text-celestial-purple mr-1">[현재]</span>}
                      [{d.age}세] {d.reason}
                    </p>
                    <p className="text-xs text-white/50 mt-0.5">▶ {d.strategy}</p>
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </motion.div>
    );
  };

  const renderCompatibilityTab = () => {
    const ASHTA_LABELS: Record<string, string> = {
      varna: "바르나 (계층)",
      vashya: "바쉬야 (지배)",
      tara: "타라 (별자리)",
      yoni: "요니 (본능)",
      maitri: "마이트리 (우정)",
      gana: "가나 (기질)",
      bhakoot: "바쿠트 (운명)",
      nadi: "나디 (신체)",
    };
    const ASHTA_MAX: Record<string, number> = {
      varna: 1, vashya: 2, tara: 3, yoni: 4, maitri: 5, gana: 6, bhakoot: 7, nadi: 8,
    };

    return (
      <motion.div key="compatibility" initial={{ opacity: 0, y: 20 }} animate={{ opacity: 1, y: 0 }} exit={{ opacity: 0, y: -20 }} className="space-y-8">
        <h2 className="text-2xl font-bold text-white">궁합 분석</h2>

        {/* 두 번째 사람 입력 폼 */}
        <div className="glass p-6 rounded-2xl">
          <h3 className="text-lg font-bold text-white mb-4">상대방 출생 정보</h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
            {(["year","month","day","hour","minute"] as const).map((field) => (
              <div key={field}>
                <label className="text-xs text-brand-400 mb-1 block">
                  {field === "year" ? "연도" : field === "month" ? "월" : field === "day" ? "일" : field === "hour" ? "시" : "분"}
                </label>
                <input
                  type="number"
                  value={birthData2[field]}
                  onChange={(e) => setBirthData2((prev) => ({ ...prev, [field]: parseInt(e.target.value) || 0 }))}
                  className="w-full bg-white/5 border border-white/10 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-white/30"
                />
              </div>
            ))}
          </div>
          <div className="flex gap-4 items-center flex-wrap">
            <div>
              <label className="text-xs text-brand-400 mb-1 block">도시</label>
              <select
                value={selectedCity2}
                onChange={(e) => handleCityChange2(e.target.value)}
                className="bg-white/5 border border-white/10 rounded-lg px-3 py-2 text-white text-sm focus:outline-none focus:border-white/30"
              >
                {KOREAN_CITIES.map((c) => (<option key={c.name} value={c.name}>{c.name}</option>))}
              </select>
            </div>
            <div className="flex gap-3 items-center">
              <label className="text-xs text-brand-400">성별</label>
              {["남성", "여성"].map((g) => (
                <button
                  key={g}
                  onClick={() => setIsMale2(g === "남성")}
                  className={cn("px-4 py-2 rounded-lg text-sm font-medium transition-all", isMale2 === (g === "남성") ? "bg-celestial-purple text-white" : "bg-white/5 text-white/50 hover:bg-white/10")}
                >
                  {g}
                </button>
              ))}
            </div>
            <button
              onClick={runCompatibilityAnalysis}
              disabled={compLoading}
              className="ml-auto px-6 py-2 rounded-xl bg-gradient-to-r from-pink-600 to-rose-500 text-white font-semibold text-sm shadow-lg shadow-pink-500/20 hover:shadow-pink-500/40 transition-all disabled:opacity-50"
            >
              {compLoading ? "분석 중..." : "궁합 분석 시작"}
            </button>
          </div>
        </div>

        {!compReport && (
          <div className="h-40 flex items-center justify-center text-white/40">
            <Heart className="w-6 h-6 mr-2" /> 출생 정보를 입력하고 궁합 분석을 시작하세요
          </div>
        )}

        {compReport && (
          <>
            {/* 사주 궁합 점수 */}
            {compReport.saju && (
              <div className="glass p-6 rounded-2xl">
                <h3 className="text-lg font-bold text-white mb-4">사주 궁합 (CompatibilityAudit)</h3>
                <div className="flex items-center gap-6 mb-6">
                  <div className="text-6xl font-bold" style={{ color: compReport.saju.sync_score > 70 ? "#4ade80" : compReport.saju.sync_score > 40 ? "#facc15" : "#f87171" }}>
                    {compReport.saju.sync_score?.toFixed(0) ?? "--"}
                  </div>
                  <div className="text-white/60 text-sm">/ 100점<br />궁합 동기화 지수</div>
                </div>
                {compReport.saju.synergies?.length > 0 && (
                  <div className="mb-4">
                    <p className="text-xs font-bold text-green-400 uppercase mb-2">시너지 {compReport.saju.synergies.length}개</p>
                    <div className="space-y-1">{compReport.saju.synergies.slice(0, 5).map((s: string, i: number) => (<div key={i} className="text-xs text-green-300 bg-green-500/10 rounded-lg px-3 py-1.5">{s}</div>))}</div>
                  </div>
                )}
                {compReport.saju.conflicts?.length > 0 && (
                  <div className="mb-4">
                    <p className="text-xs font-bold text-red-400 uppercase mb-2">충돌 {compReport.saju.conflicts.length}개</p>
                    <div className="space-y-1">{compReport.saju.conflicts.slice(0, 5).map((s: string, i: number) => (<div key={i} className="text-xs text-red-300 bg-red-500/10 rounded-lg px-3 py-1.5">{s}</div>))}</div>
                  </div>
                )}
                {compReport.saju.deadlocks?.length > 0 && (
                  <div>
                    <p className="text-xs font-bold text-amber-400 uppercase mb-2">교착 {compReport.saju.deadlocks.length}개</p>
                    <div className="space-y-1">{compReport.saju.deadlocks.slice(0, 3).map((s: string, i: number) => (<div key={i} className="text-xs text-amber-300 bg-amber-500/10 rounded-lg px-3 py-1.5">{s}</div>))}</div>
                  </div>
                )}
              </div>
            )}

            {/* 베딕 궁합 Ashta Kuta */}
            {compReport.vedic && (
              <div className="glass p-6 rounded-2xl">
                <div className="flex items-center justify-between mb-4">
                  <h3 className="text-lg font-bold text-white">베딕 궁합 (Ashta Kuta)</h3>
                  <div className="text-2xl font-bold text-celestial-cyan">
                    {compReport.vedic.total_score?.toFixed(1) ?? "--"} <span className="text-sm text-white/40">/ 36점</span>
                  </div>
                </div>
                {compReport.vedic.message && (
                  <p className="text-sm text-white/60 mb-4 italic">{compReport.vedic.message}</p>
                )}
                <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                  {Object.entries(ASHTA_LABELS).map(([key, label]) => {
                    const val = compReport.vedic[key];
                    const max = ASHTA_MAX[key];
                    const pct = val != null ? (val / max) * 100 : 0;
                    return (
                      <div key={key} className="bg-white/5 rounded-xl p-3">
                        <p className="text-xs text-brand-400 mb-1">{label}</p>
                        <div className="flex items-end gap-1 mb-2">
                          <span className="text-xl font-bold text-white">{val?.toFixed ? val.toFixed(1) : val ?? "--"}</span>
                          <span className="text-xs text-white/40 mb-0.5">/ {max}</span>
                        </div>
                        <div className="w-full bg-white/10 rounded-full h-1.5">
                          <div className="h-1.5 rounded-full bg-gradient-to-r from-celestial-purple to-celestial-cyan transition-all" style={{ width: `${pct}%` }} />
                        </div>
                      </div>
                    );
                  })}
                </div>
              </div>
            )}
          </>
        )}
      </motion.div>
    );
  };

  return (
    <div className="h-screen w-full relative flex overflow-hidden">
      <ShootingStars />

      {/* Sidebar */}
      <nav className="w-64 glass border-r border-white/10 flex flex-col p-6 z-10">
        <div className="flex items-center gap-3 mb-10">
          <div className="w-10 h-10 bg-gradient-to-br from-celestial-purple to-celestial-cyan rounded-xl flex items-center justify-center shadow-lg shadow-indigo-500/20">
            <Sparkles className="w-6 h-6 text-white" />
          </div>
          <h1 className="text-2xl font-bold tracking-tight text-white">Eon.</h1>
        </div>

        <div className="space-y-2 flex-1">
          {[
            { id: "overview", label: "대시보드", icon: LayoutDashboard },
            { id: "saju", label: "사주 분석", icon: Activity },
            { id: "vedic_charts", label: "베딕 차트 (D1-144)", icon: Star },
            { id: "strength", label: "역량 및 기운", icon: Zap },
            { id: "transit", label: "현재 운세", icon: Sun },
            { id: "compatibility", label: "궁합 분석", icon: Heart },
          ].map((tab) => (
            <button
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={cn(
                "w-full flex items-center gap-3 px-4 py-3 rounded-xl transition-all duration-200",
                activeTab === tab.id
                  ? "bg-white/10 text-white"
                  : "text-white/50 hover:text-white hover:bg-white/5"
              )}
            >
              <tab.icon className="w-5 h-5" />
              <span className="font-medium">{tab.label}</span>
            </button>
          ))}
        </div>

        <div className="mt-auto pt-6 border-t border-white/5">
          <p className="text-xs text-brand-400 font-medium">BPHS COMPLIANT v0.1.0</p>
        </div>
      </nav>

      {/* Main Content */}
      <main className="flex-1 p-10 overflow-y-auto z-10">
        <header className="mb-8">
          <div className="flex justify-between items-end mb-6">
            <div>
              <h2 className="text-4xl font-bold text-white mb-2">천문(Celestial) 인사이트</h2>
              <p className="text-brand-400">사주명리 & 베딕 점성학 통합 엔진</p>
            </div>
          </div>

          {/* 출생 정보 입력 폼 */}
          {renderBirthInputForm()}
        </header>

        <AnimatePresence mode="wait">
          {!report && !sajuReport ? (
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              className="h-[40vh] flex flex-col items-center justify-center text-center"
            >
              <div className="w-24 h-24 rounded-full bg-white/5 flex items-center justify-center mb-6">
                <Compass className="w-12 h-12 text-white/20 animate-pulse" />
              </div>
              <h3 className="text-2xl font-semibold text-white mb-2">활성화된 차트 없음</h3>
              <p className="text-brand-400 max-w-sm">
                출생 정보를 입력하고 통합 분석 시작 버튼을 눌러주세요.
              </p>
            </motion.div>
          ) : activeTab === "saju" ? (
            renderSajuResults()
          ) : activeTab === "vedic_charts" ? (
            renderVedicCharts()
          ) : activeTab === "strength" ? (
            renderStrengthTab()
          ) : activeTab === "transit" ? (
            renderTransitTab()
          ) : activeTab === "compatibility" ? (
            renderCompatibilityTab()
          ) : (
            report && report.report && (
              <motion.div
                key="results"
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -20 }}
                className="space-y-8"
              >
                {/* Hero Statistics – Karaka 트리오 + 차트 강도 */}
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                  <div className="glass p-8 rounded-[2rem] relative overflow-hidden group">
                    <div className="absolute top-0 right-0 p-8 transform translate-x-4 -translate-y-4 opacity-5 group-hover:translate-x-0 group-hover:translate-y-0 transition-all duration-500">
                      <Heart className="w-32 h-32" />
                    </div>
                    <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
                      영혼의 지표 (Atmakaraka)
                    </p>
                    <h4 className="text-3xl font-bold text-white mb-4">
                      {report.report.primary_karakas.atmakaraka}
                    </h4>
                    <p className="text-sm text-white/60 leading-relaxed">
                      이번 생에서 영혼이 추구하는 가장 강력한 욕망과 핵심 과제를 나타냅니다.
                    </p>
                  </div>

                  <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
                    <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
                      직업 지표 (Amatyakaraka)
                    </p>
                    <h4 className="text-3xl font-bold text-white mb-4">
                      {report.report.primary_karakas.amatyakaraka}
                    </h4>
                    <p className="text-sm text-white/60 leading-relaxed">
                      직업·사회적 역할에서 영혼을 보필하는 행성입니다.
                    </p>
                  </div>

                  <div className="glass p-8 rounded-[2rem] border-celestial-cyan/20 bg-celestial-cyan/5">
                    <p className="text-celestial-cyan/80 text-sm font-bold uppercase tracking-wider mb-2">
                      파트너 지표 (Darakaraka)
                    </p>
                    <h4 className="text-3xl font-bold text-white mb-4">
                      {report.report.primary_karakas.darakaraka}
                    </h4>
                    <p className="text-sm text-white/60 leading-relaxed">
                      배우자·가까운 파트너와의 관계 패턴을 나타내는 행성입니다.
                    </p>
                  </div>
                </div>

                {/* Dasha + 전체 강도 */}
                <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                  <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
                    <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-2">
                      현재 대운 (Dasha)
                    </p>
                    <h4 className="text-3xl font-bold text-white mb-4">
                      {report.report.dasha_focus.replace("Current Major Period: ", "")}
                    </h4>
                    <div className="flex items-center gap-2 text-sm text-white/60">
                      <Clock className="w-4 h-4" />
                      <span>인생의 현재 단계에서 가장 강력한 영향을 미치는 기운입니다.</span>
                    </div>
                  </div>

                  <div className="glass p-8 rounded-[2rem]">
                    <p className="text-brand-400 text-sm font-bold uppercase tracking-wider mb-2">
                      전체 차트 강도
                    </p>
                    <div className="flex items-baseline gap-2 mb-4">
                      <h4 className="text-5xl font-black text-gradient leading-none">
                        {Math.round(report.report.overall_strength_score)}
                      </h4>
                      <span className="text-white/20 font-bold">/ 600</span>
                    </div>
                    <div className="w-full bg-white/5 h-2 rounded-full overflow-hidden">
                      <div
                        className="bg-celestial-purple h-full rounded-full transition-all duration-1000"
                        style={{ width: `${(report.report.overall_strength_score / 600) * 100}%` }}
                      />
                    </div>
                  </div>
                </div>

                {/* Secondary Info */}
                <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
                  <div className="glass p-10 rounded-[2.5rem]">
                    <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
                      <Star className="w-6 h-6 text-celestial-gold" />
                      낙샤트라 청사진
                    </h5>
                    <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
                      <p className="text-white text-lg font-medium leading-relaxed">
                        {report.report.nakshatra_info}
                      </p>
                    </div>
                  </div>

                  <div className="glass p-10 rounded-[2.5rem]">
                    <h5 className="text-xl font-bold text-white mb-8 flex items-center gap-3">
                      <Shield className="w-6 h-6 text-celestial-cyan" />
                      현재 운세 경고 (사데사티)
                    </h5>
                    <div className="p-6 bg-white/5 rounded-2xl border border-white/5">
                      <p className="text-white text-lg font-medium leading-relaxed">
                        {report.report.sade_sati}
                      </p>
                    </div>
                  </div>
                </div>

                {/* Yoga 섹션 */}
                {report.report.yogas && report.report.yogas.length > 0 && (
                  <section>
                    <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
                      <Sparkles className="w-6 h-6 text-celestial-gold" />
                      베딕 요가 (Yoga) 분석
                    </h5>
                    <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
                      {report.report.yogas.map((yoga: any, i: number) => {
                        const quality = typeof yoga.quality === "string" ? yoga.quality : Object.keys(yoga.quality)[0];
                        const weakReason = typeof yoga.quality === "object" && yoga.quality.Weak ? yoga.quality.Weak : null;
                        const qColor =
                          quality === "VeryHigh" ? "text-green-400 border-green-500/30 bg-green-500/10"
                          : quality === "High" ? "text-celestial-cyan border-celestial-cyan/30 bg-celestial-cyan/10"
                          : quality === "Medium" ? "text-celestial-gold border-celestial-gold/30 bg-celestial-gold/10"
                          : "text-white/50 border-white/20 bg-white/5";
                        return (
                          <div key={i} className={`p-5 rounded-2xl border ${qColor}`}>
                            <div className="flex items-start justify-between mb-2">
                              <p className="text-sm font-bold text-white">{yoga.name}</p>
                              <span className={`text-[10px] font-black px-2 py-0.5 rounded-full border shrink-0 ml-2 ${qColor}`}>{quality}</span>
                            </div>
                            <p className="text-xs text-white/50 leading-relaxed mb-2">{yoga.description}</p>
                            {weakReason && <p className="text-xs text-white/30 italic">※ {weakReason}</p>}
                            {yoga.planets_involved?.length > 0 && (
                              <div className="flex flex-wrap gap-1 mt-2">
                                {yoga.planets_involved.map((pl: string, j: number) => (
                                  <span key={j} className="text-[10px] px-2 py-0.5 rounded-full bg-white/10 text-white/60 border border-white/10">{pl}</span>
                                ))}
                              </div>
                            )}
                          </div>
                        );
                      })}
                    </div>
                  </section>
                )}

                {/* House Grid */}
                <section>
                  <h5 className="text-xl font-bold text-white mb-6">하우스(Bhava)별 에너지 역량</h5>
                  <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-4">
                    {report.report.house_summary.map((house: any) => (
                      <div
                        key={house.house}
                        className="glass p-6 rounded-2xl text-center glass-hover cursor-help"
                      >
                        <p className="text-xs text-white/30 font-bold mb-1">하우스 {house.house}</p>
                        <p className="text-2xl font-bold text-white mb-2">
                          {Math.round(house.total_score)}
                        </p>
                        <span
                          className={cn(
                            "px-2 py-0.5 rounded text-[10px] font-black uppercase",
                            house.rating === "Excellent"
                              ? "bg-green-500/20 text-green-400"
                              : house.rating === "Strong"
                                ? "bg-blue-500/20 text-blue-400"
                                : house.rating === "Average"
                                  ? "bg-yellow-500/20 text-yellow-400"
                                  : "bg-red-500/20 text-red-400"
                          )}
                        >
                          {house.rating}
                        </span>
                      </div>
                    ))}
                  </div>
                </section>
              </motion.div>
            )
          )}
        </AnimatePresence>
      </main>
      <Analytics />
    </div>
  );
}

export default App;
