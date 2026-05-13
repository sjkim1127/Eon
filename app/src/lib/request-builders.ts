import type {
    AnalysisRequest,
    SajuAnalysisRequest,
    VedicAnalysisRequest,
    TransitAnalysisRequest,
} from "../generated/eon-api";
import type { BirthData } from "../types";

export function buildAnalysisRequest(birthData: BirthData): AnalysisRequest {
    return {
        year: birthData.year,
        month: birthData.month,
        day: birthData.day,
        hour: birthData.hour,
        minute: birthData.minute,
        isLunar: birthData.isLunar ?? false,
        isLeapMonth: birthData.isLeapMonth ?? false,
        lat: birthData.lat,
        lon: birthData.lon,
        timezone: birthData.timezone,
        unknownTime: birthData.unknownTime ?? false,
    };
}

export function buildSajuRequest(birthData: BirthData, isMale: boolean): SajuAnalysisRequest {
    return {
        ...buildAnalysisRequest(birthData),
        isMale,
        useNightRatHour: birthData.useNightRatHour ?? false,
    };
}

export function buildVedicRequest(birthData: BirthData, nowUtc: string): VedicAnalysisRequest {
    return {
        ...buildAnalysisRequest(birthData),
        nowUtc,
        targetYear: null,
    };
}

export function buildTransitRequest(birthData: BirthData, isMale: boolean, nowUtc: string): TransitAnalysisRequest {
    return {
        ...buildSajuRequest(birthData, isMale),
        nowUtc,
    };
}
