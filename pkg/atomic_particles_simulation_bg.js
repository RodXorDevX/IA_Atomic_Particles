let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
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

const SimulationEngineFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_simulationengine_free(ptr >>> 0, 1));

export class SimulationEngine {
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SimulationEngineFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_simulationengine_free(ptr, 0);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} protons
     * @param {number} neutrons
     * @param {number} electrons
     */
    add_nucleus(x, y, protons, neutrons, electrons) {
        wasm.simulationengine_add_nucleus(this.__wbg_ptr, x, y, protons, neutrons, electrons);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} particle_type
     */
    add_particle(x, y, particle_type) {
        wasm.simulationengine_add_particle(this.__wbg_ptr, x, y, particle_type);
    }
    /**
     * @returns {string}
     */
    get_nuclei_data() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.simulationengine_get_nuclei_data(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} mass
     */
    set_nuclear_mass(mass) {
        wasm.simulationengine_set_nuclear_mass(this.__wbg_ptr, mass);
    }
    /**
     * @param {number} index
     * @returns {string}
     */
    get_particle_data(index) {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.simulationengine_get_particle_data(this.__wbg_ptr, index);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {number} force
     */
    set_coulomb_force(force) {
        wasm.simulationengine_set_coulomb_force(this.__wbg_ptr, force);
    }
    /**
     * @param {number} mass
     */
    set_electron_mass(mass) {
        wasm.simulationengine_set_electron_mass(this.__wbg_ptr, mass);
    }
    /**
     * @param {number} force
     */
    set_nuclear_force(force) {
        wasm.simulationengine_set_nuclear_force(this.__wbg_ptr, force);
    }
    /**
     * @param {number} range
     */
    set_nuclear_range(range) {
        wasm.simulationengine_set_nuclear_range(this.__wbg_ptr, range);
    }
    /**
     * @param {number} speed
     */
    set_orbital_speed(speed) {
        wasm.simulationengine_set_orbital_speed(this.__wbg_ptr, speed);
    }
    /**
     * @returns {number}
     */
    get_particle_count() {
        const ret = wasm.simulationengine_get_particle_count(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} radius
     */
    set_orbital_radius(radius) {
        wasm.simulationengine_set_orbital_radius(this.__wbg_ptr, radius);
    }
    /**
     * @param {number} damping
     */
    set_nuclear_damping(damping) {
        wasm.simulationengine_set_nuclear_damping(this.__wbg_ptr, damping);
    }
    /**
     * @param {number} damping
     */
    set_electron_damping(damping) {
        wasm.simulationengine_set_electron_damping(this.__wbg_ptr, damping);
    }
    /**
     * @param {number} repulsion
     */
    set_proton_repulsion(repulsion) {
        wasm.simulationengine_set_proton_repulsion(this.__wbg_ptr, repulsion);
    }
    /**
     * @param {number} repulsion
     */
    set_electron_repulsion(repulsion) {
        wasm.simulationengine_set_electron_repulsion(this.__wbg_ptr, repulsion);
    }
    /**
     * @param {number} radius
     */
    set_max_orbital_radius(radius) {
        wasm.simulationengine_set_max_orbital_radius(this.__wbg_ptr, radius);
    }
    /**
     * @param {number} radius
     */
    set_min_orbital_radius(radius) {
        wasm.simulationengine_set_min_orbital_radius(this.__wbg_ptr, radius);
    }
    /**
     * @param {number} force
     */
    set_nuclear_attractive(force) {
        wasm.simulationengine_set_nuclear_attractive(this.__wbg_ptr, force);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} particle_type
     * @param {number} dx
     * @param {number} dy
     */
    add_particle_with_velocity(x, y, particle_type, dx, dy) {
        wasm.simulationengine_add_particle_with_velocity(this.__wbg_ptr, x, y, particle_type, dx, dy);
    }
    /**
     * @param {number} canvas_width
     * @param {number} canvas_height
     */
    constructor(canvas_width, canvas_height) {
        const ret = wasm.simulationengine_new(canvas_width, canvas_height);
        this.__wbg_ptr = ret >>> 0;
        SimulationEngineFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    clear() {
        wasm.simulationengine_clear(this.__wbg_ptr);
    }
    /**
     * @param {CanvasRenderingContext2D} context
     */
    render(context) {
        wasm.simulationengine_render(this.__wbg_ptr, context);
    }
    update() {
        wasm.simulationengine_update(this.__wbg_ptr);
    }
    /**
     * @param {number} x
     * @param {number} y
     * @param {number} protons
     * @param {number} neutrons
     * @param {number} electrons
     * @param {number} charge
     */
    add_ion(x, y, protons, neutrons, electrons, charge) {
        wasm.simulationengine_add_ion(this.__wbg_ptr, x, y, protons, neutrons, electrons, charge);
    }
}
if (Symbol.dispose) SimulationEngine.prototype[Symbol.dispose] = SimulationEngine.prototype.free;

export function __wbg___wbindgen_throw_dd24417ed36fc46e(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbg_arc_c46ca66b5ec2f1ac() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
    arg0.arc(arg1, arg2, arg3, arg4, arg5);
}, arguments) };

export function __wbg_beginPath_08eae248f93ea32d(arg0) {
    arg0.beginPath();
};

export function __wbg_ellipse_8fe237473fd39db1() { return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7) {
    arg0.ellipse(arg1, arg2, arg3, arg4, arg5, arg6, arg7);
}, arguments) };

export function __wbg_fillText_56566d8049e84e17() { return handleError(function (arg0, arg1, arg2, arg3, arg4) {
    arg0.fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
}, arguments) };

export function __wbg_fill_dd0f756eea36e037(arg0) {
    arg0.fill();
};

export function __wbg_new_25f239778d6112b9() {
    const ret = new Array();
    return ret;
};

export function __wbg_of_b8cd42ebb79fb759(arg0, arg1) {
    const ret = Array.of(arg0, arg1);
    return ret;
};

export function __wbg_setLineDash_cfbc3a6bfddeee62() { return handleError(function (arg0, arg1) {
    arg0.setLineDash(arg1);
}, arguments) };

export function __wbg_set_fillStyle_ea371e123273908b(arg0, arg1) {
    arg0.fillStyle = arg1;
};

export function __wbg_set_font_37c5ab71d0189314(arg0, arg1, arg2) {
    arg0.font = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_lineWidth_feda4b79a15c660b(arg0, arg1) {
    arg0.lineWidth = arg1;
};

export function __wbg_set_strokeStyle_857faae3a756ddf4(arg0, arg1) {
    arg0.strokeStyle = arg1;
};

export function __wbg_set_textAlign_5d82eb01e9d2291e(arg0, arg1, arg2) {
    arg0.textAlign = getStringFromWasm0(arg1, arg2);
};

export function __wbg_set_textBaseline_9e8ed61033c5023d(arg0, arg1, arg2) {
    arg0.textBaseline = getStringFromWasm0(arg1, arg2);
};

export function __wbg_stroke_a18b81eb49ff370e(arg0) {
    arg0.stroke();
};

export function __wbindgen_cast_2241b6af4c4b2941(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
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
