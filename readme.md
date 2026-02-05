# Wasm components demo

## Compile the components

#### 1. Python

Assuming you have `uv` installed, it should be enough to run the following command from `component/python`:

```bash
uvx componentize-py --wit-path ../../wit/greeter.wit --world my-world componentize --stub-wasi component -o component.wasm
```

#### 2. Rust

Requirements:

- `wit-bindgen-cli` (available through `cargo install --locked wit-bindgen-cli`)
- `wasm-tools` (available through `cargo install --locked wasm-tools`)
- Wasm core compilation target (available through `rustup target add wasm32-unknown-unknown`)

With all that in place, it should be enough to run the following commands from `component/rust`:

```bash
cargo build --release --target wasm32-unknown-unknown
wasm-tools component new ./target/wasm32-unknown-unknown/release/component.wasm -o real-component.wasm
```

## Run the host application

If you have built the components above, you can run the host application from the `host` directory, using `cargo run --release`.

## Development

Regenerate the wasm bindings:

- `wit-bindgen rust wit/greeter.wit`
