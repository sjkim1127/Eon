import * as fs from 'fs';
import init, { get_vedic_analysis } from './app/src/lib/wasm/eon_wasm.js';

async function run() {
    const wasmBuffer = fs.readFileSync('./app/src/lib/wasm/eon_wasm_bg.wasm');
    await init({ module_or_path: wasmBuffer });
    const result = await get_vedic_analysis(2004, 11, 27, 22, 0, false, false, 37.31, 126.83, "Asia/Seoul");
    console.log(JSON.stringify(result, null, 2));
}

run().catch(console.error);
