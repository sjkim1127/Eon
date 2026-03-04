import { useState, useRef, useEffect, useCallback } from "react";
import { MapPin, Loader2, Search } from "lucide-react";
import { searchCities, getTimezone, type GeoCity } from "../../utils/geonames";
import { KOREAN_CITIES } from "../../constants";

interface CitySearchInputProps {
    /** 현재 선택된 도시 표시 라벨 */
    selectedLabel: string;
    /** 도시 선택 시 콜백 */
    onSelect: (city: { name: string; lat: number; lon: number; timezone: string }) => void;
}

export function CitySearchInput({ selectedLabel, onSelect }: CitySearchInputProps) {
    const [query, setQuery] = useState("");
    const [results, setResults] = useState<GeoCity[]>([]);
    const [isOpen, setIsOpen] = useState(false);
    const [loading, setLoading] = useState(false);
    const [useFallback, setUseFallback] = useState(false);
    const inputRef = useRef<HTMLInputElement>(null);
    const containerRef = useRef<HTMLDivElement>(null);
    const debounceRef = useRef<ReturnType<typeof setTimeout> | null>(null);

    // Close dropdown on outside click
    useEffect(() => {
        const handler = (e: MouseEvent) => {
            if (containerRef.current && !containerRef.current.contains(e.target as Node)) {
                setIsOpen(false);
            }
        };
        document.addEventListener("mousedown", handler);
        return () => document.removeEventListener("mousedown", handler);
    }, []);

    // Debounced search
    const doSearch = useCallback(async (q: string) => {
        if (q.length < 2) {
            setResults([]);
            setLoading(false);
            return;
        }
        setLoading(true);
        try {
            const cities = await searchCities(q, 8);
            if (cities.length === 0 && q.length >= 2) {
                // API might be down or rate-limited → fall back to local
                setUseFallback(true);
            }
            setResults(cities);
        } catch {
            setUseFallback(true);
            setResults([]);
        } finally {
            setLoading(false);
        }
    }, []);

    const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const val = e.target.value;
        setQuery(val);
        setIsOpen(true);
        setUseFallback(false);

        if (debounceRef.current) clearTimeout(debounceRef.current);
        debounceRef.current = setTimeout(() => doSearch(val), 300);
    };

    const handleSelect = async (city: GeoCity) => {
        // Get timezone from coordinates
        const tz = await getTimezone(city.lat, city.lon);
        const label = `${city.name}, ${city.country}`;
        setQuery("");
        setIsOpen(false);
        setResults([]);
        onSelect({
            name: label,
            lat: city.lat,
            lon: city.lon,
            timezone: tz ?? "UTC",
        });
    };

    const handleFallbackSelect = (e: React.ChangeEvent<HTMLSelectElement>) => {
        const city = KOREAN_CITIES.find((c) => c.name === e.target.value);
        if (city) {
            onSelect({ name: city.name, lat: city.lat, lon: city.lon, timezone: city.timezone });
        }
    };

    // Fallback: show Korean cities dropdown
    if (useFallback) {
        return (
            <div className="relative">
                <label className="block text-xs text-white/40 mb-1.5 font-medium flex items-center gap-1">
                    <MapPin className="w-3 h-3" /> 출생지
                </label>
                <select
                    value={selectedLabel}
                    onChange={handleFallbackSelect}
                    className="w-full bg-white/5 border border-white/10 rounded-xl px-3 pr-8 py-2.5 text-white text-sm focus:border-celestial-purple/50 focus:outline-none appearance-none cursor-pointer"
                >
                    {KOREAN_CITIES.map((city) => (
                        <option key={city.name} value={city.name} className="bg-gray-900">
                            {city.name}
                        </option>
                    ))}
                </select>
                <button
                    type="button"
                    onClick={() => setUseFallback(false)}
                    className="absolute right-2 top-8 text-[10px] text-celestial-cyan/60 hover:text-celestial-cyan"
                >
                    검색
                </button>
            </div>
        );
    }

    return (
        <div className="relative" ref={containerRef}>
            <label className="block text-xs text-white/40 mb-1.5 font-medium flex items-center gap-1">
                <MapPin className="w-3 h-3" /> 출생지
            </label>

            {/* Current value display / search toggle */}
            {!isOpen && !query ? (
                <button
                    type="button"
                    onClick={() => {
                        setIsOpen(true);
                        setTimeout(() => inputRef.current?.focus(), 50);
                    }}
                    className="w-full text-left bg-white/5 border border-white/10 rounded-xl px-3 py-2.5 text-white text-sm hover:border-celestial-purple/30 focus:border-celestial-purple/50 focus:outline-none transition-colors flex items-center gap-2"
                >
                    <Search className="w-3.5 h-3.5 text-white/30 shrink-0" />
                    <span className="truncate">{selectedLabel || "도시 검색..."}</span>
                </button>
            ) : (
                <div className="relative">
                    <input
                        ref={inputRef}
                        type="text"
                        value={query}
                        onChange={handleInputChange}
                        onFocus={() => setIsOpen(true)}
                        placeholder="도시 이름 입력 (예: Seoul, Tokyo, New York)"
                        className="w-full bg-white/5 border border-celestial-purple/50 rounded-xl px-3 py-2.5 text-white text-sm focus:outline-none focus:ring-1 focus:ring-celestial-purple/30 placeholder:text-white/25"
                        autoComplete="off"
                    />
                    {loading && (
                        <Loader2 className="w-4 h-4 text-celestial-purple absolute right-3 top-3 animate-spin" />
                    )}
                </div>
            )}

            {/* Dropdown results */}
            {isOpen && (results.length > 0 || (query.length >= 2 && !loading)) && (
                <div className="absolute z-50 mt-1 w-full max-h-48 overflow-y-auto bg-gray-900/95 backdrop-blur-xl border border-white/15 rounded-xl shadow-2xl shadow-black/50">
                    {results.length > 0 ? (
                        results.map((city) => (
                            <button
                                key={city.geonameId || `${city.lat}-${city.lon}`}
                                type="button"
                                onClick={() => handleSelect(city)}
                                className="w-full text-left px-3 py-2 text-sm text-white/80 hover:bg-white/10 hover:text-white transition-colors first:rounded-t-xl last:rounded-b-xl border-b border-white/5 last:border-b-0"
                            >
                                <span className="font-semibold text-white">{city.name}</span>
                                {city.admin1 && <span className="text-white/40">, {city.admin1}</span>}
                                <span className="text-white/40">, {city.country}</span>
                                <span className="text-white/25 text-xs ml-2">
                                    ({city.lat.toFixed(2)}, {city.lon.toFixed(2)})
                                </span>
                            </button>
                        ))
                    ) : (
                        query.length >= 2 && !loading && (
                            <div className="px-3 py-3 text-xs text-white/30 text-center">
                                검색 결과 없음
                            </div>
                        )
                    )}
                </div>
            )}
        </div>
    );
}
