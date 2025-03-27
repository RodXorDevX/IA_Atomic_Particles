const canvas = document.getElementById('particleCanvas');
const ctx = canvas.getContext('2d');
let particles = [];
let isRunning = false;

class Particle {
    constructor(x, y, type) {
        this.x = x;
        this.y = y;
        this.type = type; // 'proton' o 'neutron'
        this.radius = 20;
        this.dx = (Math.random() - 0.5) * 4;
        this.dy = (Math.random() - 0.5) * 4;
        this.quarks = this.initQuarks();
    }

    initQuarks() {
        if (this.type === 'proton') {
            return [
                { type: 'up', color: '#ff6b6b' },
                { type: 'up', color: '#ff6b6b' },
                { type: 'down', color: '#4ecdc4' }
            ];
        } else {
            return [
                { type: 'up', color: '#ff6b6b' },
                { type: 'down', color: '#4ecdc4' },
                { type: 'down', color: '#4ecdc4' }
            ];
        }
    }

    update() {
        // Rebotar en los bordes
        if (this.x + this.radius > canvas.width || this.x - this.radius < 0) {
            this.dx = -this.dx;
        }
        if (this.y + this.radius > canvas.height || this.y - this.radius < 0) {
            this.dy = -this.dy;
        }

        // Actualizar posición
        this.x += this.dx;
        this.y += this.dy;
    }

    draw(ctx) {
        // Dibujar partícula principal
        ctx.beginPath();
        ctx.arc(this.x, this.y, this.radius, 0, Math.PI * 2);
        ctx.strokeStyle = this.type === 'proton' ? '#ff0000' : '#ffffff';
        ctx.stroke();
        
        // Dibujar quarks
        this.quarks.forEach((quark, i) => {
            const angle = (i * 2 * Math.PI) / 3;
            const qx = this.x + Math.cos(angle) * (this.radius/2);
            const qy = this.y + Math.sin(angle) * (this.radius/2);
            
            ctx.beginPath();
            ctx.arc(qx, qy, 5, 0, Math.PI * 2);
            ctx.fillStyle = quark.color;
            ctx.fill();
        });
    }
}

// Ajustar tamaño del canvas
function resizeCanvas() {
    canvas.width = window.innerWidth * 0.8;
    canvas.height = window.innerHeight * 0.6;
}

// Exponer funciones al ámbito global
globalThis.addProton = function() {
    const x = Math.random() * (canvas.width - 40) + 20;
    const y = Math.random() * (canvas.height - 40) + 20;
    particles.push(new Particle(x, y, 'proton'));
    if (!isRunning) {
        isRunning = true;
        animate();
    }
};

globalThis.addNeutron = function() {
    const x = Math.random() * (canvas.width - 40) + 20;
    const y = Math.random() * (canvas.height - 40) + 20;
    particles.push(new Particle(x, y, 'neutron'));
    if (!isRunning) {
        isRunning = true;
        animate();
    }
};

globalThis.toggleSimulation = function() {
    isRunning = !isRunning;
    if (isRunning) animate();
};

function animate() {
    if (!isRunning) return;
    
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    particles.forEach(particle => {
        particle.update();
        particle.draw(ctx);
    });

    // Simular interacciones entre partículas
    for (let i = 0; i < particles.length; i++) {
        for (let j = i + 1; j < particles.length; j++) {
            const dx = particles[i].x - particles[j].x;
            const dy = particles[i].y - particles[j].y;
            const distance = Math.sqrt(dx * dx + dy * dy);
            
            if (distance < 50) {
                const angle = Math.atan2(dy, dx);
                const force = 0.5;
                
                particles[i].dx += Math.cos(angle) * force;
                particles[i].dy += Math.sin(angle) * force;
                particles[j].dx -= Math.cos(angle) * force;
                particles[j].dy -= Math.sin(angle) * force;
            }
        }
    }

    requestAnimationFrame(animate);
}

// Inicialización
window.addEventListener('load', () => {
    resizeCanvas();
    // Añadir partículas iniciales
    addProton();
    addProton();
    addNeutron();
});

window.addEventListener('resize', resizeCanvas);
```
