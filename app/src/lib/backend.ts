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
    is_lunar: boolean; is_leap_month: boolean;
    lat: number; lon: number; timezone: string;
    unknown_time?: boolean;
}

export interface SajuArgs extends AnalysisArgs {
    is_male: boolean;
    use_night_rat_hour?: boolean;
}

export interface TransitArgs extends SajuArgs {
    // new context (required)
    now_utc: string;
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
            args.is_lunar, args.is_leap_month, args.lat, args.lon, args.timezone,
            args.unknown_time ?? false,
            new Date().toISOString()
        ) as Promise<VedicAnalysisResult>;
    }

    async getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult> {
        const wasm = await getWasmModule();
        return wasm.get_saju_analysis(
            args.year, args.month, args.day, args.hour, args.minute,
            args.is_lunar, args.is_leap_month, args.is_male,
            args.use_night_rat_hour ?? false,
            args.lon, args.lat, args.timezone,
            args.unknown_time ?? false
        ) as Promise<SajuAnalysisResult>;
    }

    async getTransitAnalysis(args: TransitArgs): Promise<TransitResult> {
        const wasm = await getWasmModule();
        return wasm.get_transit_analysis(
            args.year, args.month, args.day, args.hour, args.minute,
            args.is_lunar, args.is_leap_month, args.is_male,
            args.use_night_rat_hour ?? false,
            args.lon, args.lat, args.timezone,
            args.unknown_time ?? false,
            args.now_utc ?? new Date().toISOString()
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
            ...args,
            now_utc: new Date().toISOString()
        } as unknown as Record<string, unknown>);
    }

    async getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult> {
        return invoke("get_saju_analysis", args as unknown as Record<string, unknown>);
    }

    async getTransitAnalysis(args: TransitArgs): Promise<TransitResult> {
        const finalArgs = {
            ...args,
            now_utc: args.now_utc ?? new Date().toISOString()
        };
        return invoke("get_transit_analysis", finalArgs as unknown as Record<string, unknown>);
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
