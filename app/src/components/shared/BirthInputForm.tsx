import { Calendar, MapPin, User } from "lucide-react";
import { KOREAN_CITIES } from "../../constants";
import type { BirthData } from "../../types";

interface BirthInputFormProps {
  birthData: BirthData;
  setBirthData: React.Dispatch<React.SetStateAction<BirthData>>;
  selectedCity: string;
  onCityChange: (cityName: string) => void;
  isMale: boolean;
  setIsMale: (v: boolean) => void;
  isDST: boolean;
  loading: boolean;
  onAnalysis: () => void;
  sajuReport: any;
}

export function BirthInputForm({
  birthData,
  setBirthData,
  selectedCity,
  onCityChange,
  isMale,
  setIsMale,
  isDST,
  loading,
  onAnalysis,
  sajuReport,
}: BirthInputFormProps) {
  return (
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
            onChange={(e) => onCityChange(e.target.value)}
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
          onClick={onAnalysis}
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
}
