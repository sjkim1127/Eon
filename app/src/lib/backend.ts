import { invoke } from "@tauri-apps/api/core";
import type {
    SajuAnalysisRequest,
    VedicAnalysisRequest,
    TransitAnalysisRequest,
    DestinyTierRequest,
    SajuAnalysisOutput,
    VedicAnalysisOutput,
    TransitAnalysisOutput,
    TierResult,
} from "../generated/eon-api";

export interface BackendClient {
    getVedicAnalysis(request: VedicAnalysisRequest): Promise<VedicAnalysisOutput>;
    getSajuAnalysis(request: SajuAnalysisRequest): Promise<SajuAnalysisOutput>;
    getTransitAnalysis(request: TransitAnalysisRequest): Promise<TransitAnalysisOutput>;
    getDestinyTier(saju: SajuAnalysisOutput, vedic: VedicAnalysisOutput, transit: TransitAnalysisOutput | null): Promise<TierResult>;
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
    async getVedicAnalysis(request: VedicAnalysisRequest): Promise<VedicAnalysisOutput> {
        const wasm = await getWasmModule();
        return wasm.get_vedic_analysis(request) as Promise<VedicAnalysisOutput>;
    }

    async getSajuAnalysis(request: SajuAnalysisRequest): Promise<SajuAnalysisOutput> {
        const wasm = await getWasmModule();
        return wasm.get_saju_analysis(request) as Promise<SajuAnalysisOutput>;
    }

    async getTransitAnalysis(request: TransitAnalysisRequest): Promise<TransitAnalysisOutput> {
        const wasm = await getWasmModule();
        return wasm.get_transit_analysis(request) as Promise<TransitAnalysisOutput>;
    }

    async getDestinyTier(saju: SajuAnalysisOutput, vedic: VedicAnalysisOutput, transit: TransitAnalysisOutput | null): Promise<TierResult> {
        const wasm = await getWasmModule();
        const request: DestinyTierRequest = { saju, vedic, transit };
        return (wasm as any).get_destiny_tier_analysis(request) as Promise<TierResult>;
    }
}

export class TauriBackendClient implements BackendClient {
    async getVedicAnalysis(request: VedicAnalysisRequest): Promise<VedicAnalysisOutput> {
        return invoke("get_vedic_analysis", { request });
    }

    async getSajuAnalysis(request: SajuAnalysisRequest): Promise<SajuAnalysisOutput> {
        return invoke("get_saju_analysis", { request });
    }

    async getTransitAnalysis(request: TransitAnalysisRequest): Promise<TransitAnalysisOutput> {
        return invoke("get_transit_analysis", { request });
    }

    async getDestinyTier(saju: SajuAnalysisOutput, vedic: VedicAnalysisOutput, transit: TransitAnalysisOutput | null): Promise<TierResult> {
        const request: DestinyTierRequest = { saju, vedic, transit };
        return invoke("get_destiny_tier_analysis", { request });
    }
}

declare global {
    interface Window {
        __TAURI_INTERNALS__?: unknown;
    }
}

export const isTauri = () => typeof window !== "undefined" && window.__TAURI_INTERNALS__ !== undefined;

export const backendClient: BackendClient = isTauri() ? new TauriBackendClient() : new WasmBackendClient();
