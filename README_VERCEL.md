# 🚀 Despliegue Rápido en Vercel

## Problema Solucionado
El deployment no funcionaba porque Vercel no detectaba correctamente la configuración para aplicaciones WebAssembly.

## ✅ Solución Implementada

### 1. **Archivo `.env.production`**
```bash
# Variables para producción
TARGET_FPS=60
MAX_CANVAS_WIDTH=1920
MAX_CANVAS_HEIGHT=1080
VERCEL_ANALYTICS=true
DEBUG_MODE=false
SHOW_PERFORMANCE=false
```

### 2. **Configuración `vercel.json` corregida**
```json
{
  "version": 2,
  "framework": null,
  "buildCommand": "npm run vercel-build",
  "outputDirectory": ".",
  "installCommand": "npm install"
}
```

### 3. **Archivos incluidos en el repositorio**
- ✅ `pkg/` - Archivos WebAssembly compilados
- ✅ `.env.production` - Variables de entorno
- ✅ `vercel.json` - Configuración de build

## 🎯 Pasos para Activar el Deployment

### **Opción 1: Dashboard de Vercel**
1. Ve a [vercel.com/dashboard](https://vercel.com/dashboard)
2. Selecciona tu proyecto `ia-atomic-particles`
3. Ve a **Settings → Environment Variables**
4. **Importa el archivo `.env.production`** (botón "Import")
5. **Redeploy** manualmente

### **Opción 2: Línea de comandos**
```bash
# Instalar Vercel CLI
npm install -g vercel

# Login
vercel login

# Importar variables de entorno
vercel env pull .env.production

# Redeploy
vercel --prod
```

## 🔍 Verificación

Después del redeployment, verifica:
- ✅ `https://ia-atomic-particles.vercel.app/` funciona
- ✅ La simulación de partículas carga correctamente
- ✅ Los archivos WebAssembly se sirven (`/pkg/*.wasm`)

## 🐛 Si aún no funciona

### **Forzar redeployment:**
1. Ve al dashboard de Vercel
2. **Deployments** → Último deployment
3. **Redeploy** (botón)

### **Verificar logs:**
- Revisa los logs de build en Vercel
- Busca errores de WebAssembly
- Confirma que `npm run vercel-build` funciona

## 📞 Variables de Entorno Necesarias

| Variable | Valor | Descripción |
|----------|-------|-------------|
| `TARGET_FPS` | `60` | FPS objetivo |
| `MAX_CANVAS_WIDTH` | `1920` | Ancho máximo canvas |
| `MAX_CANVAS_HEIGHT` | `1080` | Alto máximo canvas |
| `VERCEL_ANALYTICS` | `true` | Activar analytics |
| `DEBUG_MODE` | `false` | Modo debug (desactivado) |
| `SHOW_PERFORMANCE` | `false` | Mostrar métricas |

## 🎉 ¡Listo!

Una vez configurado, tu aplicación estará disponible en:
**https://ia-atomic-particles.vercel.app/**

La simulación de núcleos atómicos con WebAssembly estará funcionando en la CDN global de Vercel.
