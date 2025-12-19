mod particle;
mod physics;
mod simulation;
mod utils;

pub use particle::{Particle, ParticleType};
pub use physics::PhysicsParams;
pub use simulation::Simulation;

use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub struct SimulationEngine {
    simulation: Simulation,
}

#[wasm_bindgen]
impl SimulationEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_width: f64, canvas_height: f64) -> SimulationEngine {
        utils::set_panic_hook();
        SimulationEngine {
            simulation: Simulation::new(canvas_width, canvas_height),
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) {
        self.simulation.update();
    }

    #[wasm_bindgen]
    pub fn render(&self, context: &CanvasRenderingContext2d) {
        self.simulation.render(context);
    }

    #[wasm_bindgen]
    pub fn add_particle(&mut self, x: f64, y: f64, particle_type: u32) {
        let ptype = match particle_type {
            0 => ParticleType::Proton,
            1 => ParticleType::Neutron,
            2 => ParticleType::Electron,
            _ => ParticleType::Electron,
        };
        self.simulation.add_particle(x, y, ptype);
    }

    #[wasm_bindgen]
    pub fn add_particle_with_velocity(&mut self, x: f64, y: f64, particle_type: u32, dx: f64, dy: f64) {
        let ptype = match particle_type {
            0 => ParticleType::Proton,
            1 => ParticleType::Neutron,
            2 => ParticleType::Electron,
            _ => ParticleType::Electron,
        };
        self.simulation.add_particle_with_velocity(x, y, ptype, dx, dy);
    }

    #[wasm_bindgen]
    pub fn add_nucleus(&mut self, x: f64, y: f64, protons: usize, neutrons: usize, electrons: usize) {
        self.simulation.add_nucleus(x, y, protons, neutrons, electrons);
    }

    #[wasm_bindgen]
    pub fn add_ion(&mut self, x: f64, y: f64, protons: usize, neutrons: usize, electrons: usize, charge: i32) {
        self.simulation.add_nucleus_with_charge(x, y, protons, neutrons, electrons, charge);
    }

    #[wasm_bindgen]
    pub fn clear(&mut self) {
        self.simulation.clear();
    }

    #[wasm_bindgen]
    pub fn set_nuclear_force(&mut self, force: f64) {
        self.simulation.params.nuclear_force = force;
    }

    #[wasm_bindgen]
    pub fn set_nuclear_range(&mut self, range: f64) {
        self.simulation.params.nuclear_range = range;
    }

    #[wasm_bindgen]
    pub fn set_nuclear_attractive(&mut self, force: f64) {
        self.simulation.params.nuclear_attractive = force;
    }

    #[wasm_bindgen]
    pub fn set_nuclear_damping(&mut self, damping: f64) {
        self.simulation.params.nuclear_damping = damping;
    }

    #[wasm_bindgen]
    pub fn set_proton_repulsion(&mut self, repulsion: f64) {
        self.simulation.params.proton_repulsion = repulsion;
    }

    #[wasm_bindgen]
    pub fn set_coulomb_force(&mut self, force: f64) {
        self.simulation.params.coulomb_force = force;
    }

    #[wasm_bindgen]
    pub fn set_orbital_speed(&mut self, speed: f64) {
        self.simulation.params.orbital_speed = speed;
    }

    #[wasm_bindgen]
    pub fn set_orbital_radius(&mut self, radius: f64) {
        self.simulation.params.orbital_radius = radius;
    }

    #[wasm_bindgen]
    pub fn set_min_orbital_radius(&mut self, radius: f64) {
        self.simulation.params.min_orbital_radius = radius;
    }

    #[wasm_bindgen]
    pub fn set_max_orbital_radius(&mut self, radius: f64) {
        self.simulation.params.max_orbital_radius = radius;
    }

    #[wasm_bindgen]
    pub fn set_electron_repulsion(&mut self, repulsion: f64) {
        self.simulation.params.electron_repulsion = repulsion;
    }

    #[wasm_bindgen]
    pub fn set_nuclear_mass(&mut self, mass: f64) {
        self.simulation.params.nuclear_mass = mass;
    }

    #[wasm_bindgen]
    pub fn set_electron_mass(&mut self, mass: f64) {
        self.simulation.params.electron_mass = mass;
    }

    #[wasm_bindgen]
    pub fn set_electron_damping(&mut self, damping: f64) {
        self.simulation.params.electron_damping = damping;
    }

    #[wasm_bindgen]
    pub fn get_particle_count(&self) -> usize {
        self.simulation.particles.len()
    }

    #[wasm_bindgen]
    pub fn get_particle_data(&self, index: usize) -> String {
        if let Some(particle) = self.simulation.particles.get(index) {
            format!(
                r#"{{"x":{},"y":{},"type":{},"radius":{}}}"#,
                particle.x,
                particle.y,
                match particle.ptype {
                    ParticleType::Proton => 0,
                    ParticleType::Neutron => 1,
                    ParticleType::Electron => 2,
                },
                particle.radius
            )
        } else {
            String::new()
        }
    }

    #[wasm_bindgen]
    pub fn get_nuclei_data(&self) -> String {
        let nuclei = self.simulation.detect_nuclei();
        let mut json = String::from("[");
        for (i, nucleus) in nuclei.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push_str(&format!(
                r#"{{"protons":{},"neutrons":{},"electrons":{}}}"#,
                nucleus.protons.len(),
                nucleus.neutrons.len(),
                nucleus.electrons.len()
            ));
        }
        json.push(']');
        json
    }
}

// Configuración de pánico para Wasm
pub fn set_panic_hook() {
    // Dummy implementation
}
