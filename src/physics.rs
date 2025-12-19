#[derive(Debug, Clone)]
pub struct PhysicsParams {
    pub nuclear_force: f64,
    pub nuclear_range: f64,
    pub nuclear_attractive: f64,
    pub nuclear_damping: f64,
    pub proton_repulsion: f64,
    pub coulomb_force: f64,
    pub orbital_speed: f64,
    pub orbital_radius: f64,
    pub min_orbital_radius: f64,
    pub max_orbital_radius: f64,
    pub electron_repulsion: f64,
    pub nuclear_mass: f64,
    pub electron_mass: f64,
    pub electron_damping: f64,
    pub energy_level: u32,
}

impl Default for PhysicsParams {
    fn default() -> Self {
        PhysicsParams {
            nuclear_force: 25.0,
            nuclear_range: 40.0,
            nuclear_attractive: 30.0,
            nuclear_damping: 1.0,       // SIN pérdida - sistema completamente conservativo
            proton_repulsion: 15.0,
            coulomb_force: 3.0,
            orbital_speed: 8.0,
            orbital_radius: 100.0,
            min_orbital_radius: 50.0,
            max_orbital_radius: 150.0,
            electron_repulsion: 2.0,
            nuclear_mass: 3.0,
            electron_mass: 0.01,
            electron_damping: 1.0,      // SIN pérdida - sistema completamente conservativo
            energy_level: 1,
        }
    }
}

pub struct Physics;

impl Physics {
    pub fn calculate_nuclear_force(
        distance: f64,
        params: &PhysicsParams,
    ) -> f64 {
        if distance > params.nuclear_range {
            return 0.0;
        }

        // Fuerza atractiva fuerte a corta distancia
        let attractive = if distance < params.nuclear_range * 0.5 {
            params.nuclear_attractive * (1.0 - distance / params.nuclear_range)
        } else {
            0.0
        };

        attractive
    }

    pub fn calculate_coulomb_force(
        distance: f64,
        charge1: f64,
        charge2: f64,
        params: &PhysicsParams,
    ) -> f64 {
        if distance < 1.0 {
            return 0.0;
        }

        let k = 8.9875e9;
        let coulomb = (k * charge1 * charge2) / (distance * distance) * params.coulomb_force * 1e-15;
        coulomb
    }

    pub fn calculate_orbital_motion(
        _distance_to_nucleus: f64,
        orbital_index: usize,
        params: &PhysicsParams,
    ) -> (f64, f64) {
        let target_radius = params.orbital_radius + (orbital_index as f64 * 30.0);
        let speed = params.orbital_speed * (orbital_index as f64 + 1.0).sqrt();

        // Velocidad orbital tangencial
        (speed, target_radius)
    }

    pub fn apply_force(
        particle: &mut super::particle::Particle,
        fx: f64,
        fy: f64,
    ) {
        let acceleration_x = fx / particle.mass;
        let acceleration_y = fy / particle.mass;
        
        particle.dx += acceleration_x;
        particle.dy += acceleration_y;
    }
}
