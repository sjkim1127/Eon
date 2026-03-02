import { invoke } from "@tauri-apps/api/core";

// Dynamically imported WASM module
let wasmModuleCache: any = null;
const getWasmModule = async () => {
    if (wasmModuleCache) return wasmModuleCache;
    const wasm = await import("eon-wasm");
    await wasm.default(); // Initialize the WASM module
    wasmModuleCache = wasm;
    return wasm;
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
    lat: number;
    lon: number;
}): Promise<any> => {
    if (isTauri()) {
        console.log("Using Tauri Native Backend");
        return invoke("get_vedic_analysis", args);
    } else {
        console.log("Using WASM Backend");
        const wasm = await getWasmModule();
        return wasm.get_vedic_analysis(
            args.year, args.month, args.day,
            args.hour, args.minute,
            args.lat, args.lon
        );
    }
};

export const get_saju_analysis = async (args: {
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
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
        return wasm.get_saju_analysis(
            args.year, args.month, args.day,
            args.hour, args.minute,
            args.is_male,
            args.lon, args.lat,
            args.timezone
        );
    }
};
