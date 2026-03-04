import { Calendar, User, Loader2, ChevronDown, Clock, HelpCircle } from "lucide-react";
import { CitySearchInput } from "./CitySearchInput";
import type { BirthData } from "../../types";

interface BirthInputFormProps {
  birthData: BirthData;
  setBirthData: React.Dispatch<React.SetStateAction<BirthData>>;
  selectedCity: string;
  onCitySelect: (city: { name: string; lat: number; lon: number; timezone: string }) => void;
  isMale: boolean;
  setIsMale: (v: boolean) => void;
  isDST: boolean;
  loading: boolean;
  onAnalysis: () => void;
  sajuReport: any;
  /** 드로어 내부 렌더링 시 외부 래퍼·제목 제거 */
  compact?: boolean;
  /** 드로어 닫기 콜백 (compact 모드에서 분석 버튼 클릭 시 호출) */
  onClose?: () => void;
}

export function BirthInputForm({
  birthData,
  setBirthData,
  selectedCity,
  onCitySelect,
  isMale,
  setIsMale,
  isDST,
  loading,
  onAnalysis,
  sajuReport,
  compact = false,
  onClose,
}: BirthInputFormProps) {
  const unknownTime = birthData.unknown_time ?? false;

  // 해당 년/월의 최대 일수 계산
  const daysInMonth = new Date(birthData.year, birthData.month, 0).getDate();

  // 년 범위: 1924 ~ 현재년
  const currentYear = new Date().getFullYear();
  const years = Array.from(
    { length: currentYear - 1923 },
    (_, i) => currentYear - i
  );

  const toggleUnknownTime = () => {
    setBirthData((prev) => ({
      ...prev,
      unknown_time: !prev.unknown_time,
      // 시간 모름으로 전환 시 정오(12:00)로 초기화
      hour: !prev.unknown_time ? 12 : prev.hour,
      minute: !prev.unknown_time ? 0 : prev.minute,
    }));
  };

  const inner = (
    <div>
      {!compact && (
        <h5 className="text-lg font-bold text-white mb-5 flex items-center gap-3">
          <Calendar className="w-5 h-5 text-celestial-purple" />
          출생 정보 입력
          {isDST && (
            <span className="ml-auto text-xs px-3 py-1 rounded-full bg-amber-500/20 text-amber-400 border border-amber-500/30 font-semibold animate-pulse">
              ☀️ 서머타임 자동 적용
            </span>
          )}
        </h5>
      )}
      {compact && isDST && (
        <div className="mb-4 text-xs px-3 py-2 rounded-xl bg-amber-500/10 border border-amber-500/20 text-amber-400 font-semibold">
          ☀️ 서머타임 자동 적용
        </div>
      )}
      {/* 달력 구분 (양력/음력) */}
      <div className="relative mb-3">
        <label className="block text-xs text-white/40 mb-1.5 font-medium">달력 구분</label>
        <select
          value={
            birthData.is_lunar
              ? birthData.is_leap_month
                ? "lunar_leap"
                : "lunar"
              : "solar"
          }
          onChange={(e) => {
            const val = e.target.value;
            setBirthData((prev) => ({
              ...prev,
              is_lunar: val === "lunar" || val === "lunar_leap",
              is_leap_month: val === "lunar_leap",
            }));
          }}
          className="w-full bg-white/5 border border-white/10 rounded-xl px-3 pr-8 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none appearance-none cursor-pointer"
        >
          <option value="solar" className="bg-gray-900">☀️ 양력 (Solar)</option>
          <option value="lunar" className="bg-gray-900">🌙 음력 (Lunar)</option>
          <option value="lunar_leap" className="bg-gray-900">🌙 음력 윤달 (Leap)</option>
        </select>
        <ChevronDown className="w-4 h-4 text-white/40 absolute right-2.5 top-9 pointer-events-none" />
      </div>

      {/* 날짜 행 (년/월/일) */}
      <div className="grid grid-cols-3 gap-3 mb-3">
        {/* 년 */}
        <div className="relative">
          <label className="block text-xs text-white/40 mb-1.5 font-medium">년</label>
          <select
            value={birthData.year}
            onChange={(e) => {
              const newYear = Number(e.target.value);
              const newMax = new Date(newYear, birthData.month, 0).getDate();
              setBirthData((prev) => ({
                ...prev,
                year: newYear,
                day: Math.min(prev.day, newMax),
              }));
            }}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 pr-8 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none appearance-none cursor-pointer"
          >
            {years.map((y) => (
              <option key={y} value={y} className="bg-gray-900">
                {y}
              </option>
            ))}
          </select>
          <ChevronDown className="w-4 h-4 text-white/40 absolute right-2.5 top-9 pointer-events-none" />
        </div>
        {/* 월 */}
        <div className="relative">
          <label className="block text-xs text-white/40 mb-1.5 font-medium">월</label>
          <select
            value={birthData.month}
            onChange={(e) => {
              const newMonth = Number(e.target.value);
              const newMax = new Date(birthData.year, newMonth, 0).getDate();
              setBirthData((prev) => ({
                ...prev,
                month: newMonth,
                day: Math.min(prev.day, newMax),
              }));
            }}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 pr-8 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none appearance-none cursor-pointer"
          >
            {Array.from({ length: 12 }, (_, i) => (
              <option key={i + 1} value={i + 1} className="bg-gray-900">
                {i + 1}월
              </option>
            ))}
          </select>
          <ChevronDown className="w-4 h-4 text-white/40 absolute right-2.5 top-9 pointer-events-none" />
        </div>
        {/* 일 */}
        <div className="relative">
          <label className="block text-xs text-white/40 mb-1.5 font-medium">일</label>
          <select
            value={birthData.day}
            onChange={(e) => setBirthData((prev) => ({ ...prev, day: Number(e.target.value) }))}
            className="w-full bg-white/5 border border-white/10 rounded-xl px-3 pr-8 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none appearance-none cursor-pointer"
          >
            {Array.from({ length: daysInMonth }, (_, i) => (
              <option key={i + 1} value={i + 1} className="bg-gray-900">
                {i + 1}일
              </option>
            ))}
          </select>
          <ChevronDown className="w-4 h-4 text-white/40 absolute right-2.5 top-9 pointer-events-none" />
        </div>
      </div>

      {/* 시간 + 출생지 행 */}
      <div className="grid grid-cols-3 gap-3 mb-4">
        {/* 시 */}
        <div className="relative">
          <label className="block text-xs mb-1.5 font-medium flex items-center gap-1.5">
            <span className={unknownTime ? "text-white/20" : "text-white/40"}>시</span>
            {unknownTime && (
              <span className="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-400 border border-amber-400/30">
                정오
              </span>
            )}
          </label>
          <select
            value={birthData.hour}
            onChange={(e) => setBirthData((prev) => ({ ...prev, hour: Number(e.target.value) }))}
            disabled={unknownTime}
            className={`w-full bg-white/5 border rounded-xl px-3 pr-8 py-2.5 text-sm appearance-none transition-all
              ${unknownTime
                ? "border-white/5 text-white/20 cursor-not-allowed opacity-40"
                : "border-white/10 text-white focus:border-celestial-purple/50 focus:outline-none cursor-pointer"
              }`}
          >
            {Array.from({ length: 24 }, (_, i) => (
              <option key={i} value={i} className="bg-gray-900">
                {String(i).padStart(2, "0")}시
              </option>
            ))}
          </select>
          <ChevronDown className={`w-4 h-4 absolute right-2.5 top-9 pointer-events-none ${unknownTime ? "text-white/10" : "text-white/40"}`} />
        </div>
        {/* 분 */}
        <div className="relative">
          <label className="block text-xs mb-1.5 font-medium">
            <span className={unknownTime ? "text-white/20" : "text-white/40"}>분</span>
          </label>
          <select
            value={birthData.minute}
            onChange={(e) => setBirthData((prev) => ({ ...prev, minute: Number(e.target.value) }))}
            disabled={unknownTime}
            className={`w-full bg-white/5 border rounded-xl px-3 pr-8 py-2.5 text-sm appearance-none transition-all
              ${unknownTime
                ? "border-white/5 text-white/20 cursor-not-allowed opacity-40"
                : "border-white/10 text-white focus:border-celestial-purple/50 focus:outline-none cursor-pointer"
              }`}
          >
            {Array.from({ length: 60 }, (_, i) => i).map((m) => (
              <option key={m} value={m} className="bg-gray-900">
                {String(m).padStart(2, "0")}분
              </option>
            ))}
          </select>
          <ChevronDown className={`w-4 h-4 absolute right-2.5 top-9 pointer-events-none ${unknownTime ? "text-white/10" : "text-white/40"}`} />
        </div>
        {/* 도시 */}
        <CitySearchInput
          selectedLabel={selectedCity}
          onSelect={onCitySelect}
        />
      </div>

      {/* 시간 모름 안내 배너 */}
      {unknownTime && (
        <div className="flex items-start gap-2.5 mb-4 px-4 py-3 rounded-xl bg-amber-500/10 border border-amber-500/25 text-amber-300/90 text-xs leading-relaxed">
          <HelpCircle className="w-4 h-4 mt-0.5 shrink-0 text-amber-400" />
          <span>
            <strong className="font-bold">시간 미상 모드</strong> — 정오(12:00, 午時)를 기준으로 분석합니다.
            시주(時柱) 및 시 기반 신살·용신 결과는 참고용이며 실제와 다를 수 있습니다.
          </span>
        </div>
      )}

      {/* 하단: 성별 + 시간모름 토글 + 분석 실행 + 보정 정보 */}
      <div className="flex items-center gap-3 flex-wrap">
        <button
          onClick={() => setIsMale(!isMale)}
          className="glass px-4 py-2.5 rounded-xl flex items-center gap-2 hover:bg-white/10 transition-all text-sm"
        >
          <User className="w-4 h-4 text-celestial-cyan" />
          <span className="text-white font-semibold">{isMale ? "남" : "여"}</span>
        </button>

        {/* 시간 모름 토글 */}
        <button
          onClick={toggleUnknownTime}
          className={`px-4 py-2.5 rounded-xl flex items-center gap-2 border transition-all text-sm font-semibold
            ${unknownTime
              ? "bg-amber-500/20 border-amber-500/50 text-amber-300"
              : "bg-white/5 border-white/10 text-white/50 hover:text-white/80 hover:bg-white/10"
            }`}
        >
          <Clock className="w-4 h-4" />
          {unknownTime ? "시간 미상 ✓" : "시간 모름"}
        </button>

        <button
          onClick={async () => { await Promise.resolve(onAnalysis()); onClose?.(); }}
          disabled={loading}
          className="bg-gradient-to-r from-celestial-purple to-brand-600 px-6 py-2.5 rounded-xl font-bold text-white text-sm shadow-lg shadow-indigo-500/20 hover:scale-105 active:scale-95 transition-all disabled:opacity-50 inline-flex items-center gap-2 w-full sm:w-auto justify-center"
        >
          {loading && <Loader2 className="w-4 h-4 animate-spin" />}
          {loading ? "분석 중..." : "통합 분석 시작"}
        </button>

        <div className="w-full lg:w-auto lg:ml-auto text-xs text-white/30 flex flex-wrap items-center gap-3 lg:gap-4">
          <span>
            위도 {birthData.lat.toFixed(2)}° / 경도 {birthData.lon.toFixed(2)}°
          </span>
          {birthData.is_lunar && (
            <span className="text-pink-400">
              {birthData.is_leap_month ? "음력(윤달)" : "음력"}
            </span>
          )}
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

  if (compact) return inner;
  return <div className="glass p-6 rounded-[2rem] mb-8">{inner}</div>;
}
