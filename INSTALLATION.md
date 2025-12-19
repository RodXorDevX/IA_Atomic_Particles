# 🔧 Guía de Instalación y Compilación

## Paso 1: Instalar Rust (si no está instalado)

### Windows

1. Descarga el instalador desde [rustup.rs](https://rustup.rs)
2. Ejecuta el instalador y sigue las instrucciones por defecto
3. Abre una terminal nueva y verifica:

```powershell
rustc --version
cargo --version
```

### macOS/Linux

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

## Paso 2: Instalar wasm-pack

```bash
cargo install wasm-pack
```

Verifica la instalación:

```bash
wasm-pack --version
```

## Paso 3: Instalar Node.js (si no está instalado)

Descarga desde [nodejs.org](https://nodejs.org) (versión LTS recomendada)

Verifica la instalación:

```bash
node --version
npm --version
```

## Paso 4: Clonar o navegar al proyecto

```bash
cd path/to/IA_Atomic_Particles
```

## Paso 5: Instalar dependencias

```bash
npm install
```

## Paso 6: Compilar el proyecto

### Opción A: Compilación para desarrollo con hot-reload

```bash
npm run dev
```

Esto:
1. Compila el código Rust a WebAssembly
2. Inicia un servidor de desarrollo Vite en `http://localhost:5173`
3. Observa cambios en tiempo real

### Opción B: Compilación para producción

```bash
npm run build
```

Genera archivos optimizados en la carpeta `dist/`

## Paso 7: Visualizar en el navegador

- **Desarrollo**: El navegador debería abrirse automáticamente en `http://localhost:5173`
- **Producción**: Sirve los archivos desde `dist/`

```bash
npm run preview
```

## ✅ Verificación de Instalación

Si ves lo siguiente, todo está configurado correctamente:

- ✅ Canvas negro aparece en la página
- ✅ Se muestran 2 partículas iniciales (1 protón, 1 neutrón)
- ✅ Los botones de control funcionan
- ✅ La información de partículas se actualiza en tiempo real

## 🆘 Solución de Problemas

### Error: "wasm-pack: command not found"

Instala wasm-pack:

```bash
cargo install wasm-pack
```

Asegúrate de que `~/.cargo/bin` está en tu PATH.

### Error: "module not found atomic_particles_simulation"

Compila nuevamente el WASM:

```bash
npm run build-wasm
```

### El servidor Vite no inicia

Verifica que el puerto 5173 no esté en uso:

```powershell
# Windows
netstat -ano | findstr :5173

# macOS/Linux
lsof -i :5173
```

Si está en uso, mata el proceso o usa otro puerto.

### Canvas aparece negro pero sin partículas

1. Abre la consola del navegador (F12)
2. Busca mensajes de error en la pestaña "Console"
3. Verifica que el archivo `pkg/atomic_particles_simulation_bg.wasm` se cargó correctamente (pestaña "Network")

### El programa va muy lento

Asegúrate de compilar en modo release para producción:

```bash
npm run build
```

En desarrollo, el rendimiento será variable por el no-optimized WASM.

## 📦 Estructura de Archivos Generados

Después de compilar, se crea:

```
pkg/
├── atomic_particles_simulation.js           # Bindings JS
├── atomic_particles_simulation_bg.wasm      # Binario WASM compilado
├── atomic_particles_simulation_bg.wasm.d.ts # Tipos TypeScript
└── package.json
```

## 🚀 Comandos Útiles

```bash
# Compilar WASM solo
npm run build-wasm

# Desarrollo con hot-reload
npm run dev

# Compilar para producción
npm run build

# Previsualizar versión de producción
npm run preview

# Desplegar a GitHub Pages
npm run deploy

# Limpiar archivos generados
rm -rf pkg/
rm -rf dist/
```

## 💡 Tips

1. **Durante desarrollo**: Usa `npm run dev` para ver cambios en tiempo real
2. **Para depuración**: Abre las DevTools (F12) y verifica console y network
3. **Cambios en Rust**: Vite automáticamente recompilará el WASM cuando detecte cambios
4. **Build rápido**: El build de producción es significativamente más rápido que el desarrollo

## 📚 Recursos Adicionales

- [Rust Book](https://doc.rust-lang.org/book/)
- [wasm-pack Documentation](https://rustwasm.org/docs/wasm-pack/)
- [WebAssembly MDN](https://developer.mozilla.org/en-US/docs/WebAssembly/)
- [Vite Documentation](https://vitejs.dev/)

---

**¿Problemas? Revisa la consola del navegador (F12) y los mensajes de error de npm.**
