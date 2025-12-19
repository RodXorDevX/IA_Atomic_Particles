use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ParticleType {
    Proton,
    Neutron,
    Electron,
}

#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub ptype: ParticleType,
    pub radius: f64,
    pub charge: f64,
    pub mass: f64,
    pub orbiting_around: Option<usize>,                           // Índice del protón alrededor del que orbita
    pub orbital_position: Option<(char, usize, usize, usize)>, // (subnivel, nivel, orbital_idx, posicion_en_orbital)
}

impl Particle {
    pub fn new(x: f64, y: f64, ptype: ParticleType, mass: f64) -> Self {
        let (radius, charge) = match ptype {
            ParticleType::Proton => (20.0, 1.0),
            ParticleType::Neutron => (20.0, 0.0),
            ParticleType::Electron => (10.0, -1.0),
        };

        Particle {
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            ptype,
            radius,
            charge,
            mass,
            orbiting_around: None,
            orbital_position: None,
        }
    }

    pub fn update(&mut self, canvas_width: f64, canvas_height: f64, damping: f64) {
        // Actualizar posición
        self.x += self.dx;
        self.y += self.dy;

        // Colisiones elásticas con bordes (sin pérdida de energía)
        let reflection = match self.ptype {
            ParticleType::Electron => -1.0, // Colisión completamente elástica
            _ => -1.0, // Colisión completamente elástica
        };

        if self.x + self.radius > canvas_width {
            self.x = canvas_width - self.radius;
            self.dx *= reflection;
        }
        if self.x - self.radius < 0.0 {
            self.x = self.radius;
            self.dx *= reflection;
        }
        if self.y + self.radius > canvas_height {
            self.y = canvas_height - self.radius;
            self.dy *= reflection;
        }
        if self.y - self.radius < 0.0 {
            self.y = self.radius;
            self.dy *= reflection;
        }

        // Aplicar amortiguación mínima
        self.dx *= damping;
        self.dy *= damping;
    }

    pub fn distance_to(&self, other: &Particle) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
