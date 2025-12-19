#!/bin/bash
# Script de compilación para Atomic Particles Simulation

echo "🚀 Iniciando compilación del proyecto Rust + WASM"
echo "=================================================="

# Verificar si Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Error: Rust no está instalado"
    echo "   Instálalo desde: https://rustup.rs"
    exit 1
fi

# Verificar si wasm-pack está instalado
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ Error: wasm-pack no está instalado"
    echo "   Instalando con: cargo install wasm-pack"
    cargo install wasm-pack
fi

# Verificar si Node.js está instalado
if ! command -v node &> /dev/null; then
    echo "❌ Error: Node.js no está instalado"
    echo "   Instálalo desde: https://nodejs.org"
    exit 1
fi

echo "✅ Herramientas verificadas"
echo ""

# Limpiar builds anteriores
echo "🧹 Limpiando builds anteriores..."
rm -rf pkg/ dist/
echo "   ✓ Carpetas eliminadas"
echo ""

# Compilar WASM
echo "🦀 Compilando Rust a WebAssembly..."
wasm-pack build src/rust --target bundler

if [ $? -eq 0 ]; then
    echo "   ✅ WASM compilado exitosamente"
else
    echo "   ❌ Error compilando WASM"
    exit 1
fi
echo ""

# Instalar dependencias de Node
echo "📦 Instalando dependencias de Node..."
npm install
if [ $? -eq 0 ]; then
    echo "   ✅ Dependencias instaladas"
else
    echo "   ⚠️  Algunas dependencias podrían no haber sido instaladas"
fi
echo ""

# Compilar para producción
echo "🏗️  Compilando para producción..."
npm run build
if [ $? -eq 0 ]; then
    echo "   ✅ Build de producción completado"
else
    echo "   ❌ Error en el build de producción"
    exit 1
fi
echo ""

echo "=================================================="
echo "✨ ¡Compilación completada!"
echo ""
echo "📁 Archivos generados en: ./dist/"
echo "🌐 Sirve los archivos y abre index.html en el navegador"
echo ""
echo "Para desarrollo con hot-reload:"
echo "   npm run dev"
echo ""
