@echo off
REM Script de despliegue para Vercel (Windows)
REM Uso: deploy.bat

echo 🚀 Iniciando despliegue a Vercel...

REM Verificar que estamos en la rama correcta
for /f "tokens=*" %%i in ('git branch --show-current') do set CURRENT_BRANCH=%%i
if not "%CURRENT_BRANCH%"=="main" (
    echo ❌ Debes estar en la rama 'main'. Rama actual: %CURRENT_BRANCH%
    exit /b 1
)

REM Verificar que el directorio pkg/ existe y tiene archivos WebAssembly
if not exist "pkg" (
    echo 📦 Directorio pkg/ no encontrado, compilando WebAssembly...
    npm run build-wasm
    if errorlevel 1 (
        echo ❌ Error al compilar WebAssembly
        exit /b 1
    )
) else if not exist "pkg\atomic_particles_simulation_bg.wasm" (
    echo 📦 Archivo WebAssembly no encontrado, compilando...
    npm run build-wasm
    if errorlevel 1 (
        echo ❌ Error al compilar WebAssembly
        exit /b 1
    )
)

REM Verificar que no hay cambios sin committear
git status --porcelain >nul 2>&1
if errorlevel 1 (
    echo ⚠️  Hay cambios sin committear. Añadiéndolos...
    git add .
    git commit -m "Auto-commit before deployment"
)

REM Hacer push para activar el despliegue automático
echo 📤 Subiendo cambios a GitHub...
git push origin main

if errorlevel 1 (
    echo ❌ Error al subir cambios a GitHub
    exit /b 1
) else (
    echo ✅ Cambios subidos exitosamente a GitHub
    echo.
    echo 🌍 Vercel debería detectar los cambios automáticamente y hacer el despliegue.
    echo    Monitorea el progreso en: https://vercel.com/dashboard
    echo.
    echo 📱 Una vez completado, tu aplicación estará disponible en:
    echo    https://ia-atomic-particles.vercel.app
    echo.
    echo ⏱️  El proceso puede tomar 2-5 minutos.
)

echo 🎉 ¡Despliegue iniciado exitosamente!

pause
