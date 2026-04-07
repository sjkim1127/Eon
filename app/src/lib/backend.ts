import { invoke } from "@tauri-apps/api/core";
import type {
    VedicAnalysisResult,
    SajuAnalysisResult,
    TransitResult,
} from "../types";
import type { TierResult } from "../types/analysis";

export interface CurrentContextDto {
    now_utc: string; // ISO 8601
    analysis_timezone: string;
}

export interface AnalysisArgs {
    year: number; month: number; day: number; hour: number; minute: number;
    isLunar: boolean; isLeapMonth: boolean;
    lat: number; lon: number; timezone: string;
    unknownTime?: boolean;
}

export interface SajuArgs extends AnalysisArgs {
    isMale: boolean;
    useNightRatHour?: boolean;
}

export interface TransitArgs extends SajuArgs {
    nowUtc: string;
}


export interface BackendClient {
    getVedicAnalysis(args: AnalysisArgs): Promise<VedicAnalysisResult>;
    getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult>;
    getTransitAnalysis(args: TransitArgs): Promise<TransitResult>;
    getDestinyTier(saju: SajuAnalysisResult, vedic: VedicAnalysisResult, transit: TransitResult | null): Promise<TierResult>;
}

// WASM Module Loading Cache
let wasmPromiseCache: Promise<typeof import("eon-wasm")> | null = null;

const getWasmModule = async (): Promise<typeof import("eon-wasm")> => {
    if (!wasmPromiseCache) {
        wasmPromiseCache = (async () => {
            try {
                const mod = await import("eon-wasm");
                if (mod.default && typeof mod.default === "function") {
                    await mod.default();
                }
                return mod;
            } catch (err) {
                console.error("Failed to load or initialize WASM module:", err);
                throw new Error("WASM module initialization failed.");
            }
        })();
    }
    return wasmPromiseCache;
};

export class WasmBackendClient implements BackendClient {
    async getVedicAnalysis(args: AnalysisArgs): Promise<VedicAnalysisResult> {
        const wasm = await getWasmModule();
        return wasm.get_vedic_analysis(
            args.year, args.month, args.day, args.hour, args.minute,
            args.isLunar, args.isLeapMonth, args.lat, args.lon, args.timezone,
            args.unknownTime ?? false,
            new Date().toISOString()
        ) as Promise<VedicAnalysisResult>;
    }

    async getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult> {
        const wasm = await getWasmModule();
        return wasm.get_saju_analysis(
            args.year, args.month, args.day, args.hour, args.minute,
            args.isLunar, args.isLeapMonth, args.isMale,
            args.useNightRatHour ?? false,
            args.lon, args.lat, args.timezone,
            args.unknownTime ?? false
        ) as Promise<SajuAnalysisResult>;
    }

    async getTransitAnalysis(args: TransitArgs): Promise<TransitResult> {
        const wasm = await getWasmModule();
        return wasm.get_transit_analysis(
            args.year, args.month, args.day, args.hour, args.minute,
            args.isLunar, args.isLeapMonth, args.isMale,
            args.useNightRatHour ?? false,
            args.lon, args.lat, args.timezone,
            args.unknownTime ?? false,
            args.nowUtc ?? new Date().toISOString()
        ) as Promise<TransitResult>;
    }

    async getDestinyTier(saju: SajuAnalysisResult, vedic: VedicAnalysisResult, transit: TransitResult | null): Promise<TierResult> {
        const wasm = await getWasmModule();
        return (wasm as any).get_destiny_tier_analysis(saju, vedic, transit) as Promise<TierResult>;
    }
}

export class TauriBackendClient implements BackendClient {
    async getVedicAnalysis(args: AnalysisArgs): Promise<VedicAnalysisResult> {
        return invoke("get_vedic_analysis", {
            year: args.year,
            month: args.month,
            day: args.day,
            hour: args.hour,
            minute: args.minute,
            is_lunar: args.isLunar,
            is_leap_month: args.isLeapMonth,
            lat: args.lat,
            lon: args.lon,
            timezone: args.timezone,
            unknown_time: args.unknownTime,
            now_utc: new Date().toISOString()
        });
    }

    async getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult> {
        return invoke("get_saju_analysis", {
            year: args.year,
            month: args.month,
            day: args.day,
            hour: args.hour,
            minute: args.minute,
            is_lunar: args.isLunar,
            is_leap_month: args.isLeapMonth,
            is_male: args.isMale,
            use_night_rat_hour: args.useNightRatHour,
            lon: args.lon,
            lat: args.lat,
            timezone: args.timezone,
            unknown_time: args.unknownTime
        });
    }

    async getTransitAnalysis(args: TransitArgs): Promise<TransitResult> {
        return invoke("get_transit_analysis", {
            year: args.year,
            month: args.month,
            day: args.day,
            hour: args.hour,
            minute: args.minute,
            is_lunar: args.isLunar,
            is_leap_month: args.isLeapMonth,
            is_male: args.isMale,
            use_night_rat_hour: args.useNightRatHour,
            lon: args.lon,
            lat: args.lat,
            timezone: args.timezone,
            unknown_time: args.unknownTime,
            now_utc: args.nowUtc ?? new Date().toISOString()
        });
    }

    async getDestinyTier(saju: SajuAnalysisResult, vedic: VedicAnalysisResult, transit: TransitResult | null): Promise<TierResult> {
        return invoke("get_destiny_tier_analysis", { sajuVal: saju, vedicVal: vedic, transitVal: transit });
    }
}

declare global {
    interface Window {
        __TAURI_INTERNALS__?: unknown;
    }
}

export const isTauri = () => typeof window !== "undefined" && window.__TAURI_INTERNALS__ !== undefined;

export const backendClient: BackendClient = isTauri() ? new TauriBackendClient() : new WasmBackendClient();
