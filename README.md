# ⚛️ Simulación de Núcleos Atómicos - Física Cuántica Interactiva

[![Vercel Deployment](https://img.shields.io/badge/Vercel-Deployed-success)](https://ia-atomic-particles.vercel.app/)
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=white)](https://webassembly.org/)

> **🔗 [Ver Demo en Vivo](https://ia-atomic-particles.vercel.app/)**

Una simulación interactiva avanzada de física nuclear y atómica que combina **Rust**, **WebAssembly** y **JavaScript** para crear una experiencia educativa inmersiva. Explora la formación de núcleos atómicos, fuerzas fundamentales y la tabla periódica completa.

## 🌟 Características Principales

### ⚛️ **Simulación Nuclear Avanzada**
- **118 elementos químicos** con propiedades físicas reales
- **Física nuclear precisa** (fuerzas nucleares fuerte, débil y electromagnética)
- **Formación de núcleos** a partir de protones y neutrones
- **Detección automática** de elementos químicos formados

### 🎮 **Controles Interactivos**
- **Tabla periódica completa** con elementos clickeables
- **Sliders en tiempo real** para ajustar parámetros físicos
- **Botones de partículas** (protones, neutrones, electrones)
- **Controles de velocidad** y animación
- **Configuraciones guardables** (localStorage)

### 🎨 **Interfaz Moderna**
- **Canvas WebAssembly** de alto rendimiento
- **Interfaz responsive** adaptable a cualquier dispositivo
- **Visualización de orbitales** electrónicos
- **Indicadores de estado** en tiempo real

## 🛠️ Tecnologías Utilizadas

- **🦀 Rust** - Lógica de simulación de alto rendimiento
- **🕸️ WebAssembly** - Ejecución nativa en el navegador
- **⚛️ JavaScript/TypeScript** - Interfaz de usuario
- **🎨 HTML5 Canvas** - Renderizado gráfico
- **🚀 Vercel** - Deployment y hosting global

## 📁 Estructura del Proyecto

```
IA_Atomic_Particles/
├── 📁 src/
│   ├── lib.rs              # Punto de entrada WebAssembly
│   ├── simulation.rs       # Lógica de simulación principal
│   ├── particle.rs         # Definiciones de partículas
│   ├── physics.rs          # Cálculos físicos
│   └── utils.rs            # Utilidades
├── 📁 pkg/                 # Archivos WebAssembly compilados
├── 📄 index.html           # Interfaz principal
├── 📄 package.json         # Dependencias Node.js
├── 📄 Cargo.toml          # Dependencias Rust
├── 📄 vercel.json         # Configuración de deployment
└── 📄 README.md           # Esta documentación
```

## 🚀 Instalación y Uso

### **Opción 1: Demo en Vivo (Recomendado)**
👉 **[https://ia-atomic-particles.vercel.app/](https://ia-atomic-particles.vercel.app/)**

### **Opción 2: Instalación Local**

#### Prerrequisitos
- **Rust** (1.70+): [rustup.rs](https://rustup.rs/)
- **Node.js** (18+): [nodejs.org](https://nodejs.org/)
- **wasm-pack**: `cargo install wasm-pack`

#### Instalación
```bash
# Clona el repositorio
git clone https://github.com/RodXorDevX/IA_Atomic_Particles.git
cd IA_Atomic_Particles

# Instala dependencias
npm install

# Compila WebAssembly y ejecuta
npm run dev
```

## 🎯 Cómo Usar la Simulación

### **1. Explorar la Tabla Periódica**
- **Haz clic** en cualquier elemento químico
- **Observa** cómo se forman los átomos automáticamente
- **Experimenta** con diferentes combinaciones

### **2. Controles de Física**
- **Fuerza Nuclear**: Ajusta la intensidad de la fuerza fuerte
- **Masa Nuclear**: Cambia las masas de protones/neutrones
- **Velocidad Orbital**: Controla la velocidad de electrones
- **Repulsión**: Ajusta fuerzas electromagnéticas

### **3. Añadir Partículas Manualmente**
- **+ Protón**: Añade protones individuales
- **+ Neutrón**: Añade neutrones individuales
- **+ Electrón**: Añade electrones individuales
- **Elementos**: Haz clic en la tabla periódica

### **4. Configuraciones Avanzadas**
- **Guardar/Cargar**: Preserva tus configuraciones favoritas
- **Velocidad**: Controla la velocidad de simulación (0.1x - 20x)
- **Pausar/Reanudar**: Control temporal de la simulación

## 🔬 Física Implementada

### **Fuerzas Fundamentales**
- **Fuerza Nuclear Fuerte**: Une protones y neutrones
- **Fuerza Electromagnética**: Repulsión protón-protón
- **Fuerza Nuclear Débil**: Desintegración radiactiva
- **Fuerza Gravitacional**: Efectos a escala atómica

### **Modelos Cuánticos**
- **Orbitales Atómicos**: s, p, d, f
- **Números Cuánticos**: n, l, m_l, m_s
- **Estructura Nuclear**: Capas y subcapas
- **Isótopos**: Variaciones en neutrones

## 📊 Rendimiento

- **WebAssembly**: Ejecución nativa ~20x más rápida que JavaScript
- **Canvas Optimizado**: 60 FPS con miles de partículas
- **Memoria Eficiente**: Gestión automática de recursos
- **Compilación AOT**: Sin sobrecarga de runtime

## 🤝 Contribuir

¡Las contribuciones son bienvenidas! Para contribuir:

1. **Fork** el repositorio
2. **Crea** una rama para tu feature (`git checkout -b feature/nueva-funcionalidad`)
3. **Commit** tus cambios (`git commit -am 'Añade nueva funcionalidad'`)
4. **Push** a la rama (`git push origin feature/nueva-funcionalidad`)
5. **Abre** un Pull Request

### **Áreas de Mejora**
- Más elementos químicos
- Física de partículas avanzada
- Visualizaciones 3D
- Modo multijugador
- Integración con bases de datos científicas

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver el archivo `LICENSE` para más detalles.

## 🙏 Agradecimientos

- **Rust Community** por el excelente ecosistema WebAssembly
- **Vercel** por el hosting gratuito y confiable
- **Mozilla** por las herramientas WebAssembly
- **Comunidad Open Source** por las bibliotecas utilizadas

---

**⭐ Si te gusta este proyecto, ¡dale una estrella en GitHub!**

**🔗 [Ver Demo Interactiva](https://ia-atomic-particles.vercel.app/)**
