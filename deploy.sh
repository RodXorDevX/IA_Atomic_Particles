#!/bin/bash

# Script de despliegue para Vercel
# Uso: ./deploy.sh

echo "🚀 Iniciando despliegue a Vercel..."

# Verificar que estamos en la rama correcta
CURRENT_BRANCH=$(git branch --show-current)
if [ "$CURRENT_BRANCH" != "main" ]; then
    echo "❌ Debes estar en la rama 'main'. Rama actual: $CURRENT_BRANCH"
    exit 1
fi

# Verificar que el directorio pkg/ existe y tiene archivos WebAssembly
if [ ! -d "pkg" ] || [ ! -f "pkg/atomic_particles_simulation_bg.wasm" ]; then
    echo "📦 Compilando WebAssembly..."
    npm run build-wasm
    if [ $? -ne 0 ]; then
        echo "❌ Error al compilar WebAssembly"
        exit 1
    fi
fi

# Verificar que no hay cambios sin committear
if [ -n "$(git status --porcelain)" ]; then
    echo "⚠️  Hay cambios sin committear. Añadiéndolos..."
    git add .
    git commit -m "Auto-commit before deployment"
fi

# Hacer push para activar el despliegue automático
echo "📤 Subiendo cambios a GitHub..."
git push origin main

if [ $? -eq 0 ]; then
    echo "✅ Cambios subidos exitosamente a GitHub"
    echo ""
    echo "🌍 Vercel debería detectar los cambios automáticamente y hacer el despliegue."
    echo "   Monitorea el progreso en: https://vercel.com/dashboard"
    echo ""
    echo "📱 Una vez completado, tu aplicación estará disponible en:"
    echo "   https://ia-atomic-particles.vercel.app"
    echo ""
    echo "⏱️  El proceso puede tomar 2-5 minutos."
else
    echo "❌ Error al subir cambios a GitHub"
    exit 1
fi

echo "🎉 ¡Despliegue iniciado exitosamente!"
