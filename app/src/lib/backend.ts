import { invoke } from "@tauri-apps/api/core";
import type {
    VedicAnalysisResult,
    SajuAnalysisResult,
    TransitResult,
    CompatibilityAudit,
    AshtaKutaResult,
} from "../types";
import type { AiAuditReport, TierResult } from "../types/analysis";

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
    // legacy support
    current_year: number; current_month: number; current_day: number;
    // new context
    now_utc?: string;
}

export interface CompArgs {
    year1: number; month1: number; day1: number; hour1: number; minute1: number;
    is_lunar1: boolean; is_leap_month1: boolean;
    is_male1: boolean; lon1: number; lat1: number; timezone1: string;
    year2: number; month2: number; day2: number; hour2: number; minute2: number;
    is_lunar2: boolean; is_leap_month2: boolean;
    is_male2: boolean; lon2: number; lat2: number; timezone2: string;
    use_night_rat_hour1?: boolean;
    use_night_rat_hour2?: boolean;
}

export interface BackendClient {
    getVedicAnalysis(args: AnalysisArgs): Promise<VedicAnalysisResult>;
    getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult>;
    getTransitAnalysis(args: TransitArgs): Promise<TransitResult>;
    getDestinyTier(saju: SajuAnalysisResult, vedic: VedicAnalysisResult, transit: TransitResult | null): Promise<TierResult>;
    getSajuCompatibility(args: CompArgs): Promise<CompatibilityAudit>;
    getVedicCompatibility(args: CompArgs): Promise<AshtaKutaResult>;
    getAiAudit(args: SajuArgs): Promise<AiAuditReport>;
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
            args.is_lunar, args.is_leap_month, args.lat, args.lon, args.timezone
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
            args.current_year, args.current_month, args.current_day,
            args.unknown_time ?? false,
            args.now_utc ?? new Date().toISOString()
        ) as Promise<TransitResult>;
    }

    async getDestinyTier(saju: SajuAnalysisResult, vedic: VedicAnalysisResult, transit: TransitResult | null): Promise<TierResult> {
        const wasm = await getWasmModule();
        return (wasm as any).get_destiny_tier_analysis(saju, vedic, transit) as Promise<TierResult>;
    }

    async getSajuCompatibility(args: CompArgs): Promise<CompatibilityAudit> {
        const wasm = await getWasmModule();
        return wasm.get_saju_compatibility(
            args.year1, args.month1, args.day1, args.hour1, args.minute1,
            args.is_lunar1, args.is_leap_month1, args.is_male1, args.lon1, args.lat1,
            args.use_night_rat_hour1 ?? false,
            args.year2, args.month2, args.day2, args.hour2, args.minute2,
            args.is_lunar2, args.is_leap_month2, args.is_male2, args.lon2, args.lat2,
            args.use_night_rat_hour2 ?? false,
            args.timezone1, args.timezone2
        ) as Promise<CompatibilityAudit>;
    }

    async getVedicCompatibility(args: CompArgs): Promise<AshtaKutaResult> {
        const wasm = await getWasmModule();
        return wasm.get_vedic_compatibility(
            args.year1, args.month1, args.day1, args.hour1, args.minute1,
            args.is_lunar1, args.is_leap_month1, args.lat1, args.lon1,
            args.year2, args.month2, args.day2, args.hour2, args.minute2,
            args.is_lunar2, args.is_leap_month2, args.lat2, args.lon2,
            args.timezone1, args.timezone2
        ) as Promise<AshtaKutaResult>;
    }

    async getAiAudit(args: SajuArgs): Promise<AiAuditReport> {
        const wasm = await getWasmModule();
        return (wasm as any).get_ai_audit(
            args.year, args.month, args.day, args.hour, args.minute,
            args.is_lunar, args.is_leap_month, args.is_male,
            args.use_night_rat_hour ?? false,
            args.lon, args.lat, args.timezone,
            args.unknown_time ?? false
        ) as Promise<AiAuditReport>;
    }
}

export class TauriBackendClient implements BackendClient {
    async getVedicAnalysis(args: AnalysisArgs): Promise<VedicAnalysisResult> {
        return invoke("get_vedic_analysis", args as unknown as Record<string, unknown>);
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

    async getSajuCompatibility(args: CompArgs): Promise<CompatibilityAudit> {
        return invoke("get_saju_compatibility", args as unknown as Record<string, unknown>);
    }

    async getVedicCompatibility(args: CompArgs): Promise<AshtaKutaResult> {
        return invoke("get_vedic_compatibility", args as unknown as Record<string, unknown>);
    }

    async getAiAudit(args: SajuArgs): Promise<AiAuditReport> {
        return invoke("get_ai_audit", args as unknown as Record<string, unknown>);
    }
}

declare global {
    interface Window {
        __TAURI_INTERNALS__?: unknown;
    }
}

export const isTauri = () => typeof window !== "undefined" && window.__TAURI_INTERNALS__ !== undefined;

export const backendClient: BackendClient = isTauri() ? new TauriBackendClient() : new WasmBackendClient();
