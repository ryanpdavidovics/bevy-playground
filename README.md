
# Installing Rust

The best way to obtain the rust toolchain is to install rustup
Per https://rustup.rs/:
run:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
````

# Preparing project

Add Wasm compilation target using rustup:

```sh
rustup target add wasm32-unknown-unknown
```

Requires wasm-server-runner to easily run project in browser
https://bevy-cheatbook.github.io/platforms/wasm.html
https://github.com/jakobhellermann/wasm-server-runner

```sh
cargo install wasm-server-runner
```

Ensure these lines are entered in the ~/.cargo/config.toml file

```toml
[target.wasm32-unknown-unknown]
runner = "wasm-server-runner"
```

# Running

Compile and run project by entering

```sh
cargo run --target wasm32-unknown-unknown
```
And opening the provided IP in the browser