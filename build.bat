@echo off
REM Script de compilación para Atomic Particles Simulation (Windows)

echo.
echo 🚀 Iniciando compilación del proyecto Rust + WASM
echo ==================================================

REM Verificar si Rust está instalado
where cargo >nul 2>nul
if errorlevel 1 (
    echo ❌ Error: Rust no está instalado
    echo    Instálalo desde: https://rustup.rs
    exit /b 1
)

REM Verificar si wasm-pack está instalado
where wasm-pack >nul 2>nul
if errorlevel 1 (
    echo ❌ Error: wasm-pack no está instalado
    echo    Instalando con: cargo install wasm-pack
    call cargo install wasm-pack
)

REM Verificar si Node.js está instalado
where node >nul 2>nul
if errorlevel 1 (
    echo ❌ Error: Node.js no está instalado
    echo    Instálalo desde: https://nodejs.org
    exit /b 1
)

echo ✅ Herramientas verificadas
echo.

REM Limpiar builds anteriores
echo 🧹 Limpiando builds anteriores...
if exist pkg rmdir /s /q pkg
if exist dist rmdir /s /q dist
echo    ✓ Carpetas eliminadas
echo.

REM Compilar WASM
echo 🦀 Compilando Rust a WebAssembly...
call wasm-pack build src/rust --target bundler
if errorlevel 1 (
    echo    ❌ Error compilando WASM
    exit /b 1
)
echo    ✅ WASM compilado exitosamente
echo.

REM Instalar dependencias de Node
echo 📦 Instalando dependencias de Node...
call npm install
if errorlevel 1 (
    echo    ⚠️  Algunas dependencias podrían no haber sido instaladas
)
echo    ✅ Dependencias instaladas
echo.

REM Compilar para producción
echo 🏗️  Compilando para producción...
call npm run build
if errorlevel 1 (
    echo    ❌ Error en el build de producción
    exit /b 1
)
echo    ✅ Build de producción completado
echo.

echo ==================================================
echo ✨ ¡Compilación completada!
echo.
echo 📁 Archivos generados en: .\dist\
echo 🌐 Sirve los archivos y abre index.html en el navegador
echo.
echo Para desarrollo con hot-reload:
echo    npm run dev
echo.
pause
