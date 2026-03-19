let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

const CLOSURE_DTORS = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(state => state.dtor(state.a, state.b));

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {

        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            state.a = a;
            real._wbg_cb_unref();
        }
    };
    real._wbg_cb_unref = () => {
        if (--state.cnt === 0) {
            state.dtor(state.a, state.b);
            state.a = 0;
            CLOSURE_DTORS.unregister(state);
        }
    };
    CLOSURE_DTORS.register(real, state, state);
    return real;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

function takeFromExternrefTable0(idx) {
    const value = wasm.__wbindgen_externrefs.get(idx);
    wasm.__externref_table_dealloc(idx);
    return value;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

let WASM_VECTOR_LEN = 0;

function wasm_bindgen__convert__closures_____invoke__h4cd75a9079ae9638(arg0, arg1, arg2) {
    wasm.wasm_bindgen__convert__closures_____invoke__h4cd75a9079ae9638(arg0, arg1, arg2);
}

function wasm_bindgen__convert__closures_____invoke__h22aa556dbecabb1f(arg0, arg1, arg2, arg3) {
    wasm.wasm_bindgen__convert__closures_____invoke__h22aa556dbecabb1f(arg0, arg1, arg2, arg3);
}

const PositionFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_position_free(ptr >>> 0, 1));

const SwissEphErrorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_swissepherror_free(ptr >>> 0, 1));

/**
 * Planetary position result
 */
export class Position {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Position.prototype);
        obj.__wbg_ptr = ptr;
        PositionFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PositionFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_position_free(ptr, 0);
    }
    /**
     * Ecliptic longitude in degrees
     * @returns {number}
     */
    get longitude() {
        const ret = wasm.__wbg_get_position_longitude(this.__wbg_ptr);
        return ret;
    }
    /**
     * Ecliptic longitude in degrees
     * @param {number} arg0
     */
    set longitude(arg0) {
        wasm.__wbg_set_position_longitude(this.__wbg_ptr, arg0);
    }
    /**
     * Ecliptic latitude in degrees
     * @returns {number}
     */
    get latitude() {
        const ret = wasm.__wbg_get_position_latitude(this.__wbg_ptr);
        return ret;
    }
    /**
     * Ecliptic latitude in degrees
     * @param {number} arg0
     */
    set latitude(arg0) {
        wasm.__wbg_set_position_latitude(this.__wbg_ptr, arg0);
    }
    /**
     * Distance (AU for planets, Earth radii for Moon)
     * @returns {number}
     */
    get distance() {
        const ret = wasm.__wbg_get_position_distance(this.__wbg_ptr);
        return ret;
    }
    /**
     * Distance (AU for planets, Earth radii for Moon)
     * @param {number} arg0
     */
    set distance(arg0) {
        wasm.__wbg_set_position_distance(this.__wbg_ptr, arg0);
    }
    /**
     * Longitude speed (degrees/day)
     * @returns {number}
     */
    get longitude_speed() {
        const ret = wasm.__wbg_get_position_longitude_speed(this.__wbg_ptr);
        return ret;
    }
    /**
     * Longitude speed (degrees/day)
     * @param {number} arg0
     */
    set longitude_speed(arg0) {
        wasm.__wbg_set_position_longitude_speed(this.__wbg_ptr, arg0);
    }
    /**
     * Latitude speed (degrees/day)
     * @returns {number}
     */
    get latitude_speed() {
        const ret = wasm.__wbg_get_position_latitude_speed(this.__wbg_ptr);
        return ret;
    }
    /**
     * Latitude speed (degrees/day)
     * @param {number} arg0
     */
    set latitude_speed(arg0) {
        wasm.__wbg_set_position_latitude_speed(this.__wbg_ptr, arg0);
    }
    /**
     * Distance speed (AU/day)
     * @returns {number}
     */
    get distance_speed() {
        const ret = wasm.__wbg_get_position_distance_speed(this.__wbg_ptr);
        return ret;
    }
    /**
     * Distance speed (AU/day)
     * @param {number} arg0
     */
    set distance_speed(arg0) {
        wasm.__wbg_set_position_distance_speed(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) Position.prototype[Symbol.dispose] = Position.prototype.free;

/**
 * Error returned by Swiss Ephemeris calculations
 */
export class SwissEphError {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(SwissEphError.prototype);
        obj.__wbg_ptr = ptr;
        SwissEphErrorFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SwissEphErrorFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_swissepherror_free(ptr, 0);
    }
    /**
     * Error message from the library
     * @returns {string}
     */
    get message() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_swissepherror_message(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Error message from the library
     * @param {string} arg0
     */
    set message(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_swissepherror_message(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * Return code
     * @returns {number}
     */
    get code() {
        const ret = wasm.__wbg_get_swissepherror_code(this.__wbg_ptr);
        return ret;
    }
    /**
     * Return code
     * @param {number} arg0
     */
    set code(arg0) {
        wasm.__wbg_set_swissepherror_code(this.__wbg_ptr, arg0);
    }
}
if (Symbol.dispose) SwissEphError.prototype[Symbol.dispose] = SwissEphError.prototype.free;

/**
 * Calculate planetary position using UT (Universal Time)
 * @param {number} jd_ut
 * @param {number} planet
 * @param {number} flags
 * @returns {Position}
 */
export function calc_ut(jd_ut, planet, flags) {
    const ret = wasm.calc_ut(jd_ut, planet, flags);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return Position.__wrap(ret[0]);
}

/**
 * 사주(四柱) 분석 — WASM에서 호출 가능
 *
 * 생년월일시 + 성별 + 좌표 + 타임존을 받아 사주 분석 결과를 반환합니다.
 * BirthInfo를 사용하여 DST(서머타임) + 경도 기반 진태양시 보정을 자동 적용합니다.
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {number} hour
 * @param {number} minute
 * @param {boolean} is_lunar
 * @param {boolean} is_leap_month
 * @param {boolean} is_male
 * @param {number} lon
 * @param {number} lat
 * @param {string} timezone
 * @returns {any}
 */
export function get_saju_analysis(year, month, day, hour, minute, is_lunar, is_leap_month, is_male, lon, lat, timezone) {
    const ptr0 = passStringToWasm0(timezone, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_saju_analysis(year, month, day, hour, minute, is_lunar, is_leap_month, is_male, lon, lat, ptr0, len0);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * 사주 궁합 분석 - WASM에서 호출 가능
 * @param {number} year1
 * @param {number} month1
 * @param {number} day1
 * @param {number} hour1
 * @param {number} minute1
 * @param {boolean} is_lunar1
 * @param {boolean} is_leap_month1
 * @param {boolean} is_male1
 * @param {number} lon1
 * @param {number} lat1
 * @param {number} year2
 * @param {number} month2
 * @param {number} day2
 * @param {number} hour2
 * @param {number} minute2
 * @param {boolean} is_lunar2
 * @param {boolean} is_leap_month2
 * @param {boolean} is_male2
 * @param {number} lon2
 * @param {number} lat2
 * @param {string} timezone1
 * @param {string} timezone2
 * @returns {any}
 */
export function get_saju_compatibility(year1, month1, day1, hour1, minute1, is_lunar1, is_leap_month1, is_male1, lon1, lat1, year2, month2, day2, hour2, minute2, is_lunar2, is_leap_month2, is_male2, lon2, lat2, timezone1, timezone2) {
    const ptr0 = passStringToWasm0(timezone1, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(timezone2, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.get_saju_compatibility(year1, month1, day1, hour1, minute1, is_lunar1, is_leap_month1, is_male1, lon1, lat1, year2, month2, day2, hour2, minute2, is_lunar2, is_leap_month2, is_male2, lon2, lat2, ptr0, len0, ptr1, len1);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * 현재 운세(세운/월운/일운) 분석 — WASM에서 호출 가능
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {number} hour
 * @param {number} minute
 * @param {boolean} is_lunar
 * @param {boolean} is_leap_month
 * @param {boolean} is_male
 * @param {number} lon
 * @param {number} lat
 * @param {string} timezone
 * @param {number} current_year
 * @param {number} current_month
 * @param {number} current_day
 * @returns {any}
 */
export function get_transit_analysis(year, month, day, hour, minute, is_lunar, is_leap_month, is_male, lon, lat, timezone, current_year, current_month, current_day) {
    const ptr0 = passStringToWasm0(timezone, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_transit_analysis(year, month, day, hour, minute, is_lunar, is_leap_month, is_male, lon, lat, ptr0, len0, current_year, current_month, current_day);
    if (ret[2]) {
        throw takeFromExternrefTable0(ret[1]);
    }
    return takeFromExternrefTable0(ret[0]);
}

/**
 * @param {number} year
 * @param {number} month
 * @param {number} day
 * @param {number} hour
 * @param {number} minute
 * @param {boolean} is_lunar
 * @param {boolean} is_leap_month
 * @param {number} lat
 * @param {number} lon
 * @param {string} timezone
 * @returns {Promise<any>}
 */
export function get_vedic_analysis(year, month, day, hour, minute, is_lunar, is_leap_month, lat, lon, timezone) {
    const ptr0 = passStringToWasm0(timezone, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_vedic_analysis(year, month, day, hour, minute, is_lunar, is_leap_month, lat, lon, ptr0, len0);
    return ret;
}

/**
 * 베딕 궁합 분석 (Ashta Kuta) - WASM에서 호출 가능
 * @param {number} year1
 * @param {number} month1
 * @param {number} day1
 * @param {number} hour1
 * @param {number} minute1
 * @param {boolean} is_lunar1
 * @param {boolean} is_leap_month1
 * @param {number} lat1
 * @param {number} lon1
 * @param {number} year2
 * @param {number} month2
 * @param {number} day2
 * @param {number} hour2
 * @param {number} minute2
 * @param {boolean} is_lunar2
 * @param {boolean} is_leap_month2
 * @param {number} lat2
 * @param {number} lon2
 * @param {string} timezone1
 * @param {string} timezone2
 * @returns {Promise<any>}
 */
export function get_vedic_compatibility(year1, month1, day1, hour1, minute1, is_lunar1, is_leap_month1, lat1, lon1, year2, month2, day2, hour2, minute2, is_lunar2, is_leap_month2, lat2, lon2, timezone1, timezone2) {
    const ptr0 = passStringToWasm0(timezone1, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(timezone2, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.get_vedic_compatibility(year1, month1, day1, hour1, minute1, is_lunar1, is_leap_month1, lat1, lon1, year2, month2, day2, hour2, minute2, is_lunar2, is_leap_month2, lat2, lon2, ptr0, len0, ptr1, len1);
    return ret;
}

/**
 * @param {string} name
 * @returns {string}
 */
export function greet(name) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.greet(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * WASM 패닉 메시지를 브라우저 콘솔에 표시
 */
export function init_panic_hook() {
    wasm.init_panic_hook();
}

/**
 * Set the ephemeris path
 * @param {string} path
 */
export function set_ephe_path(path) {
    const ptr0 = passStringToWasm0(path, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.set_ephe_path(ptr0, len0);
}

/**
 * Get Swiss Ephemeris version
 * @returns {string}
 */
export function version() {
    let deferred1_0;
    let deferred1_1;
    try {
        const ret = wasm.version();
        deferred1_0 = ret[0];
        deferred1_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
    }
}

export function __wbg_Error_52673b7de5a0ca89(arg0, arg1) {
    const ret = Error(getStringFromWasm0(arg0, arg1));
    return ret;
};

export function __wbg_String_8f0eb39a4a4c2f66(arg0, arg1) {
    const ret = String(arg1);
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg___wbindgen_is_function_8d400b8b1af978cd(arg0) {
    const ret = typeof(arg0) === 'function';
    return ret;
};

export function __wbg___wbindgen_is_undefined_f6b95eab589e0269(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

export function __wbg___wbindgen_throw_dd24417ed36fc46e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbg__wbg_cb_unref_87dfb5aaa0cbcea7(arg0) {
    arg0._wbg_cb_unref();
};

export function __wbg_call_3020136f7a2d6e44() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.call(arg1, arg2);
    return ret;
}, arguments) };

export function __wbg_call_abb4ff46ce38be40() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

export function __wbg_error_7534b8e9a36f1ab4(arg0, arg1) {
    let deferred0_0;
    let deferred0_1;
    try {
        deferred0_0 = arg0;
        deferred0_1 = arg1;
        console.error(getStringFromWasm0(arg0, arg1));
    } finally {
        wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
    }
};

export function __wbg_getTime_ad1e9878a735af08(arg0) {
    const ret = arg0.getTime();
    return ret;
};

export function __wbg_new_0_23cedd11d9b40c9d() {
    const ret = new Date();
    return ret;
};

export function __wbg_new_1ba21ce319a06297() {
    const ret = new Object();
    return ret;
};

export function __wbg_new_25f239778d6112b9() {
    const ret = new Array();
    return ret;
};

export function __wbg_new_8a6f238a6ece86ea() {
    const ret = new Error();
    return ret;
};

export function __wbg_new_ff12d2b041fb48f1(arg0, arg1) {
    try {
        var state0 = {a: arg0, b: arg1};
        var cb0 = (arg0, arg1) => {
            const a = state0.a;
            state0.a = 0;
            try {
                return wasm_bindgen__convert__closures_____invoke__h22aa556dbecabb1f(a, state0.b, arg0, arg1);
            } finally {
                state0.a = a;
            }
        };
        const ret = new Promise(cb0);
        return ret;
    } finally {
        state0.a = state0.b = 0;
    }
};

export function __wbg_new_no_args_cb138f77cf6151ee(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
};

export function __wbg_queueMicrotask_9b549dfce8865860(arg0) {
    const ret = arg0.queueMicrotask;
    return ret;
};

export function __wbg_queueMicrotask_fca69f5bfad613a5(arg0) {
    queueMicrotask(arg0);
};

export function __wbg_resolve_fd5bfbaa4ce36e1e(arg0) {
    const ret = Promise.resolve(arg0);
    return ret;
};

export function __wbg_set_3f1d0b984ed272ed(arg0, arg1, arg2) {
    arg0[arg1] = arg2;
};

export function __wbg_set_7df433eea03a5c14(arg0, arg1, arg2) {
    arg0[arg1 >>> 0] = arg2;
};

export function __wbg_stack_0ed75d68575b0f3c(arg0, arg1) {
    const ret = arg1.stack;
    const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_static_accessor_GLOBAL_769e6b65d6557335() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_GLOBAL_THIS_60cf02db4de8e1c1() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_SELF_08f5a74c69739274() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_WINDOW_a8924b26aa92d024() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_swissepherror_new(arg0) {
    const ret = SwissEphError.__wrap(arg0);
    return ret;
};

export function __wbg_then_4f95312d68691235(arg0, arg1) {
    const ret = arg0.then(arg1);
    return ret;
};

export function __wbindgen_cast_2241b6af4c4b2941(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_cast_b3587965ddad1edc(arg0, arg1) {
    // Cast intrinsic for `Closure(Closure { dtor_idx: 25, function: Function { arguments: [Externref], shim_idx: 26, ret: Unit, inner_ret: Some(Unit) }, mutable: true }) -> Externref`.
    const ret = makeMutClosure(arg0, arg1, wasm.wasm_bindgen__closure__destroy__h5e9e2a75c5727948, wasm_bindgen__convert__closures_____invoke__h4cd75a9079ae9638);
    return ret;
};

export function __wbindgen_cast_d6cd19b81560fd6e(arg0) {
    // Cast intrinsic for `F64 -> Externref`.
    const ret = arg0;
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
};
