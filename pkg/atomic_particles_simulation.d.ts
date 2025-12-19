/* tslint:disable */
/* eslint-disable */

export class SimulationEngine {
  free(): void;
  [Symbol.dispose](): void;
  add_nucleus(x: number, y: number, protons: number, neutrons: number, electrons: number): void;
  add_particle(x: number, y: number, particle_type: number): void;
  get_nuclei_data(): string;
  set_nuclear_mass(mass: number): void;
  get_particle_data(index: number): string;
  set_coulomb_force(force: number): void;
  set_electron_mass(mass: number): void;
  set_nuclear_force(force: number): void;
  set_nuclear_range(range: number): void;
  set_orbital_speed(speed: number): void;
  get_particle_count(): number;
  set_orbital_radius(radius: number): void;
  set_nuclear_damping(damping: number): void;
  set_electron_damping(damping: number): void;
  set_proton_repulsion(repulsion: number): void;
  set_electron_repulsion(repulsion: number): void;
  set_max_orbital_radius(radius: number): void;
  set_min_orbital_radius(radius: number): void;
  set_nuclear_attractive(force: number): void;
  add_particle_with_velocity(x: number, y: number, particle_type: number, dx: number, dy: number): void;
  constructor(canvas_width: number, canvas_height: number);
  clear(): void;
  render(context: CanvasRenderingContext2D): void;
  update(): void;
  add_ion(x: number, y: number, protons: number, neutrons: number, electrons: number, charge: number): void;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_simulationengine_free: (a: number, b: number) => void;
  readonly simulationengine_add_ion: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly simulationengine_add_nucleus: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly simulationengine_add_particle: (a: number, b: number, c: number, d: number) => void;
  readonly simulationengine_add_particle_with_velocity: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly simulationengine_clear: (a: number) => void;
  readonly simulationengine_get_nuclei_data: (a: number) => [number, number];
  readonly simulationengine_get_particle_count: (a: number) => number;
  readonly simulationengine_get_particle_data: (a: number, b: number) => [number, number];
  readonly simulationengine_new: (a: number, b: number) => number;
  readonly simulationengine_render: (a: number, b: any) => void;
  readonly simulationengine_set_coulomb_force: (a: number, b: number) => void;
  readonly simulationengine_set_electron_damping: (a: number, b: number) => void;
  readonly simulationengine_set_electron_mass: (a: number, b: number) => void;
  readonly simulationengine_set_electron_repulsion: (a: number, b: number) => void;
  readonly simulationengine_set_max_orbital_radius: (a: number, b: number) => void;
  readonly simulationengine_set_min_orbital_radius: (a: number, b: number) => void;
  readonly simulationengine_set_nuclear_attractive: (a: number, b: number) => void;
  readonly simulationengine_set_nuclear_damping: (a: number, b: number) => void;
  readonly simulationengine_set_nuclear_force: (a: number, b: number) => void;
  readonly simulationengine_set_nuclear_mass: (a: number, b: number) => void;
  readonly simulationengine_set_nuclear_range: (a: number, b: number) => void;
  readonly simulationengine_set_orbital_radius: (a: number, b: number) => void;
  readonly simulationengine_set_orbital_speed: (a: number, b: number) => void;
  readonly simulationengine_set_proton_repulsion: (a: number, b: number) => void;
  readonly simulationengine_update: (a: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
