
#### `src/particles.js`
```javascript
// src/particles.js

class Quark {
    constructor(type) {
        this.type = type; // 'up' o 'down'
    }
}

class Proton {
    constructor() {
        this.quarks = [new Quark('up'), new Quark('up'), new Quark('down')];
    }
}

class Neutron {
    constructor() {
        this.quarks = [new Quark('up'), new Quark('down'), new Quark('down')];
    }
}

export { Proton, Neutron };
```
