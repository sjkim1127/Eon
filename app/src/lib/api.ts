import { invoke } from "@tauri-apps/api/core";
// Dynamically imported WASM module
const getWasmModule = async () => {
    const wasm = await import("eon-wasm");
    await wasm.default(); // Initialize the WASM module
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
        // wasm expects flat arguments rather than a single object
        return wasm.get_vedic_analysis(
            args.year,
            args.month,
            args.day,
            args.hour,
            args.minute,
            args.lat,
            args.lon
        );
    }
};
