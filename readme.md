# Wasm components demo

## Compile the components

#### 1. Python

Assuming you have `uv` installed, it should be enough to run the following command from `component/python`:

```bash
uvx componentize-py --wit-path ../../wit/greeter.wit --world my-world componentize --stub-wasi component -o component.wasm
```

#### 2. Rust

Requirements:

- `wasm-tools` (available through `cargo install --locked wasm-tools`)
- Wasm core compilation target (available through `rustup target add wasm32-unknown-unknown`)

With all that in place, it should be enough to run the following commands from `component/rust`:

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/release/component.wasm -o real-component.wasm
```

## Run the host applications

Note: this assumes you have built the components, as explained above.

#### 1. Rust

Just run `cargo run --release` from `host/rust`.

#### 2. Node

For completeness, there's also a Node host, which is a "port" of the Rust one.

Node does not natively support components, but we can easily run them anyway thanks to `jco`, a tool that converts components to plain WebAssembly modules and some glue code. Note that the Wasm component is _not_ sandboxed in this case!

Run the following commands from `host/node` to set things up:

```bash
npm install
npx jco transpile ../../component/rust/real-component.wasm --out-dir rust-component --instantiation async
npx jco transpile ../../component/python/component.wasm --out-dir python-component --instantiation async
```

Now you can run the actual program:

```bash
node index.js
```

## Development

Regenerate the wasm bindings:

- Ensure `wit-bindgen-cli` is installed (available through `cargo install --locked wit-bindgen-cli`)
- `wit-bindgen rust wit/greeter.wit`
