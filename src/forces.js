
#### `src/forces.js`
```javascript
// src/forces.js

class StrongForce {
    apply() {
        console.log("Aplicando fuerza de interacción fuerte.");
    }
}

class WeakForce {
    apply() {
        console.log("Aplicando fuerza de interacción débil.");
    }
}

class ElectromagneticForce {
    apply() {
        console.log("Aplicando fuerza electromagnética.");
    }
}

export { StrongForce, WeakForce, ElectromagneticForce };
```
