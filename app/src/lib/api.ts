import { invoke } from "@tauri-apps/api/core";
import type { VedicAnalysisResult } from "../types";

// Dynamically imported WASM module — cache the Promise to prevent race conditions
// when multiple API calls (saju, vedic, transit) fire simultaneously via Promise.allSettled
let wasmPromiseCache: Promise<any> | null = null;
const getWasmModule = async () => {
    if (!wasmPromiseCache) {
        wasmPromiseCache = import("eon-wasm");
    }
    return wasmPromiseCache;
};

// Check if we are running inside the Tauri environment
declare global {
    interface Window {
        __TAURI_INTERNALS__?: any;
    }
}

const isTauri = () => {
    return typeof window !== "undefined" && window.__TAURI_INTERNALS__ !== undefined;
};

export const get_vedic_analysis = async (args: {
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    is_lunar: boolean;
    is_leap_month: boolean;
    lat: number;
    lon: number;
    timezone: string;
}): Promise<VedicAnalysisResult> => {
    if (isTauri()) {
        console.log("Using Tauri Native Backend");
        return invoke("get_vedic_analysis", args);
    } else {
        console.log("Using WASM Backend");
        const wasm = await getWasmModule();
        return wasm.get_vedic_analysis(
            args.year, args.month, args.day,
            args.hour, args.minute,
            args.is_lunar, args.is_leap_month,
            args.lat, args.lon,
            args.timezone
        );
    }
};

export const get_saju_analysis = async (args: {
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    is_lunar: boolean;
    is_leap_month: boolean;
    is_male: boolean;
    lon: number;
    lat: number;
    timezone: string;
}): Promise<any> => {
    if (isTauri()) {
        console.log("Using Tauri Native Backend (Saju)");
        return invoke("get_saju_analysis", args);
    } else {
        console.log("Using WASM Backend (Saju)");
        const wasm = await getWasmModule();
        const result = wasm.get_saju_analysis(
            args.year, args.month, args.day,
            args.hour, args.minute,
            args.is_lunar, args.is_leap_month,
            args.is_male,
            args.lon, args.lat,
            args.timezone
        );
        // serde_wasm_bindgen workaround: timeline_json → report.timeline
        if (result.timeline_json && result.report) {
            try {
                result.report.timeline = JSON.parse(result.timeline_json);
                console.log("[WASM] timeline parsed:", result.report.timeline?.length, "items");
            } catch (e) {
                console.warn("[WASM] timeline_json parse failed:", e);
            }
        }
        return result;
    }
};

export const get_transit_analysis = async (args: {
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    is_lunar: boolean;
    is_leap_month: boolean;
    is_male: boolean;
    lon: number;
    lat: number;
    timezone: string;
    current_year: number;
    current_month: number;
    current_day: number;
}): Promise<any> => {
    if (isTauri()) {
        console.log("Using Tauri Native Backend (Transit)");
        return invoke("get_transit_analysis", args);
    } else {
        console.log("Using WASM Backend (Transit)", args);
        const wasm = await getWasmModule();
        try {
            const result = wasm.get_transit_analysis(
                args.year, args.month, args.day,
                args.hour, args.minute,
                args.is_lunar, args.is_leap_month,
                args.is_male,
                args.lon, args.lat,
                args.timezone,
                args.current_year, args.current_month, args.current_day
            );
            console.log("Transit result:", result);
            return result;
        } catch (e) {
            console.error("[Transit WASM Error]", e);
            throw e;
        }
    }
};

export const get_saju_compatibility = async (args: {
    year1: number; month1: number; day1: number; hour1: number; minute1: number;
    is_lunar1: boolean; is_leap_month1: boolean;
    is_male1: boolean; lon1: number; lat1: number;
    year2: number; month2: number; day2: number; hour2: number; minute2: number;
    is_lunar2: boolean; is_leap_month2: boolean;
    is_male2: boolean; lat2: number;
    lon2: number;
    timezone1: string;
    timezone2: string;
}): Promise<any> => {
    if (isTauri()) {
        console.log("Using Tauri Native Backend (Saju Compatibility)");
        return invoke("get_saju_compatibility", args);
    } else {
        console.log("Using WASM Backend (Saju Compatibility)");
        const wasm = await getWasmModule();
        return wasm.get_saju_compatibility(
            args.year1, args.month1, args.day1, args.hour1, args.minute1,
            args.is_lunar1, args.is_leap_month1,
            args.is_male1, args.lon1, args.lat1,
            args.year2, args.month2, args.day2, args.hour2, args.minute2,
            args.is_lunar2, args.is_leap_month2,
            args.is_male2, args.lon2, args.lat2,
            args.timezone1, args.timezone2
        );
    }
};

export const get_vedic_compatibility = async (args: {
    year1: number; month1: number; day1: number; hour1: number; minute1: number; is_lunar1: boolean; is_leap_month1: boolean; lat1: number; lon1: number;
    year2: number; month2: number; day2: number; hour2: number; minute2: number; is_lunar2: boolean; is_leap_month2: boolean; lat2: number; lon2: number;
    timezone1: string;
    timezone2: string;
}): Promise<any> => {
    if (isTauri()) {
        return invoke("get_vedic_compatibility", args);
    } else {
        const wasm = await getWasmModule();
        return wasm.get_vedic_compatibility(
            args.year1, args.month1, args.day1, args.hour1, args.minute1, args.is_lunar1, args.is_leap_month1, args.lat1, args.lon1,
            args.year2, args.month2, args.day2, args.hour2, args.minute2, args.is_lunar2, args.is_leap_month2, args.lat2, args.lon2,
            args.timezone1, args.timezone2
        );
    }
};

export const get_ai_audit = async (args: {
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    is_lunar: boolean;
    is_leap_month: boolean;
    is_male: boolean;
    lon: number;
    lat: number;
    timezone: string;
}): Promise<any> => {
    if (isTauri()) {
        console.log("Using Tauri Native Backend (AI Audit)");
        return invoke("get_ai_audit", args);
    } else {
        // WASM 환경에서는 AI Audit 미지원 (Tauri 전용)
        console.warn("AI Audit is only available in the Tauri desktop app.");
        return null;
    }
};
