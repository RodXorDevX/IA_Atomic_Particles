import { Particle } from '../models/Particle';

// Constantes físicas
const k = 8.9875e9; // Constante de Coulomb en N·m²/C²
const q_proton = 1.602e-19; // Carga del protón en Coulombs
const q_electron = -1.602e-19; // Carga del electrón en Coulombs
const scale = 1e-12; // Escala para convertir metros a picómetros

export const calculateForces = (particles: Particle[]): Particle[] => {
    for (let i = 0; i < particles.length; i++) {
        for (let j = i + 1; j < particles.length; j++) {
            const dx = (particles[j].x - particles[i].x) * scale;
            const dy = (particles[j].y - particles[i].y) * scale;
            const distance = Math.sqrt(dx * dx + dy * dy);
            
            if (distance === 0) continue;
            
            let totalForce = 0;
            
            // Repulsión protón-protón (Coulomb)
            if (particles[i].type === 'proton' && particles[j].type === 'proton') {
                totalForce = (k * q_proton * q_proton) / (distance * distance);
            }
            
            // Interacción protón-electrón (Coulomb)
            else if ((particles[i].type === 'proton' && particles[j].type === 'electron') ||
                     (particles[i].type === 'electron' && particles[j].type === 'proton')) {
                totalForce = -(k * q_proton * q_electron) / (distance * distance);
                totalForce *= 500; // Ajustar el escalamiento
            }
            
            // Repulsión electrón-electrón (Coulomb)
            else if (particles[i].type === 'electron' && particles[j].type === 'electron') {
                totalForce = (k * q_electron * q_electron) / (distance * distance);
                totalForce *= 1000;
            }
            
            // Fuerza nuclear fuerte (corta distancia)
            else if ((particles[i].type === 'proton' && particles[j].type === 'neutron') ||
                     (particles[i].type === 'neutron' && particles[j].type === 'proton')) {
                const nuclearRange = 40 * scale;
                if (distance < nuclearRange) {
                    totalForce = -30 * Math.exp(-distance / nuclearRange);
                }
            }
            
            if (totalForce !== 0) {
                const forceMagnitude = totalForce / distance;
                const fx = forceMagnitude * dx;
                const fy = forceMagnitude * dy;
                
                particles[i].dx -= fx / particles[i].mass;
                particles[i].dy -= fy / particles[i].mass;
                particles[j].dx += fx / particles[j].mass;
                particles[j].dy += fy / particles[j].mass;
            }
        }
    }
    
    return particles;
}; 