# godot-gdext-demo

A minimal Godot 4 + gdext (Rust) project demonstrating the full development
loop: build, run, test, and CI.

## Prerequisites

- [Godot 4.6](https://godotengine.org/download/) on your `$PATH` as `godot`
- [Rust stable](https://rustup.rs/) + `cargo`

## Quickstart

```sh
# Compile the Rust extension and open the editor
make run-editor

# Or just run the hello scene headlessly to see "Hello from Rust!" in stdout
make run
```

## Makefile targets

| Target        | What it does                                      |
|---------------|---------------------------------------------------|
| `make build`  | `cargo build` (debug)                             |
| `make build-release` | `cargo build --release`                    |
| `make check`  | `cargo check` + `cargo clippy -D warnings`        |
| `make test`   | Pure-Rust unit tests (no Godot required)          |
| `make run`    | Build + headless scene smoke test                 |
| `make run-editor` | Build + open Godot editor                     |
| `make watch`  | Auto-rebuild on save (requires `cargo-watch`)     |
| `make clean`  | Remove `rust/target/`                             |

## Project layout

```
godot-gdext-demo/
├── rust/                   # Cargo workspace
│   ├── Cargo.toml          # workspace root
│   └── game/               # the cdylib crate
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs          # extension entry point
│           └── hello_node.rs   # example Node2D subclass
├── godot/                  # Godot project (open this in the editor)
│   ├── project.godot
│   ├── godot_gdext_demo.gdextension   # maps platforms → .so/.dll/.dylib
│   ├── scenes/
│   │   └── hello.tscn
│   └── assets/
├── .github/workflows/ci.yml
├── Makefile
└── .gitignore
```

## How the Rust ↔ Godot bridge works

The `.gdextension` file tells Godot where to find the compiled library for
each platform. When you run `cargo build`, the output goes to
`rust/target/debug/libgodot_gdext_demo.so` (Linux) — exactly where the
`.gdextension` file points. Godot loads it at startup and all `#[derive(GodotClass)]`
structs become available as native node types in the editor and in `.tscn` files.

## Testing strategy

There are two separate test layers:

- **`cargo test`** — pure Rust logic, no Godot runtime. Fast, runs anywhere.
  Put game logic (pathfinding, AI, resource systems) in plain Rust and test it here.
- **CI headless smoke test** — builds the extension, runs the hello scene with
  `--headless --quit-after 2`. Confirms the extension loads and `_ready()` fires
  without crashing. Expand this with GUT scenes as the project grows.

## Targeting iOS and Wasm

Both are experimentally supported in gdext. They're not wired up here yet —
get comfortable with the desktop loop first. When ready:

- **Wasm**: add the `experimental-wasm` feature flag in `game/Cargo.toml` and
  cross-compile to `wasm32-unknown-unknown`. Godot's Web export handles the rest.
- **iOS**: cross-compile to `aarch64-apple-ios` (requires macOS with Xcode).
  Add the iOS library paths to `godot_gdext_demo.gdextension`.
