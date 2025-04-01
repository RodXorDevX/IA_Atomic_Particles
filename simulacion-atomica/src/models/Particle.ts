export type ParticleType = 'proton' | 'neutron' | 'electron';
export type OrbitalType = 'none' | 's' | 'p' | 'd' | 'f' | 'g';

export interface ParticleProps {
    x: number;
    y: number;
    type: ParticleType;
    orbitalType?: OrbitalType;
    radius?: number;
    charge?: number;
    mass?: number;
    dx?: number;
    dy?: number;
}

export class Particle {
    x: number;
    y: number;
    type: ParticleType;
    orbitalType: OrbitalType;
    radius: number;
    charge: number;
    mass: number;
    dx: number;
    dy: number;

    constructor({
        x,
        y,
        type,
        orbitalType = 'none',
        radius,
        charge,
        mass,
        dx = (Math.random() - 0.5) * 2,
        dy = (Math.random() - 0.5) * 2
    }: ParticleProps) {
        this.x = x;
        this.y = y;
        this.type = type;
        this.orbitalType = orbitalType;
        this.radius = radius || (type === 'electron' ? 10 : 20);
        this.charge = charge || (type === 'proton' ? 1 : type === 'electron' ? -1 : 0);
        this.mass = mass || (type === 'electron' ? 0.0005 : 3.0);
        this.dx = dx;
        this.dy = dy;
    }

    update(canvasWidth: number, canvasHeight: number, damping: number): void {
        // Colisiones con bordes
        if (this.x + this.radius > canvasWidth) {
            this.x = canvasWidth - this.radius;
            this.dx *= -0.9;
        }
        if (this.x - this.radius < 0) {
            this.x = this.radius;
            this.dx *= -0.9;
        }
        if (this.y + this.radius > canvasHeight) {
            this.y = canvasHeight - this.radius;
            this.dy *= -0.9;
        }
        if (this.y - this.radius < 0) {
            this.y = this.radius;
            this.dy *= -0.9;
        }

        // Actualizar posición
        this.x += this.dx;
        this.y += this.dy;

        // Aplicar amortiguación
        this.dx *= damping;
        this.dy *= damping;
    }
} 