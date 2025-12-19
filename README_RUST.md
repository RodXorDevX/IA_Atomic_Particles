# Simulación de Núcleos Atómicos - Rust + WebAssembly

Una simulación interactiva de partículas atómicas completamente migrada a **Rust** usando **WebAssembly** para máximo rendimiento.

## 🚀 Características

- **100% Rust**: Lógica de física y simulación completamente implementada en Rust
- **WebAssembly**: Compilación a WASM para máximo rendimiento en el navegador
- **Física Realista**: 
  - Fuerzas nucleares fuertes
  - Repulsión de Coulomb (electrostática)
  - Fuerzas atractivas entre nucleones
  - Movimiento orbital de electrones
- **Interactivo**: Controles en tiempo real para ajustar parámetros físicos
- **Visualización**: Canvas 2D con renderización smooth

## 📋 Requisitos

- Rust 1.70+
- `wasm-pack`: `cargo install wasm-pack`
- Node.js 16+
- npm

## 🔨 Compilación

### Compilar el proyecto WebAssembly

```bash
# Compilar solo el WASM
npm run build-wasm

# O compilar todo y servir en desarrollo
npm run dev

# Construir para producción
npm run build
```

### Estructura del Proyecto Rust

```
src/rust/
├── lib.rs              # Punto de entrada y bindings WASM
├── particle.rs         # Definición de partículas
├── physics.rs          # Cálculos de física
├── simulation.rs       # Motor de simulación
└── utils.rs           # Utilidades
```

## 📚 Módulos Rust

### `particle.rs`
Define las partículas (Protón, Neutrón, Electrón) con sus propiedades:
- Posición y velocidad
- Masa y carga
- Radio de colisión

### `physics.rs`
Calcula todas las fuerzas:
- Fuerza nuclear fuerte
- Fuerza de Coulomb (electrostática)
- Repulsión protón-protón
- Repulsión electrón-electrón

### `simulation.rs`
Motor principal que:
- Gestiona colecciones de partículas
- Actualiza posiciones y velocidades
- Detecta núcleos atómicos
- Renderiza en canvas

### `lib.rs`
Expone la API WebAssembly mediante `wasm-bindgen`:
- `SimulationEngine`: Clase principal
- Getters/Setters para parámetros
- Métodos de actualización y renderización

## 🎮 Uso

1. Instalar dependencias:
```bash
npm install
```

2. Compilar y ejecutar en desarrollo:
```bash
npm run dev
```

3. Compilar para producción:
```bash
npm run build
```

## 🧪 Parámetros Ajustables

### Fuerzas Nucleares
- **Fuerza Nuclear Fuerte**: Atracción entre nucleones (0-50)
- **Rango Nuclear**: Distancia de acción de la fuerza (20-150)
- **Fuerza Atractiva**: Magnitud de atracción (0-50)
- **Amortiguación**: Fricción en el núcleo (0-1)

### Interacciones Electrostáticas
- **Repulsión Protón-Protón**: Carga repulsiva (0-30)
- **Fuerza Coulomb**: Factor de atracción electrón-protón (0-10)
- **Velocidad Orbital**: Rapidez del movimiento orbital (0-20)
- **Repulsión Electrón-Electrón**: Carga repulsiva (0-5)

### Masas y Amortiguación
- **Masa Nuclear**: Inercia de protones/neutrones (1-10)
- **Masa Electrón**: Inercia de electrones (0.001-0.1)
- **Amortiguación Electrón**: Fricción del electrón (0.9-1)

## 🏗️ Cambios Principales de la Migración

### De JavaScript a Rust

1. **Gestión de Memoria**: Rust maneja automáticamente memoria sin garbage collection
2. **Type Safety**: Sistema de tipos fuerte evita errores en tiempo de compilación
3. **Rendimiento**: Compilación a código máquina nativo vía WASM
4. **Concurrencia**: Potencial para paralelización (futuro)

### Ventajas del WASM

- **~100x más rápido** que JavaScript puro
- **Mejor compresión** de código
- **Determinístico**: Sin variaciones de GC
- **Seguro**: Memoria aislada en sandbox

## 📊 Ecuaciones de Física Implementadas

### Fuerza Nuclear Fuerte
```
F_nuclear = N_attractive * (1 - distance/N_range) if distance < N_range
```

### Fuerza de Coulomb
```
F_coulomb = k * q1 * q2 / r² * coulomb_factor * scale
```

### Segunda Ley de Newton (Discreta)
```
a = F / m
v_new = v_old + a * dt
x_new = x_old + v_new * dt
```

## 🐛 Troubleshooting

### Error: "wasm-pack not found"
```bash
cargo install wasm-pack
```

### Error: "module not found atomic_particles_simulation"
```bash
npm run build-wasm
```

### Canvas negro sin partículas
- Verificar consola del navegador para errores
- Asegurar que el WASM se compiló correctamente
- Actualizar la página

## 📝 Notas de Desarrollo

- El código Rust se encuentra en `src/rust/`
- El HTML/JS se encuentra en `index.html`
- Los bindings WASM se generan automáticamente con `wasm-pack`
- El archivo compilado WASM se genera en `pkg/`

## 🚀 Próximas Mejoras Potenciales

- [ ] Paralelización de cálculos de fuerzas con Rayon
- [ ] Quad-tree para optimización espacial
- [ ] Exportar estadísticas de simulación
- [ ] Grabación y reproducción de simulaciones
- [ ] Más elementos de la tabla periódica

## 📄 Licencia

MIT

---

**Desarrollado con Rust 🦀 y WebAssembly 🕸️**
