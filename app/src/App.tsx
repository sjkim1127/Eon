import { useState } from "react";
import { get_vedic_analysis, get_saju_analysis, get_transit_analysis } from "./lib/api";
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
      </motion.div>
    );
  };

  const renderVedicCharts = () => {
    if (!report || !report.chart || !report.chart.planets) return null;
    const planets = report.chart.planets;

    // 대표적인 분할 차트 매핑
    const VARGA_MAP = [
      { id: "rasi", label: "D1: Rasi (기본 차트)", key: "rasi" },
      { id: "navamsa", label: "D9: Navamsa (영혼/결혼)", key: "navamsa_rasi" },
      { id: "dasamsa", label: "D10: Dasamsa (직업/사회)", key: "dasamsa_rasi" },
      { id: "trimsamsa", label: "D30: Trimsamsa (재앙/질병)", key: "trimsamsa_rasi" },
      { id: "shashtyamsa", label: "D60: Shashtyamsa (카르마/과거생)", key: "shashtyamsa_rasi" },
      { id: "dwadasdwadasamsa", label: "D144: Dwadasdwadasamsa (궁극적 결실)", key: "dwadasdwadasamsa_rasi" },
    ];

    return (
      <motion.div
        key="vedic-charts"
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -20 }}
        className="space-y-8"
      >
        <div className="glass p-8 rounded-[2rem]">
          <h5 className="text-xl font-bold text-white mb-6 flex items-center gap-3">
            <Star className="w-6 h-6 text-celestial-purple" />
            베딕 분할 차트 (D1 - D144)
          </h5>
          <p className="text-sm text-white/60 mb-8 leading-relaxed">
            각 행성이 위치한 하우스(Rasi Sign 1~12)를 다차원 분할 차트(Varga)에 따라 심층적으로 분석합니다.
          </p>

          <div className="grid grid-cols-1 xl:grid-cols-2 gap-8">
            {VARGA_MAP.map((varga) => {
              // 해당 varga에 배치된 행성들을 그룹화 (1번~12번 Sign)
              const grouped: Record<number, string[]> = {};
              planets.forEach((p: any) => {
                const sign = p[varga.key];
                if (!grouped[sign]) grouped[sign] = [];
                grouped[sign].push(p.planet);
              });

              return (
                <div key={varga.id} className="bg-white/5 border border-white/10 p-6 rounded-2xl relative">
                  <h6 className="text-sm font-bold text-brand-400 uppercase tracking-wider mb-4 border-b border-white/10 pb-2">
                    {varga.label}
                  </h6>
                  <div className="grid grid-cols-4 gap-2">
                    {Array.from({ length: 12 }, (_, i) => i + 1).map((sign) => (
                      <div key={sign} className="p-2 bg-black/20 rounded-lg min-h-[60px] flex flex-col items-center justify-center border border-white/5">
                        <span className="text-[10px] text-white/30 font-black mb-1">SIGN {sign}</span>
                        <div className="flex flex-wrap items-center justify-center gap-1">
                          {grouped[sign]?.map((pl, idx) => (
                            <span key={idx} className="text-xs font-bold text-white px-1.5 py-0.5 rounded-full bg-celestial-purple/30 border border-celestial-purple/50">
                              {pl}
                            </span>
                          ))}
                          {!grouped[sign] && <span className="text-white/10">—</span>}
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              );
            })}
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
                    {node.is_bottleneck && (
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
                <p className="text-4xl font-black text-white">{entropy.shannon_entropy?.toFixed(3) ?? "—"}</p>
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
            <div className="space-y-1 text-sm text-white/60">
              <p>천간 십성: <span className="text-white font-semibold">{yr?.stem_god?.hangul ?? yr?.stem_god ?? "—"}</span></p>
              <p>지지 십성: <span className="text-white font-semibold">{yr?.branch_god?.hangul ?? yr?.branch_god ?? "—"}</span></p>
              {yr?.special_events?.length > 0 && (
                <p className="text-red-400 font-semibold">⚠️ {yr.special_events.join(" / ")}</p>
              )}
            </div>
          </div>
          <div className="glass p-8 rounded-[2rem] border-celestial-purple/20 bg-celestial-purple/5">
            <p className="text-celestial-purple/80 text-sm font-bold uppercase tracking-wider mb-3">월운 (月運) — {mo?.month}월</p>
            <h4 className="text-4xl font-black text-white mb-3">{mo?.ganzi?.hangul ?? "—"}</h4>
            <div className="space-y-1 text-sm text-white/60">
              <p>천간 십성: <span className="text-white font-semibold">{mo?.stem_god?.hangul ?? mo?.stem_god ?? "—"}</span></p>
              <p>지지 십성: <span className="text-white font-semibold">{mo?.branch_god?.hangul ?? mo?.branch_god ?? "—"}</span></p>
            </div>
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
          ) : (
            report && report.report && (
              <motion.div
                key="results"
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0, y: -20 }}
                className="space-y-8"
              >
                {/* Hero Statistics */}
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
                      Atmakaraka - 이번 생에서 영혼이 추구하는 가장 강력한 욕망과 핵심 과제를 나타냅니다.
                    </p>
                  </div>

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
