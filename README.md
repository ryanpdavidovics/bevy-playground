
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

# Tutorial

1. Create rust project

```sh
cargo new bevy-tutorial
```

2. Add Bevy to project

```sh
cd bevy-tutorial/
cargo add bevy
```

3. Add optimization configuration (helps with performance for testing)

Enter 
```
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```
into the end of the project's Cargo.toml file

4. Create basic app

Delete code inside main function, and change to

```rust
fn main() {
  bevy::app::App::new()
    .run();
}
```

5. Test

Run 
```sh
cargo run --target wasm32-unknown-unknown
```
And open in browser

Should see blank page. This is your empty game environment.

6. Add object

We can create entities using components, either of our own creation or pre-existing ones.
A component just needs to be a struct that derives the bevy Component trait

```rust
#[derive(bevy::ecs::component::Component)]
struct CenterMarker;
```

We can create a system that inserts an object into our game environment. In this case, we will use
MaterialMesh2dBundle to generate the object

```rust`


7. Add Camera

```rust

```