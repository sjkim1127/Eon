/**
 * GeoNames API utility — 도시 검색 + 타임존 조회
 * https://www.geonames.org/export/geonames-search.html
 *
 * 환경변수: VITE_GEONAMES_USERNAME (무료 계정 username)
 */

const GEONAMES_BASE = "https://secure.geonames.org";

export interface GeoCity {
    name: string;
    /** 행정구역 (예: Gyeonggi-do) */
    admin1: string;
    country: string;
    countryCode: string;
    lat: number;
    lon: number;
    population: number;
    /** geonames 고유 ID */
    geonameId: number;
}

function getUsername(): string {
    return (
        (typeof import.meta !== "undefined" && (import.meta as any).env?.VITE_GEONAMES_USERNAME) ||
        "demo"
    );
}

/**
 * 도시 자동완성 검색
 * @param query 검색어 (최소 2자)
 * @param maxRows 최대 결과 수 (기본 8)
 */
export async function searchCities(
    query: string,
    maxRows = 8,
): Promise<GeoCity[]> {
    if (!query || query.length < 2) return [];

    const username = getUsername();
    const url = `${GEONAMES_BASE}/searchJSON?q=${encodeURIComponent(query)}&maxRows=${maxRows}&featureClass=P&orderby=population&username=${username}&style=MEDIUM`;

    try {
        const res = await fetch(url);
        if (!res.ok) throw new Error(`GeoNames HTTP ${res.status}`);
        const data = await res.json();

        if (!data.geonames || !Array.isArray(data.geonames)) return [];

        return data.geonames.map((g: any) => ({
            name: g.name ?? g.toponymName ?? "",
            admin1: g.adminName1 ?? "",
            country: g.countryName ?? "",
            countryCode: g.countryCode ?? "",
            lat: parseFloat(g.lat),
            lon: parseFloat(g.lng),
            population: g.population ?? 0,
            geonameId: g.geonameId ?? 0,
        }));
    } catch (err) {
        console.warn("[GeoNames] searchCities error:", err);
        throw err;
    }
}

/**
 * 좌표 기반 타임존 조회
 * @returns IANA 타임존 ID (예: "Asia/Seoul") 또는 null
 */
export async function getTimezone(
    lat: number,
    lon: number,
): Promise<string | null> {
    const username = getUsername();
    const url = `${GEONAMES_BASE}/timezoneJSON?lat=${lat}&lng=${lon}&username=${username}`;

    try {
        const res = await fetch(url);
        if (!res.ok) throw new Error(`GeoNames HTTP ${res.status}`);
        const data = await res.json();
        return data.timezoneId ?? null;
    } catch (err) {
        console.warn("[GeoNames] getTimezone error:", err);
        return null;
    }
}
