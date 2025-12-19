# 🚀 Despliegue en Vercel - Simulación de Núcleos Atómicos

Esta guía explica cómo desplegar la aplicación de simulación de partículas atómicas en Vercel.

## 📋 Prerrequisitos

- ✅ **Cuenta en Vercel** ([vercel.com](https://vercel.com))
- ✅ **GitHub/GitLab/Bitbucket** (repositorio conectado)
- ✅ **Rust 1.70+** (para compilación local)
- ✅ **wasm-pack** (para compilar WebAssembly)

## 🛠️ Configuración del Proyecto

### Archivos de Configuración Creados

#### `vercel.json`
```json
{
  "version": 2,
  "builds": [
    {
      "src": "package.json",
      "use": "@vercel/static-build",
      "config": {
        "distDir": "."
      }
    }
  ],
  "routes": [
    {
      "src": "/pkg/(.*)",
      "dest": "/pkg/$1",
      "headers": {
        "Cache-Control": "public, max-age=31536000, immutable"
      }
    },
    {
      "src": "/(.*)",
      "dest": "/$1"
    }
  ],
  "headers": [
    {
      "source": "/pkg/(.*)",
      "headers": [
        {
          "key": "Cache-Control",
          "value": "public, max-age=31536000, immutable"
        },
        {
          "key": "Content-Type",
          "value": "application/wasm"
        }
      ]
    }
  ]
}
```

#### `package.json` Actualizado
```json
{
  "scripts": {
    "build-wasm": "wasm-pack build . --target web",
    "vercel-build": "npm run build-wasm"
  },
  "devDependencies": {
    "wasm-pack": "^0.12.1"
  }
}
```

## 🚀 Proceso de Despliegue

### Opción 1: Despliegue Automático (Recomendado)

1. **Sube tu código a GitHub/GitLab/Bitbucket**
   ```bash
   git add .
   git commit -m "Ready for Vercel deployment"
   git push origin main
   ```

2. **Conecta con Vercel**
   - Ve a [vercel.com](https://vercel.com)
   - Haz clic en "New Project"
   - Importa tu repositorio
   - Vercel detectará automáticamente la configuración

3. **Configuración en Vercel**
   - **Framework Preset**: `Other`
   - **Root Directory**: `./` (raíz del proyecto)
   - **Build Command**: `npm run vercel-build`
   - **Output Directory**: `./`

### Opción 2: Despliegue Manual

1. **Compila WebAssembly localmente**
   ```bash
   npm install
   npm run build-wasm
   ```

2. **Verifica que `pkg/` contenga los archivos**
   ```
   pkg/
   ├── atomic_particles_simulation.js
   ├── atomic_particles_simulation_bg.wasm
   └── ...
   ```

3. **Despliega con Vercel CLI**
   ```bash
   npm install -g vercel
   vercel --prod
   ```

## ⚙️ Configuración de Build en Vercel

### Build Settings
- **Build Command**: `npm run vercel-build`
- **Output Directory**: `./`
- **Install Command**: `npm install`

### Environment Variables (si es necesario)
No se requieren variables de entorno especiales para esta aplicación.

## 🔍 Verificación del Despliegue

### Archivos que deben estar disponibles:
- ✅ `index.html` (página principal)
- ✅ `pkg/atomic_particles_simulation.js` (bindings JS)
- ✅ `pkg/atomic_particles_simulation_bg.wasm` (WebAssembly)
- ✅ `pkg/atomic_particles_simulation.d.ts` (TypeScript definitions)

### URLs de ejemplo:
- `https://tu-app.vercel.app/` → Página principal
- `https://tu-app.vercel.app/pkg/atomic_particles_simulation_bg.wasm` → WebAssembly

## 🐛 Solución de Problemas

### Error: "WebAssembly module not found"
```bash
# Verifica que los archivos pkg/ estén incluidos
ls -la pkg/

# Recompila WebAssembly
npm run build-wasm
```

### Error: "Build failed"
```bash
# Instala dependencias
npm install

# Verifica Rust
rustc --version
wasm-pack --version
```

### Error: "404 on WebAssembly files"
- Verifica que `vercel.json` esté en la raíz
- Asegúrate de que `pkg/` no esté en `.gitignore`

## 🎯 Optimizaciones para Vercel

### Cache de WebAssembly
Los archivos WebAssembly se sirven con cache agresivo (1 año) para máxima performance.

### Headers de Seguridad
Vercel añade automáticamente headers de seguridad recomendados.

### CDN Global
La aplicación se distribuye automáticamente en la CDN global de Vercel.

## 📊 Monitoreo

### Vercel Analytics
- Activa Vercel Analytics para métricas de uso
- Monitorea performance y errores

### Logs de Build
- Revisa logs de build en el dashboard de Vercel
- Busca errores de compilación de WebAssembly

## 🔄 Actualizaciones

### Para actualizar la aplicación:
1. Haz cambios en el código
2. Compila WebAssembly: `npm run build-wasm`
3. Sube cambios: `git push`
4. Vercel redeploy automáticamente

## 🌐 URLs Personalizadas

### Configurar dominio personalizado:
1. Ve a Settings → Domains
2. Añade tu dominio
3. Configura DNS según las instrucciones

## 📞 Soporte

- **Documentación Vercel**: [vercel.com/docs](https://vercel.com/docs)
- **WebAssembly en Vercel**: Busca guías específicas
- **Issues del proyecto**: Reporta problemas en el repositorio

---

## ✅ Checklist de Despliegue

- [ ] Código subido a repositorio Git
- [ ] `vercel.json` configurado correctamente
- [ ] `package.json` con script `vercel-build`
- [ ] `pkg/` no ignorado en `.gitignore`
- [ ] WebAssembly compilado localmente
- [ ] Proyecto importado en Vercel
- [ ] Build exitoso en Vercel
- [ ] Aplicación funcionando en URL de Vercel

---

**¡Tu simulación de partículas atómicas está lista para el mundo! 🌍⚛️**
