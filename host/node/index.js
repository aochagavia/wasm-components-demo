import { readFile } from 'fs/promises';
import { instantiate as instantiateRust } from './rust-component/real-component.js';
import { instantiate as instantiatePython } from './python-component/component.js';

class Rng {
    constructor() {
        this.state = 0;
    }

    nextU32(upperBoundInclusive) {
        const next = this.state % (upperBoundInclusive + 1);
        this.state += 1;
        return next;
    }
}

class LoadedComponent {
    constructor(instance, greeter) {
        this.instance = instance;
        this.Greeter = greeter;
    }

    static async load(wasmModuleDir, instantiateFn) {
        async function getCoreModule(path) {
            const wasmBytes = await readFile(new URL('file://' + import.meta.dirname + '/' + wasmModuleDir + `/` + path));
            return await WebAssembly.compile(wasmBytes);
        }

        const instance = await instantiateFn(getCoreModule, {
            'ochagavia:test/host-types': {
                Rng: Rng
            }
        });

        const greeter = instance['ochagavia:test/guest-types@0.1.0'].Greeter;
        return new LoadedComponent(instance, greeter);
    }
}

async function main() {
    const componentPaths = [
        ['python-component', instantiatePython],
        ['rust-component', instantiateRust],
    ];

    for (const [componentPath, instantiateFn] of componentPaths) {
        console.log(`---\nTesting component: \`${componentPath}\`\n---`);

        const component = await LoadedComponent.load(componentPath, instantiateFn);
        const greeter = new component.Greeter(new Rng());

        for (let i = 0; i < 4; i++) {
            const result = greeter.greet();
            console.log(`Result: ${result}`);
        }
    }
}

main().catch(console.error);
