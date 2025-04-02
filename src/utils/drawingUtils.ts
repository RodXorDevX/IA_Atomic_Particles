import { Particle } from '../models/Particle';

export const drawParticle = (ctx: CanvasRenderingContext2D, particle: Particle) => {
    ctx.beginPath();
    ctx.arc(particle.x, particle.y, particle.radius, 0, Math.PI * 2);
    ctx.strokeStyle = particle.type === 'proton' ? '#ff0000' : 
                     particle.type === 'neutron' ? '#ffffff' : '#00ffff';
    ctx.stroke();

    if (particle.type !== 'electron') {
        drawQuarks(ctx, particle);
    }
};

const drawQuarks = (ctx: CanvasRenderingContext2D, particle: Particle) => {
    const types = particle.type === 'proton' ? ['up', 'up', 'down'] : ['up', 'down', 'down'];
    
    types.forEach((type, i) => {
        const angle = (i * 2 * Math.PI) / 3;
        const qx = particle.x + Math.cos(angle) * (particle.radius * 0.5);
        const qy = particle.y + Math.sin(angle) * (particle.radius * 0.5);
        
        ctx.beginPath();
        ctx.arc(qx, qy, 5, 0, Math.PI * 2);
        ctx.fillStyle = type === 'up' ? '#ff6b6b' : '#4ecdc4';
        ctx.fill();
    });
}; 