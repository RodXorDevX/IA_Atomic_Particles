# ⚡ Quick Start Guide

## 🚀 En 5 Minutos

### 1️⃣ Instalar Herramientas (Una sola vez)

```bash
# Windows (desde PowerShell como administrador)
rustup-init.exe          # Descargar de https://rustup.rs
cargo install wasm-pack
# Instalar Node.js desde https://nodejs.org

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack
# Instalar Node.js desde https://nodejs.org
```

### 2️⃣ Navegar al Proyecto

```bash
cd path/to/IA_Atomic_Particles
```

### 3️⃣ Instalar Dependencias

```bash
npm install
```

### 4️⃣ Compilar y Ejecutar

**Windows:**
```bash
.\build.bat
```

**macOS/Linux:**
```bash
bash build.sh
```

**O manualmente:**
```bash
npm run dev
```

### 5️⃣ Abrir en Navegador

```
http://localhost:5173
```

---

## 📋 Comandos Principales

```bash
# Compilar WASM
npm run build-wasm

# Desarrollo con hot-reload
npm run dev

# Compilación producción
npm run build

# Previsualizar
npm run preview

# Desplegar
npm run deploy
```

---

## 🐛 Si Algo Falla

### "wasm-pack not found"
```bash
cargo install wasm-pack
```

### "module not found atomic_particles_simulation"
```bash
npm run build-wasm
npm install
```

### Canvas negro sin partículas
1. Abre DevTools (F12)
2. Verifica console (no debe haber errores)
3. Verifica Network > verifica que `.wasm` se cargó
4. Actualiza la página

### Node_modules corrupto
```bash
rm -r node_modules package-lock.json
npm install
```

---

## 📚 Documentación Completa

- **README_RUST.md** - Guía completa
- **INSTALLATION.md** - Instalación detallada
- **MIGRATION.md** - Detalles técnicos
- **COMPLETION_SUMMARY.md** - Resumen del proyecto

---

## ✅ Verificar Instalación

```bash
# Verificar Rust
rustc --version    # Debe mostrar versión

# Verificar wasm-pack
wasm-pack --version

# Verificar Node
node --version
npm --version
```

---

## 💡 Tips

- **Primera compilación es lenta** (~20-30s), después es más rápido
- **Usa `npm run dev`** durante desarrollo
- **Usa `npm run build`** para producción optimizada
- **F12** abre DevTools para debugging

---

## 🎯 Próximo Paso

¡Abre el navegador en `http://localhost:5173` y disfruta la simulación!

**¿Dudas?** Revisa los archivos `.md` documentación completa.
