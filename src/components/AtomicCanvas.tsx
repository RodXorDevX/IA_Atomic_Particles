import React, { useRef, useState } from 'react';
import { Particle, ParticleType } from '../models/Particle';
import { useAnimationFrame } from '../hooks/useAnimationFrame';
import { drawParticle } from '../utils/drawingUtils';
import { calculateForces } from '../utils/physicsUtils';
import '../styles/AtomicCanvas.css';

interface AtomicCanvasProps {
    width: number;
    height: number;
}

export const AtomicCanvas: React.FC<AtomicCanvasProps> = ({ width, height }) => {
    const canvasRef = useRef<HTMLCanvasElement>(null);
    const [particles, setParticles] = useState<Particle[]>([]);
    const [isRunning, setIsRunning] = useState(true);

    const addParticle = (type: ParticleType) => {
        const x = Math.random() * (width - 40) + 20;
        const y = Math.random() * (height - 40) + 20;
        const newParticle = new Particle({ x, y, type });
        setParticles(prev => [...prev, newParticle]);
    };

    const clearCanvas = () => {
        setParticles([]);
    };

    const animate = () => {
        if (!canvasRef.current || !isRunning) return;
        
        const ctx = canvasRef.current.getContext('2d');
        if (!ctx) return;

        // Limpiar canvas
        ctx.clearRect(0, 0, width, height);

        // Calcular fuerzas y actualizar partículas
        const updatedParticles = calculateForces([...particles]);
        updatedParticles.forEach(particle => {
            particle.update(width, height, particle.type === 'electron' ? 0.99 : 0.7);
            drawParticle(ctx, particle);
        });

        setParticles(updatedParticles);
    };

    useAnimationFrame(animate, isRunning);

    return (
        <div className="atomic-canvas-container">
            <div className="controls">
                <button onClick={() => addParticle('proton')}>+ Protón</button>
                <button onClick={() => addParticle('neutron')}>+ Neutrón</button>
                <button onClick={() => addParticle('electron')}>+ Electrón</button>
                <button onClick={clearCanvas}>Limpiar</button>
                <button onClick={() => setIsRunning(!isRunning)}>
                    {isRunning ? 'Pausar' : 'Reanudar'}
                </button>
            </div>
            <canvas
                ref={canvasRef}
                width={width}
                height={height}
                className="atomic-canvas"
            />
        </div>
    );
}; 