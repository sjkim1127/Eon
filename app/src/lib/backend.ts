import { invoke } from "@tauri-apps/api/core";
import type {
    VedicAnalysisResult,
    SajuAnalysisResult,
    TransitResult,
    CompatibilityAudit,
    AshtaKutaResult,
} from "../types";

export interface AnalysisArgs {
    year: number; month: number; day: number; hour: number; minute: number;
    is_lunar: boolean; is_leap_month: boolean;
    lat: number; lon: number; timezone: string;
}

export interface SajuArgs extends AnalysisArgs {
    is_male: boolean;
}

export interface TransitArgs extends SajuArgs {
    current_year: number; current_month: number; current_day: number;
}

export interface CompArgs {
    year1: number; month1: number; day1: number; hour1: number; minute1: number;
    is_lunar1: boolean; is_leap_month1: boolean;
    is_male1: boolean; lon1: number; lat1: number; timezone1: string;
    year2: number; month2: number; day2: number; hour2: number; minute2: number;
    is_lunar2: boolean; is_leap_month2: boolean;
    is_male2: boolean; lon2: number; lat2: number; timezone2: string;
}

export interface BackendClient {
    getVedicAnalysis(args: AnalysisArgs): Promise<VedicAnalysisResult>;
    getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult>;
    getTransitAnalysis(args: TransitArgs): Promise<TransitResult>;
    getSajuCompatibility(args: CompArgs): Promise<CompatibilityAudit>;
    getVedicCompatibility(args: CompArgs): Promise<AshtaKutaResult>;
    getAiAudit(args: SajuArgs): Promise<unknown>;
}

// WASM Module Loading Cache
let wasmPromiseCache: Promise<typeof import("eon-wasm")> | null = null;
const getWasmModule = async () => {
    if (!wasmPromiseCache) {
        wasmPromiseCache = import("eon-wasm");
    }
    return wasmPromiseCache;
};

// JSON 파싱 헬퍼 함수
const processSajuResult = (result: any): SajuAnalysisResult => {
    if (result.timeline_json && result.report) {
        try {
            result.report.timeline = JSON.parse(result.timeline_json);
        } catch (e) {
            console.warn("[WASM] timeline_json parse failed:", e);
        }
    }
    return result as SajuAnalysisResult;
};

export class WasmBackendClient implements BackendClient {
    async getVedicAnalysis(args: AnalysisArgs): Promise<VedicAnalysisResult> {
        const wasm = await getWasmModule();
        return wasm.get_vedic_analysis(
            args.year, args.month, args.day, args.hour, args.minute,
            args.is_lunar, args.is_leap_month, args.lat, args.lon, args.timezone
        );
    }

    async getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult> {
        const wasm = await getWasmModule();
        const result = wasm.get_saju_analysis(
            args.year, args.month, args.day, args.hour, args.minute,
            args.is_lunar, args.is_leap_month, args.is_male,
            args.lon, args.lat, args.timezone
        );
        return processSajuResult(result);
    }

    async getTransitAnalysis(args: TransitArgs): Promise<TransitResult> {
        const wasm = await getWasmModule();
        return wasm.get_transit_analysis(
            args.year, args.month, args.day, args.hour, args.minute,
            args.is_lunar, args.is_leap_month, args.is_male,
            args.lon, args.lat, args.timezone,
            args.current_year, args.current_month, args.current_day
        );
    }

    async getSajuCompatibility(args: CompArgs): Promise<CompatibilityAudit> {
        const wasm = await getWasmModule();
        return wasm.get_saju_compatibility(
            args.year1, args.month1, args.day1, args.hour1, args.minute1,
            args.is_lunar1, args.is_leap_month1, args.is_male1, args.lon1, args.lat1,
            args.year2, args.month2, args.day2, args.hour2, args.minute2,
            args.is_lunar2, args.is_leap_month2, args.is_male2, args.lon2, args.lat2,
            args.timezone1, args.timezone2
        );
    }

    async getVedicCompatibility(args: CompArgs): Promise<AshtaKutaResult> {
        const wasm = await getWasmModule();
        return wasm.get_vedic_compatibility(
            args.year1, args.month1, args.day1, args.hour1, args.minute1,
            args.is_lunar1, args.is_leap_month1, args.lat1, args.lon1,
            args.year2, args.month2, args.day2, args.hour2, args.minute2,
            args.is_lunar2, args.is_leap_month2, args.lat2, args.lon2,
            args.timezone1, args.timezone2
        );
    }

    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    async getAiAudit(_args: SajuArgs): Promise<unknown> {
        console.warn("AI Audit is only available in the Tauri desktop app.");
        return null;
    }
}

export class TauriBackendClient implements BackendClient {
    async getVedicAnalysis(args: AnalysisArgs): Promise<VedicAnalysisResult> {
        return invoke("get_vedic_analysis", args as any);
    }

    async getSajuAnalysis(args: SajuArgs): Promise<SajuAnalysisResult> {
        const result = await invoke("get_saju_analysis", args as any);
        return processSajuResult(result);
    }

    async getTransitAnalysis(args: TransitArgs): Promise<TransitResult> {
        return invoke("get_transit_analysis", args as any);
    }

    async getSajuCompatibility(args: CompArgs): Promise<CompatibilityAudit> {
        return invoke("get_saju_compatibility", args as any);
    }

    async getVedicCompatibility(args: CompArgs): Promise<AshtaKutaResult> {
        return invoke("get_vedic_compatibility", args as any);
    }

    async getAiAudit(args: SajuArgs): Promise<unknown> {
        return invoke("get_ai_audit", args as any);
    }
}

// ── 환경 감지 및 클라이언트 선택 ──
declare global {
    interface Window {
        __TAURI_INTERNALS__?: unknown;
    }
}

export const isTauri = () => typeof window !== "undefined" && window.__TAURI_INTERNALS__ !== undefined;

export const backendClient: BackendClient = isTauri() ? new TauriBackendClient() : new WasmBackendClient();
