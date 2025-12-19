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
