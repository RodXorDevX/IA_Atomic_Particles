use crate::particle::{Particle, ParticleType};
use crate::physics::{Physics, PhysicsParams};
use web_sys::CanvasRenderingContext2d;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct Nucleus {
    pub protons: Vec<usize>,
    pub neutrons: Vec<usize>,
    pub electrons: Vec<usize>,
}

pub struct Simulation {
    pub particles: Vec<Particle>,
    pub canvas_width: f64,
    pub canvas_height: f64,
    pub params: PhysicsParams,
    pub time_step: f64,  // Para movimiento continuo de electrones
}

impl Simulation {
    pub fn new(canvas_width: f64, canvas_height: f64) -> Self {
        Simulation {
            particles: Vec::new(),
            canvas_width,
            canvas_height,
            params: PhysicsParams::default(),
            time_step: 0.0,
        }
    }

    pub fn add_particle(&mut self, x: f64, y: f64, ptype: ParticleType) {
        let mass = match ptype {
            ParticleType::Proton | ParticleType::Neutron => self.params.nuclear_mass,
            ParticleType::Electron => self.params.electron_mass,
        };
        self.particles.push(Particle::new(x, y, ptype, mass));
    }

    pub fn add_particle_with_velocity(&mut self, x: f64, y: f64, ptype: ParticleType, dx: f64, dy: f64) {
        let mass = match ptype {
            ParticleType::Proton | ParticleType::Neutron => self.params.nuclear_mass,
            ParticleType::Electron => self.params.electron_mass,
        };
        let mut particle = Particle::new(x, y, ptype, mass);
        particle.dx = dx;
        particle.dy = dy;
        self.particles.push(particle);
    }

    pub fn add_nucleus(
        &mut self,
        x: f64,
        y: f64,
        protons: usize,
        neutrons: usize,
        electrons: usize,
    ) {
        self.add_nucleus_with_charge(x, y, protons, neutrons, electrons, 0)
    }

    pub fn add_nucleus_with_charge(
        &mut self,
        x: f64,
        y: f64,
        protons: usize,
        neutrons: usize,
        electrons: usize,
        charge: i32, // Carga iónica: 0=neutro, >0=catión (+), <0=anión (-)
    ) {
        let radius = 40.0;

        // Guardar índice donde comenzarán los protones
        let proton_start_index = self.particles.len();

        // Añadir protones
        for i in 0..protons {
            let angle = (i as f64 * 2.0 * std::f64::consts::PI) / protons as f64;
            let px = x + angle.cos() * radius;
            let py = y + angle.sin() * radius;
            self.add_particle(px, py, ParticleType::Proton);
        }

        // Añadir neutrones
        for i in 0..neutrons {
            let angle = (i as f64 * 2.0 * std::f64::consts::PI) / neutrons.max(1) as f64;
            let px = x + angle.cos() * (radius * 1.2);
            let py = y + angle.sin() * (radius * 1.2);
            self.add_particle(px, py, ParticleType::Neutron);
        }

        // Añadir electrones con configuración orbital cuántica correcta desde el inicio
        for i in 0..electrons {
            // Determinar información orbital para este electrón
            if let Some((subnivel, principal_n, orbital_idx, pos_in_orbital)) = self.get_orbital_info(i) {
                let (orbital_radius, _orbital_speed, angle_offset) =
                    self.get_orbital_parameters(subnivel, principal_n, orbital_idx);

                // Calcular ángulo base para este orbital específico
                let orbital_angle_offset = self.get_orbital_angle(subnivel, orbital_idx);
                let total_angle = angle_offset + orbital_angle_offset;

                // Posicionar electrón en su orbital específico
                let px = x + orbital_radius * total_angle.cos();
                let py = y + orbital_radius * total_angle.sin();

                let mut electron = Particle::new(px, py, ParticleType::Electron, self.params.electron_mass);

                // Asignar información orbital completa desde el inicio
                electron.orbital_position = Some((subnivel, principal_n, orbital_idx, pos_in_orbital));

                // Asociar electrón con el primer protón del núcleo
                if protons > 0 {
                    electron.orbiting_around = Some(proton_start_index);
                }

                self.particles.push(electron);
            }
        }
    }

    pub fn clear(&mut self) {
        self.particles.clear();
    }

    // Estructura orbital cuántica con subniveles y orbitales individuales
    fn get_orbital_info(&self, electron_index: usize) -> Option<(char, usize, usize, usize)> {
        // Retorna: (subnivel_tipo, nivel_principal, orbital_idx, posicion_en_orbital)

        let mut current_electron = 0;

        // 1s: 1 orbital, máximo 2 electrones
        if electron_index < 2 {
            return Some(('s', 1, 0, electron_index));
        }
        current_electron += 2;

        // 2s: 1 orbital, máximo 2 electrones
        if electron_index < current_electron + 2 {
            return Some(('s', 2, 0, electron_index - current_electron));
        }
        current_electron += 2;

        // 2p: 3 orbitales, máximo 6 electrones
        if electron_index < current_electron + 6 {
            let orbital_idx = (electron_index - current_electron) / 2; // 2 electrones por orbital
            let pos_in_orbital = (electron_index - current_electron) % 2;
            return Some(('p', 2, orbital_idx, pos_in_orbital));
        }
        current_electron += 6;

        // 3s: 1 orbital, máximo 2 electrones
        if electron_index < current_electron + 2 {
            return Some(('s', 3, 0, electron_index - current_electron));
        }
        current_electron += 2;

        // 3p: 3 orbitales, máximo 6 electrones
        if electron_index < current_electron + 6 {
            let orbital_idx = (electron_index - current_electron) / 2; // 2 electrones por orbital
            let pos_in_orbital = (electron_index - current_electron) % 2;
            return Some(('p', 3, orbital_idx, pos_in_orbital));
        }
        current_electron += 6;

        // 4s: 1 orbital, máximo 2 electrones
        if electron_index < current_electron + 2 {
            return Some(('s', 4, 0, electron_index - current_electron));
        }
        current_electron += 2;

        // 3d: 5 orbitales, máximo 10 electrones
        if electron_index < current_electron + 10 {
            let orbital_idx = (electron_index - current_electron) / 2; // 2 electrones por orbital
            let pos_in_orbital = (electron_index - current_electron) % 2;
            return Some(('d', 3, orbital_idx, pos_in_orbital));
        }
        current_electron += 10;

        // 4p: 3 orbitales, máximo 6 electrones
        if electron_index < current_electron + 6 {
            let orbital_idx = (electron_index - current_electron) / 2; // 2 electrones por orbital
            let pos_in_orbital = (electron_index - current_electron) % 2;
            return Some(('p', 4, orbital_idx, pos_in_orbital));
        }
        current_electron += 6;

        // 5s: 1 orbital, máximo 2 electrones
        if electron_index < current_electron + 2 {
            return Some(('s', 5, 0, electron_index - current_electron));
        }
        current_electron += 2;

        // 4d: 5 orbitales, máximo 10 electrones
        if electron_index < current_electron + 10 {
            let orbital_idx = (electron_index - current_electron) / 2; // 2 electrones por orbital
            let pos_in_orbital = (electron_index - current_electron) % 2;
            return Some(('d', 4, orbital_idx, pos_in_orbital));
        }
        current_electron += 10;

        // 5p: 3 orbitales, máximo 6 electrones
        if electron_index < current_electron + 6 {
            let orbital_idx = (electron_index - current_electron) / 2; // 2 electrones por orbital
            let pos_in_orbital = (electron_index - current_electron) % 2;
            return Some(('p', 5, orbital_idx, pos_in_orbital));
        }
        current_electron += 6;

        // 6s: 1 orbital, máximo 2 electrones
        if electron_index < current_electron + 2 {
            return Some(('s', 6, 0, electron_index - current_electron));
        }
        current_electron += 2;

        // 4f: 7 orbitales, máximo 14 electrones
        if electron_index < current_electron + 14 {
            let orbital_idx = (electron_index - current_electron) / 2; // 2 electrones por orbital
            let pos_in_orbital = (electron_index - current_electron) % 2;
            return Some(('f', 4, orbital_idx, pos_in_orbital));
        }
        current_electron += 14;

        // Continuar con más niveles si es necesario...
        None // Límite de electrones soportados (hasta 4f)
    }

    // Obtener el número de orbitales por subnivel
    fn get_orbitals_count(&self, subnivel_tipo: char) -> usize {
        match subnivel_tipo {
            's' => 1,  // 1 orbital s
            'p' => 3,  // 3 orbitales p
            'd' => 5,  // 5 orbitales d
            'f' => 7,  // 7 orbitales f
            _ => 1,
        }
    }

    // Obtener el máximo de electrones por subnivel
    fn get_max_electrons_sublevel(&self, subnivel_tipo: char) -> usize {
        match subnivel_tipo {
            's' => 2,   // 2 electrones
            'p' => 6,   // 6 electrones
            'd' => 10,  // 10 electrones
            'f' => 14,  // 14 electrones
            _ => 2,
        }
    }

    // Obtener ángulo para orbitales individuales dentro de un subnivel
    fn get_orbital_angle(&self, subnivel_tipo: char, orbital_idx: usize) -> f64 {
        let orbitals_count = self.get_orbitals_count(subnivel_tipo);
        match subnivel_tipo {
            's' => 0.0, // Solo hay un orbital s, sin orientación específica
            'p' => {
                // 3 orbitales p: px (eje x), py (eje y), pz (eje z)
                match orbital_idx {
                    0 => 0.0,           // px (horizontal)
                    1 => std::f64::consts::PI / 2.0,  // py (vertical)
                    2 => std::f64::consts::PI,       // pz (horizontal opuesto)
                    _ => 0.0,
                }
            },
            'd' => {
                // 5 orbitales d con diferentes orientaciones
                (orbital_idx as f64 * 2.0 * std::f64::consts::PI) / orbitals_count as f64
            },
            'f' => {
                // 7 orbitales f con diferentes orientaciones
                (orbital_idx as f64 * 2.0 * std::f64::consts::PI) / orbitals_count as f64
            },
            _ => 0.0,
        }
    }

    // Obtener radio y velocidad según tipo y nivel orbital
    fn get_orbital_parameters(&self, orbital_type: char, principal_n: usize, orbital_idx: usize) -> (f64, f64, f64) {
        let base_radius = match principal_n {
            1 => 50.0,  // 1s
            2 => match orbital_type {
                's' => 75.0,   // 2s
                'p' => 100.0,  // 2p
                _ => 85.0,
            },
            3 => match orbital_type {
                's' => 125.0,  // 3s
                'p' => 150.0,  // 3p
                'd' => 175.0,  // 3d
                _ => 140.0,
            },
            4 => match orbital_type {
                's' => 200.0,  // 4s
                'p' => 225.0,  // 4p
                'd' => 250.0,  // 4d
                'f' => 275.0,  // 4f
                _ => 215.0,
            },
            _ => 100.0 + (principal_n as f64 * 25.0),
        };

        let base_speed = match orbital_type {
            's' => 4.0,  // Electrones s son más rápidos (esféricos)
            'p' => 3.0,  // Electrones p son intermedios
            'd' => 2.5,  // Electrones d son más lentos
            'f' => 2.0,  // Electrones f son los más lentos
            _ => 3.0,
        };

        // Ángulo base para movimiento orbital (usamos el índice del orbital)
        let angle_offset = match orbital_type {
            's' => 0.0,  // Esférico, cualquier ángulo
            'p' => (orbital_idx as f64) * 2.0 * std::f64::consts::PI / 3.0,  // 3 orientaciones
            'd' => (orbital_idx as f64) * 2.0 * std::f64::consts::PI / 5.0,  // 5 orientaciones
            'f' => (orbital_idx as f64) * 2.0 * std::f64::consts::PI / 7.0,  // 7 orientaciones
            _ => 0.0,
        };

        (base_radius, base_speed, angle_offset)
    }

    pub fn update(&mut self) {
        // Incrementar tiempo continuo para movimiento orbital
        self.time_step += 0.05;

        self.calculate_forces();

        // Detectar y manejar fusiones antes de actualizar posiciones
        self.handle_fusions_and_orbits();

        for particle in &mut self.particles {
            particle.update(
                self.canvas_width,
                self.canvas_height,
                if particle.ptype == ParticleType::Electron {
                    self.params.electron_damping
                } else {
                    self.params.nuclear_damping
                },
            );
        }
    }

    fn handle_fusions_and_orbits(&mut self) {
        let particles_count = self.particles.len();
        let mut to_remove: std::collections::HashSet<usize> = std::collections::HashSet::new();

        // PASO 1: Detectar PN y EN órbitas (sin fusión)
        for i in 0..particles_count {
            if to_remove.contains(&i) {
                continue;
            }

            let particle_i = &self.particles[i];
            let pi_type = particle_i.ptype;
            let pi_x = particle_i.x;
            let pi_y = particle_i.y;

            for j in i + 1..particles_count {
                if to_remove.contains(&j) {
                    continue;
                }

                let particle_j = &self.particles[j];
                let pj_type = particle_j.ptype;
                let pj_x = particle_j.x;
                let pj_y = particle_j.y;

                let dx = pj_x - pi_x;
                let dy = pj_y - pi_y;
                let distance = (dx * dx + dy * dy).sqrt();

                // ÓRBITA P-N: Protón + Neutrón orbitan entre sí (SIN fusión)
                if (pi_type == ParticleType::Proton && pj_type == ParticleType::Neutron)
                    || (pi_type == ParticleType::Neutron && pj_type == ParticleType::Proton)
                {
                    if distance < 60.0 && distance > 15.0 {
                        let (heavier_idx, lighter_idx) = if pi_type == ParticleType::Proton {
                            (i, j)
                        } else {
                            (j, i)
                        };
                        self.particles[lighter_idx].orbiting_around = Some(heavier_idx);
                    }
                }

                // ÓRBITA P-E: Electrón + Protón → Orbital atómico con configuración spdf
                if (pi_type == ParticleType::Electron && pj_type == ParticleType::Proton)
                    || (pi_type == ParticleType::Proton && pj_type == ParticleType::Electron)
                {
                    let (electron_idx, proton_idx) = if pi_type == ParticleType::Electron {
                        (i, j)
                    } else {
                        (j, i)
                    };

                    // Solo capturar si el electrón no está ya orbitando y tiene orbital_position asignado
                    if self.particles[electron_idx].orbiting_around.is_none() &&
                       self.particles[electron_idx].orbital_position.is_some() &&
                       distance < 80.0 && distance > 10.0 {

                        // Si el electrón ya tiene orbital_position (creado desde add_nucleus), solo asociarlo
                        self.particles[electron_idx].orbiting_around = Some(proton_idx);

                    } else if self.particles[electron_idx].orbiting_around.is_none() &&
                              self.particles[electron_idx].orbital_position.is_none() &&
                              distance < 80.0 && distance > 10.0 {

                        // Electrón libre: asignar nuevo orbital
                        let electron_count = self.particles.iter()
                            .filter(|p| p.orbiting_around == Some(proton_idx) && p.ptype == ParticleType::Electron)
                            .count();

                        if let Some((subnivel, principal_n, orbital_idx, pos_in_orbital)) = self.get_orbital_info(electron_count) {
                            self.particles[electron_idx].orbiting_around = Some(proton_idx);
                            self.particles[electron_idx].orbital_position = Some((subnivel, principal_n, orbital_idx, pos_in_orbital));
                        }
                    }
                }
            }
        }

        // PASO 2: Aplicar posiciones orbitales
        self.apply_orbital_mechanics();

        // Remover partículas si es necesario (nada por ahora, ya no hay fusión P-N)
        let mut indices: Vec<_> = to_remove.into_iter().collect();
        indices.sort_by(|a, b| b.cmp(a));
        for idx in indices {
            if idx < self.particles.len() {
                self.particles.remove(idx);
            }
        }
    }

    fn apply_orbital_mechanics(&mut self) {
        let particles_count = self.particles.len();
        let mut orbital_updates: Vec<(usize, f64, f64, f64, f64)> = Vec::new();

        for i in 0..particles_count {
            let particle = &self.particles[i];

            // Aplicar órbita P-N (neutrón orbita protón)
            if particle.ptype == ParticleType::Neutron {
                if let Some(proton_idx) = particle.orbiting_around {
                    if proton_idx < particles_count {
                        let proton = &self.particles[proton_idx];
                        let px = proton.x;
                        let py = proton.y;
                        let pdx = proton.dx;
                        let pdy = proton.dy;

                        let dx = px - particle.x;
                        let dy = py - particle.y;
                        let distance = (dx * dx + dy * dy).sqrt().max(1.0);

                        // Órbita a 45 píxeles
                        let orbital_radius = 45.0;
                        let angle = dy.atan2(dx);
                        let orbital_speed = 2.5;

                        let new_x = px + orbital_radius * angle.cos();
                        let new_y = py + orbital_radius * angle.sin();
                        let new_dx = pdx - orbital_speed * angle.sin();
                        let new_dy = pdy + orbital_speed * angle.cos();

                        orbital_updates.push((i, new_x, new_y, new_dx, new_dy));
                    }
                }
            }

            // Aplicar movimiento orbital realista dentro de formas características
            // Solo para electrones que están en órbita y tienen asignada información orbital completa
            if particle.ptype == ParticleType::Electron {
                if particle.orbiting_around.is_some() && particle.orbital_position.is_some() {
                    if let Some((subnivel, principal_n, orbital_idx, pos_in_orbital)) = particle.orbital_position {
                        if let Some(proton_idx) = particle.orbiting_around {
                            if proton_idx < particles_count {
                                let proton = &self.particles[proton_idx];
                                let px = proton.x;
                                let py = proton.y;

                                // Movimiento orbital basado en el tiempo continuo y el tipo de subnivel
                                let (new_x, new_y, new_dx, new_dy) = self.calculate_orbital_motion(
                                    px, py, subnivel, principal_n, orbital_idx, pos_in_orbital, self.time_step
                                );

                                orbital_updates.push((i, new_x, new_y, new_dx, new_dy));
                            }
                        }
                    }
                }
            }
        }

        // Aplicar actualizaciones de posición
        for (idx, x, y, dx, dy) in orbital_updates {
            if idx < self.particles.len() {
                self.particles[idx].x = x;
                self.particles[idx].y = y;
                self.particles[idx].dx = dx;
                self.particles[idx].dy = dy;
            }
        }
    }

    fn calculate_forces(&mut self) {
        let particles_count = self.particles.len();
        let mut forces: Vec<(f64, f64)> = vec![(0.0, 0.0); particles_count];

        for i in 0..particles_count {
            for j in i + 1..particles_count {
                let particle_i = &self.particles[i];
                let particle_j = &self.particles[j];

                let dx = particle_j.x - particle_i.x;
                let dy = particle_j.y - particle_i.y;
                let distance = (dx * dx + dy * dy).sqrt().max(1.0);

                let mut force_magnitude = 0.0;

                // Fuerzas nucleares entre protones y neutrones
                if (particle_i.ptype == ParticleType::Proton
                    || particle_i.ptype == ParticleType::Neutron)
                    && (particle_j.ptype == ParticleType::Proton
                        || particle_j.ptype == ParticleType::Neutron)
                {
                    force_magnitude +=
                        Physics::calculate_nuclear_force(distance, &self.params);

                    // Repulsión entre protones
                    if particle_i.ptype == ParticleType::Proton
                        && particle_j.ptype == ParticleType::Proton
                    {
                        force_magnitude -= self.params.proton_repulsion / (distance + 0.1);
                    }
                }

                // Fuerzas de Coulomb (electrostáticas)
                if particle_i.charge != 0.0 && particle_j.charge != 0.0 {
                    let coulomb =
                        Physics::calculate_coulomb_force(distance, particle_i.charge, particle_j.charge, &self.params);
                    force_magnitude += coulomb;
                }

                // Repulsión entre electrones
                if particle_i.ptype == ParticleType::Electron
                    && particle_j.ptype == ParticleType::Electron
                {
                    force_magnitude -= self.params.electron_repulsion / (distance + 0.1);
                }

                let force_x = (force_magnitude * dx) / distance;
                let force_y = (force_magnitude * dy) / distance;

                forces[i].0 += force_x;
                forces[i].1 += force_y;
                forces[j].0 -= force_x;
                forces[j].1 -= force_y;
            }
        }

        for i in 0..particles_count {
            Physics::apply_force(&mut self.particles[i], forces[i].0, forces[i].1);
        }
    }

    pub fn detect_nuclei(&self) -> Vec<Nucleus> {
        let mut nuclei = Vec::new();
        let mut used_particles = std::collections::HashSet::new();

        // Detectar protones agrupados
        for (i, particle) in self.particles.iter().enumerate() {
            if particle.ptype != ParticleType::Proton || used_particles.contains(&i) {
                continue;
            }

            let mut nucleus = Nucleus {
                protons: vec![i],
                neutrons: Vec::new(),
                electrons: Vec::new(),
            };
            used_particles.insert(i);

            // Buscar partículas cercanas
            for (j, other) in self.particles.iter().enumerate() {
                if used_particles.contains(&j) || i == j {
                    continue;
                }

                let distance = particle.distance_to(other);
                if distance < 100.0 {
                    match other.ptype {
                        ParticleType::Proton => {
                            nucleus.protons.push(j);
                            used_particles.insert(j);
                        }
                        ParticleType::Neutron => {
                            nucleus.neutrons.push(j);
                            used_particles.insert(j);
                        }
                        ParticleType::Electron => {
                            nucleus.electrons.push(j);
                        }
                    }
                }
            }

            if nucleus.protons.len() > 0 {
                nuclei.push(nucleus);
            }
        }

        nuclei
    }

    pub fn render(&self, context: &CanvasRenderingContext2d) {

        // Detectar núcleos y crear un conjunto de índices de partículas en núcleos
        let nuclei = self.detect_nuclei();
        let mut nucleus_particle_indices = std::collections::HashSet::new();

        for nucleus in &nuclei {
            for &proton_idx in &nucleus.protons {
                nucleus_particle_indices.insert(proton_idx);
            }
            for &neutron_idx in &nucleus.neutrons {
                nucleus_particle_indices.insert(neutron_idx);
            }
        }

        // Dibujar núcleos detectados y orbitales cuánticos SOLO si hay electrones en ese nivel
        for nucleus in &nuclei {
            if let Some(proton_idx) = nucleus.protons.get(0) {
                if let Some(_proton) = self.particles.get(*proton_idx) {
                    // Calcular centro del núcleo
                    let mut center_x = 0.0;
                    let mut center_y = 0.0;
                    let proton_count = nucleus.protons.len();

                    for &p_idx in &nucleus.protons {
                        if let Some(p) = self.particles.get(p_idx) {
                            center_x += p.x;
                            center_y += p.y;
                        }
                    }

                    if proton_count > 0 {
                        center_x /= proton_count as f64;
                        center_y /= proton_count as f64;

                        // Contar electrones por orbital según el nuevo sistema spdf
                        let mut e_1s = 0;
                        let mut e_2s = 0;
                        let mut e_2p = 0;
                        let mut e_3s = 0;
                        let mut e_3p = 0;
                        let mut e_4s = 0;
                        let mut e_3d = 0;
                        let mut e_4f = 0;

                        for &e_idx in &nucleus.electrons {
                            if let Some(e) = self.particles.get(e_idx) {
                                if let Some((subnivel, principal_n, _orbital_idx, _pos_in_orbital)) = e.orbital_position {
                                    match (principal_n, subnivel) {
                                        (1, 's') => e_1s += 1,
                                        (2, 's') => e_2s += 1,
                                        (2, 'p') => e_2p += 1,
                                        (3, 's') => e_3s += 1,
                                        (3, 'p') => e_3p += 1,
                                        (4, 's') => e_4s += 1,
                                        (3, 'd') => e_3d += 1,
                                        (4, 'f') => e_4f += 1,
                                        _ => {}
                                    }
                                }
                            }
                        }

                        // Visualización explícita de subniveles spdf con formas características
                        let orbital_visualizations = [
                            ("1s", e_1s, 60.0, "rgba(255,152,0,0.4)", "s"),     // naranja - esfera
                            ("2s", e_2s, 90.0, "rgba(255,255,0,0.4)", "s"),    // amarillo - esfera más grande
                            ("2p", e_2p, 120.0, "rgba(76,175,80,0.4)", "p"),   // verde - formas de huso
                            ("3s", e_3s, 150.0, "rgba(156,39,176,0.4)", "s"),  // púrpura - esfera
                            ("3p", e_3p, 180.0, "rgba(33,150,243,0.4)", "p"),  // azul - husos
                            ("4s", e_4s, 210.0, "rgba(255,87,34,0.4)", "s"),   // naranja rojo - esfera
                            ("3d", e_3d, 240.0, "rgba(142,36,170,0.4)", "d"),  // violeta - formas de trébol
                            ("4f", e_4f, 280.0, "rgba(0,188,212,0.4)", "f"),   // cyan - formas complejas
                        ];
                        for (sublevel_name, e_count, radius, color, kind) in orbital_visualizations.iter() {
                            if *e_count > 0 {
                                // Visualizar formas características de cada subnivel
                                self.draw_subnivel_shape(context, center_x, center_y, *radius, color, kind, *e_count);

                                // Etiqueta del subnivel con número de electrones
                                context.set_fill_style(&"#ffffff".into());
                                context.set_font("bold 12px Arial");
                                context.set_text_align("center");
                                let label = format!("{}({}e⁻)", sublevel_name, e_count);
                                let label_y = match *kind {
                                    "s" => center_y + *radius + 20.0,
                                    "p" => center_y + *radius + 40.0,
                                    "d" => center_y + *radius + 60.0,
                                    "f" => center_y + *radius + 80.0,
                                    _ => center_y + *radius + 20.0,
                                };
                                let _ = context.fill_text(&label, center_x, label_y);
                            }
                        }

                        // Dibujar círculo de fondo del elemento con color único y 50% transparencia
                        self.draw_element_background(context, center_x, center_y, nucleus.protons.len());

                        // Dibujar letra del elemento en el centro del núcleo
                        self.draw_element_symbol(context, center_x, center_y, nucleus);
                    }
                }
            }
        }

        // Dibujar partículas
        for (idx, particle) in self.particles.iter().enumerate() {
            let in_nucleus = nucleus_particle_indices.contains(&idx);

            // Dibujar círculo principal
            context.begin_path();
            let draw_radius = if particle.ptype == ParticleType::Electron { particle.radius / 3.0 } else { particle.radius };
            let _ = context.arc(
                particle.x,
                particle.y,
                draw_radius,
                0.0,
                2.0 * std::f64::consts::PI,
            );

            match particle.ptype {
                ParticleType::Proton => {
                    let color = if in_nucleus {
                        "rgba(255, 0, 0, 0.05)"  // 95% transparencia (5% opacidad)
                    } else {
                        "#ff0000"
                    };
                    let _ = context.set_stroke_style(&color.into());
                    let _ = context.set_line_width(2.0);
                    let _ = context.stroke();
                    // Dibujar quarks (up, up, down) para protón
                    if !in_nucleus {
                        self.draw_quarks(context, particle.x, particle.y, particle.radius, &["up", "up", "down"]);
                    }
                }
                ParticleType::Neutron => {
                    let color = if in_nucleus {
                        "rgba(255, 255, 255, 0.05)"  // 95% transparencia
                    } else {
                        "#ffffff"
                    };
                    let _ = context.set_stroke_style(&color.into());
                    let _ = context.set_line_width(2.0);
                    let _ = context.stroke();
                    // Dibujar quarks (up, down, down) para neutrón
                    if !in_nucleus {
                        self.draw_quarks(context, particle.x, particle.y, particle.radius, &["up", "down", "down"]);
                    }
                }
                ParticleType::Electron => {
                    // Color según orbital cuántico
                    let color = if let Some((subnivel, principal_n, _orbital_idx, _pos_in_orbital)) = particle.orbital_position {
                        match (principal_n, subnivel) {
                            (1, 's') => "#ff9800",  // 1s: naranja
                            (2, 's') => "#ffff00",  // 2s: amarillo
                            (2, 'p') => "#4caf50",  // 2p: verde
                            (3, 's') => "#9c27b0",  // 3s: púrpura
                            (3, 'p') => "#2196f3",  // 3p: azul
                            (4, 's') => "#ff5722",  // 4s: naranja rojo
                            (3, 'd') => "#8e24aa",  // 3d: violeta
                            (4, 'f') => "#00bcd4",  // 4f: cyan
                            (5, 's') => "#795548",  // 5s: café
                            (4, 'd') => "#607d8b",  // 4d: azul gris
                            (5, 'p') => "#e91e63",  // 5p: rosa
                            (6, 's') => "#9e9e9e",  // 6s: gris
                            _ => "#ffffff",
                        }
                    } else {
                        "#ffffff" // Libre
                    };
                    let _ = context.set_fill_style(&color.into());
                    let _ = context.fill();
                }
            }
        }
    }

    fn get_element_symbol(proton_count: usize) -> &'static str {
        match proton_count {
            1 => "H",   // Hidrógeno
            2 => "He",  // Helio
            3 => "Li",  // Litio
            4 => "Be",  // Berilio
            5 => "B",   // Boro
            6 => "C",   // Carbono
            7 => "N",   // Nitrógeno
            8 => "O",   // Oxígeno
            9 => "F",   // Flúor
            10 => "Ne",  // Neón
            11 => "Na", // Sodio
            12 => "Mg", // Magnesio
            13 => "Al", // Aluminio
            14 => "Si", // Silicio
            15 => "P",  // Fósforo
            16 => "S",  // Azufre
            17 => "Cl", // Cloro
            18 => "Ar", // Argón
            19 => "K",  // Potasio
            20 => "Ca", // Calcio
            21 => "Sc", // Escandio
            22 => "Ti", // Titanio
            23 => "V",  // Vanadio
            24 => "Cr", // Cromo
            25 => "Mn", // Manganeso
            26 => "Fe", // Hierro
            27 => "Co", // Cobalto
            28 => "Ni", // Níquel
            29 => "Cu", // Cobre
            30 => "Zn", // Zinc
            31 => "Ga", // Galio
            32 => "Ge", // Germanio
            33 => "As", // Arsénico
            34 => "Se", // Selenio
            35 => "Br", // Bromo
            36 => "Kr", // Kriptón
            37 => "Rb", // Rubidio
            38 => "Sr", // Estroncio
            39 => "Y",  // Itrio
            40 => "Zr", // Circonio
            41 => "Nb", // Niobio
            42 => "Mo", // Molibdeno
            43 => "Tc", // Tecnecio
            44 => "Ru", // Rutenio
            45 => "Rh", // Rodio
            46 => "Pd", // Paladio
            47 => "Ag", // Plata
            48 => "Cd", // Cadmio
            49 => "In", // Indio
            50 => "Sn", // Estaño
            51 => "Sb", // Antimonio
            52 => "Te", // Telurio
            53 => "I",  // Yodo
            54 => "Xe", // Xenón
            55 => "Cs", // Cesio
            56 => "Ba", // Bario
            57 => "La", // Lantano
            58 => "Ce", // Cerio
            59 => "Pr", // Praseodimio
            60 => "Nd", // Neodimio
            61 => "Pm", // Prometio
            62 => "Sm", // Samario
            63 => "Eu", // Europio
            64 => "Gd", // Gadolinio
            65 => "Tb", // Terbio
            66 => "Dy", // Disprosio
            67 => "Ho", // Holmio
            68 => "Er", // Erbio
            69 => "Tm", // Tulio
            70 => "Yb", // Iterbio
            71 => "Lu", // Lutecio
            72 => "Hf", // Hafnio
            73 => "Ta", // Tántalo
            74 => "W",  // Tungsteno
            75 => "Re", // Renio
            76 => "Os", // Osmio
            77 => "Ir", // Iridio
            78 => "Pt", // Platino
            79 => "Au", // Oro
            80 => "Hg", // Mercurio
            81 => "Tl", // Talio
            82 => "Pb", // Plomo
            83 => "Bi", // Bismuto
            84 => "Po", // Polonio
            85 => "At", // Astato
            86 => "Rn", // Radón
            87 => "Fr", // Francio
            88 => "Ra", // Radio
            89 => "Ac", // Actinio
            90 => "Th", // Torio
            91 => "Pa", // Protactinio
            92 => "U",  // Uranio
            93 => "Np", // Neptunio
            94 => "Pu", // Plutonio
            95 => "Am", // Americio
            96 => "Cm", // Curio
            97 => "Bk", // Berkelio
            98 => "Cf", // Californio
            99 => "Es", // Einstenio
            100 => "Fm", // Fermio
            101 => "Md", // Mendelevio
            102 => "No", // Nobelio
            103 => "Lr", // Laurencio
            104 => "Rf", // Rutherfordio
            105 => "Db", // Dubnio
            106 => "Sg", // Seaborgio
            107 => "Bh", // Bohrio
            108 => "Hs", // Hassio
            109 => "Mt", // Meitnerio
            110 => "Ds", // Darmstadtio
            111 => "Rg", // Roentgenio
            112 => "Cn", // Copernicio
            113 => "Nh", // Nihonio
            114 => "Fl", // Flerovio
            115 => "Mc", // Moscovio
            116 => "Lv", // Livermorio
            117 => "Ts", // Tenesino
            118 => "Og", // Oganesson
            _ => "Unknown", // Para elementos más allá de 118
        }
    }

    fn get_element_color(proton_count: usize) -> &'static str {
        // Colores únicos para cada elemento con 50% transparencia
        match proton_count {
            1 => "rgba(255, 0, 127, 0.5)",     // Magenta para H
            2 => "rgba(0, 255, 255, 0.5)",     // Cian para He
            3 => "rgba(0, 255, 0, 0.5)",       // Verde para Li
            4 => "rgba(255, 165, 0, 0.5)",     // Naranja para Be
            5 => "rgba(255, 0, 255, 0.5)",     // Magenta para B
            6 => "rgba(255, 255, 0, 0.5)",     // Amarillo para C
            7 => "rgba(255, 20, 147, 0.5)",    // Rosa para N
            8 => "rgba(0, 191, 255, 0.5)",     // Azul para O
            9 => "rgba(127, 255, 0, 0.5)",     // Verde lima para F
            10 => "rgba(255, 105, 180, 0.5)",  // Rosa para Ne
            11 => "rgba(255, 0, 0, 0.5)",       // Rojo para Na
            12 => "rgba(192, 192, 192, 0.5)",  // Plata para Mg
            13 => "rgba(218, 165, 32, 0.5)",   // Dorado para Al
            14 => "rgba(128, 128, 128, 0.5)",  // Gris para Si
            15 => "rgba(255, 165, 0, 0.5)",    // Naranja para P
            16 => "rgba(255, 255, 0, 0.5)",    // Amarillo para S
            17 => "rgba(0, 255, 0, 0.5)",      // Verde para Cl
            18 => "rgba(138, 43, 226, 0.5)",   // Púrpura para Ar
            19 => "rgba(255, 0, 0, 0.5)",      // Rojo para K
            20 => "rgba(255, 255, 255, 0.5)",  // Blanco para Ca
            26 => "rgba(184, 134, 11, 0.5)",   // Marrón oscuro para Fe
            29 => "rgba(184, 115, 51, 0.5)",   // Cobriz para Cu
            47 => "rgba(192, 192, 192, 0.5)",  // Plata para Ag
            53 => "rgba(128, 0, 128, 0.5)",     // Púrpura oscuro para I
            54 => "rgba(0, 255, 255, 0.5)",     // Cian para Xe
            79 => "rgba(255, 215, 0, 0.5)",     // Oro para Au
            80 => "rgba(192, 192, 192, 0.5)",  // Plata para Hg
            82 => "rgba(128, 128, 128, 0.5)",  // Gris para Pb
            86 => "rgba(220, 20, 60, 0.5)",    // Carmesí para Rn
            88 => "rgba(0, 255, 0, 0.5)",      // Verde para Ra
            92 => "rgba(0, 191, 255, 0.5)",     // Azul para U
            _ => "rgba(255, 255, 255, 0.5)",   // Blanco por defecto
        }
    }

    fn draw_element_background(&self, context: &CanvasRenderingContext2d, x: f64, y: f64, proton_count: usize) {
        let radius = 20.0; // Tamaño similar a protones/neutrones
        context.begin_path();
        let _ = context.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI);
        let _ = context.set_fill_style(&Self::get_element_color(proton_count).into());
        let _ = context.fill();
        // Agregar contorno para mejor visibilidad
        let _ = context.set_stroke_style(&"#ffffff".into());
        let _ = context.set_line_width(1.0);
        let _ = context.stroke();
    }

    fn draw_element_symbol(&self, context: &CanvasRenderingContext2d, center_x: f64, center_y: f64, nucleus: &Nucleus) {
        // Calcular carga real: protones - electrones en este núcleo específico
        let charge = nucleus.protons.len() as i32 - nucleus.electrons.len() as i32;

        let symbol = Self::get_element_symbol(nucleus.protons.len());
        let display_text = if charge != 0 {
            let charge_abs = charge.abs();
            if charge_abs > 3 {
                // Limitar la visualización de cargas grandes a +3 o -3 máximo
                format!("{}{}{}", symbol, if charge > 0 { "+" } else { "-" }, 3)
            } else {
                format!("{}{}", symbol, if charge > 0 { "+" } else { "-" }.to_string().repeat(charge_abs as usize))
            }
        } else {
            symbol.to_string()
        };

        // Texto grande y legible en el centro exacto del núcleo
        let _ = context.set_fill_style(&"#ffffff".into()); // Texto blanco brillante
        let _ = context.set_font("bold 20px Arial"); // Tamaño más grande para legibilidad
        let _ = context.set_text_align("center");
        let _ = context.set_text_baseline("middle");

        let _ = context.fill_text(&display_text, center_x, center_y);
    }

    // Dibujar formas características de los subniveles orbitales
    fn draw_subnivel_shape(&self, context: &CanvasRenderingContext2d, center_x: f64, center_y: f64, radius: f64, color: &str, kind: &str, electron_count: usize) {
        context.set_stroke_style(&color.into());
        context.set_line_width(2.0);

        match kind {
            "s" => {
                // Subnivel s: forma esférica simétrica
                let dash_array = js_sys::Array::of2(&4.into(), &2.into());
                context.set_line_dash(&dash_array);
                context.begin_path();
                let _ = context.arc(center_x, center_y, radius, 0.0, 2.0 * std::f64::consts::PI);
                let _ = context.stroke();

                // Dibujar capa interna para mostrar densidad de probabilidad
                context.set_line_dash(&js_sys::Array::new()).ok();
                context.set_line_width(1.0);
                context.begin_path();
                let _ = context.arc(center_x, center_y, radius * 0.7, 0.0, 2.0 * std::f64::consts::PI);
                let _ = context.stroke();
            },

            "p" => {
                // Subnivel p: tres lóbulos orientados (px, py, pz)
                let dash_array = js_sys::Array::of2(&3.into(), &2.into());
                context.set_line_dash(&dash_array);
                for (i, angle) in [0.0, std::f64::consts::PI/3.0, 2.0*std::f64::consts::PI/3.0].iter().enumerate() {
                    if i < electron_count.min(3) {
                        let lobe_angle = *angle;
                        let lobe_x = center_x + lobe_angle.cos() * radius * 0.8;
                        let lobe_y = center_y + lobe_angle.sin() * radius * 0.8;

                        // Dibujar lóbulo en forma de 8
                        context.begin_path();
                        context.ellipse(
                            lobe_x, lobe_y,
                            radius * 0.3, radius * 0.15,
                            lobe_angle,
                            0.0, 2.0 * std::f64::consts::PI
                        );
                        let _ = context.stroke();

                        // Lóbulo opuesto (signo negativo)
                        let opposite_x = center_x - lobe_angle.cos() * radius * 0.8;
                        let opposite_y = center_y - lobe_angle.sin() * radius * 0.8;
                        context.begin_path();
                        context.ellipse(
                            opposite_x, opposite_y,
                            radius * 0.3, radius * 0.15,
                            lobe_angle + std::f64::consts::PI,
                            0.0, 2.0 * std::f64::consts::PI
                        );
                        let _ = context.stroke();
                    }
                }
            },

            "d" => {
                // Subnivel d: cinco lóbulos en forma de trébol
                let dash_array = js_sys::Array::of2(&2.into(), &2.into());
                context.set_line_dash(&dash_array);
                for i in 0..electron_count.min(5) {
                    let angle = (i as f64) * 2.0 * std::f64::consts::PI / 5.0;
                    let lobe_x = center_x + angle.cos() * radius * 0.6;
                    let lobe_y = center_y + angle.sin() * radius * 0.6;

                    // Lóbulos con forma de pétalo
                    context.begin_path();
                    context.ellipse(
                        lobe_x, lobe_y,
                        radius * 0.25, radius * 0.12,
                        angle + std::f64::consts::PI/2.0,
                        0.0, 2.0 * std::f64::consts::PI
                    );
                    let _ = context.stroke();

                    // Lóbulo perpendicular
                    let perp_x = center_x + angle.cos() * radius * 0.6;
                    let perp_y = center_y + angle.sin() * radius * 0.6;
                    context.begin_path();
                    context.ellipse(
                        perp_x, perp_y,
                        radius * 0.15, radius * 0.25,
                        angle,
                        0.0, 2.0 * std::f64::consts::PI
                    );
                    let _ = context.stroke();
                }
            },

            "f" => {
                // Subnivel f: siete lóbulos con formas complejas
                let dash_array = js_sys::Array::of2(&1.into(), &1.into());
                context.set_line_dash(&dash_array);
                for i in 0..electron_count.min(7) {
                    let base_angle = (i as f64) * 2.0 * std::f64::consts::PI / 7.0;

                    // Forma compleja con múltiples lóbulos pequeños
                    for j in 0..3 {
                        let offset_angle = (j as f64) * std::f64::consts::PI / 6.0;
                        let lobe_angle = base_angle + offset_angle;
                        let lobe_radius = radius * (0.4 + j as f64 * 0.1);

                        let lobe_x = center_x + lobe_angle.cos() * lobe_radius;
                        let lobe_y = center_y + lobe_angle.sin() * lobe_radius;

                        context.begin_path();
                        context.ellipse(
                            lobe_x, lobe_y,
                            radius * 0.12, radius * 0.08,
                            lobe_angle,
                            0.0, 2.0 * std::f64::consts::PI
                        );
                        let _ = context.stroke();
                    }
                }
            },

            _ => {}
        }

        // Reset line dash
        context.set_line_dash(&js_sys::Array::new()).ok();
    }

    // Calcular movimiento orbital realista dentro de las formas de los orbitales
    fn calculate_orbital_motion(&self, nucleus_x: f64, nucleus_y: f64, subnivel: char, principal_n: usize, orbital_idx: usize, pos_in_orbital: usize, time: f64) -> (f64, f64, f64, f64) {
        let base_radius = match principal_n {
            1 => 50.0,  // 1s
            2 => match subnivel {
                's' => 75.0,   // 2s
                'p' => 100.0,  // 2p
                _ => 85.0,
            },
            3 => match subnivel {
                's' => 125.0,  // 3s
                'p' => 150.0,  // 3p
                'd' => 175.0,  // 3d
                _ => 140.0,
            },
            4 => match subnivel {
                's' => 200.0,  // 4s
                'p' => 225.0,  // 4p
                'd' => 250.0,  // 4d
                'f' => 275.0,  // 4f
                _ => 215.0,
            },
            _ => 100.0 + (principal_n as f64 * 25.0),
        };

        match subnivel {
            's' => {
                // Movimiento esférico para orbitales s
                let angle = time + pos_in_orbital as f64 * std::f64::consts::PI;
                let x = nucleus_x + base_radius * angle.cos();
                let y = nucleus_y + base_radius * angle.sin();
                let speed = 4.0;
                let dx = -speed * angle.sin();
                let dy = speed * angle.cos();
                (x, y, dx, dy)
            },

            'p' => {
                // Movimiento en forma de 8 para orbitales p
                let base_angle = (orbital_idx as f64) * 2.0 * std::f64::consts::PI / 3.0;
                let motion_angle = time * 2.0;

                // Figura de 8 paramétrica
                let t = motion_angle;
                let figure8_x = base_radius * 0.4 * t.sin();
                let figure8_y = base_radius * 0.8 * t.cos() * 0.5;

                // Rotar según el orbital específico
                let x = nucleus_x + base_angle.cos() * figure8_x - base_angle.sin() * figure8_y;
                let y = nucleus_y + base_angle.sin() * figure8_x + base_angle.cos() * figure8_y;

                let speed = 3.5;
                let dx = speed * figure8_x.cos() * base_angle.cos() - speed * figure8_y.sin() * base_angle.sin();
                let dy = speed * figure8_x.cos() * base_angle.sin() + speed * figure8_y.sin() * base_angle.cos();
                (x, y, dx, dy)
            },

            'd' => {
                // Movimiento en forma de trébol para orbitales d
                let base_angle = (orbital_idx as f64) * 2.0 * std::f64::consts::PI / 5.0;
                let petal_angle = time * 1.5;

                // Movimiento pétalo de trébol
                let r = base_radius * 0.6 * (1.0 + 0.5 * (5.0 * petal_angle).cos());
                let x = nucleus_x + r * (base_angle + petal_angle).cos();
                let y = nucleus_y + r * (base_angle + petal_angle).sin();

                let speed = 2.8;
                let dx = -speed * (base_angle + petal_angle).sin() + speed * 0.5 * (5.0 * petal_angle).sin() * (base_angle + petal_angle).cos();
                let dy = speed * (base_angle + petal_angle).cos() + speed * 0.5 * (5.0 * petal_angle).sin() * (base_angle + petal_angle).sin();
                (x, y, dx, dy)
            },

            'f' => {
                // Movimiento complejo para orbitales f
                let base_angle = (orbital_idx as f64) * 2.0 * std::f64::consts::PI / 7.0;
                let complex_angle = time * 1.2;

                // Movimiento con múltiples frecuencias
                let r1 = base_radius * 0.4;
                let r2 = base_radius * 0.3;

                let x = nucleus_x + r1 * (base_angle + complex_angle).cos() + r2 * (base_angle - complex_angle * 1.5).cos();
                let y = nucleus_y + r1 * (base_angle + complex_angle).sin() + r2 * (base_angle - complex_angle * 1.5).sin();

                let speed = 2.2;
                let dx = -speed * r1 * (base_angle + complex_angle).sin() + speed * r2 * 1.5 * (base_angle - complex_angle * 1.5).sin();
                let dy = speed * r1 * (base_angle + complex_angle).cos() - speed * r2 * 1.5 * (base_angle - complex_angle * 1.5).cos();
                (x, y, dx, dy)
            },

            _ => {
                // Movimiento circular por defecto
                let angle = time;
                let x = nucleus_x + base_radius * angle.cos();
                let y = nucleus_y + base_radius * angle.sin();
                let speed = 3.0;
                let dx = -speed * angle.sin();
                let dy = speed * angle.cos();
                (x, y, dx, dy)
            }
        }
    }

    
    fn draw_quarks(&self, context: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, quark_types: &[&str]) {
        const PI: f64 = std::f64::consts::PI;
        let num_quarks = quark_types.len() as f64;

        for (i, quark_type) in quark_types.iter().enumerate() {
            let angle = (i as f64 * 2.0 * PI) / num_quarks;
            let qx = x + angle.cos() * (radius * 0.5);
            let qy = y + angle.sin() * (radius * 0.5);

            context.begin_path();
            let _ = context.arc(qx, qy, 5.0, 0.0, 2.0 * PI);

            let fill_color = if *quark_type == "up" { "#ff6b6b" } else { "#4ecdc4" };
            let _ = context.set_fill_style(&fill_color.into());
            let _ = context.fill();
        }
    }
}
