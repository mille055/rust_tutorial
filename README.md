# Rust Learning Lab 🦀

A personal Rust playground where I can experiment with the language from the ground up:

- basic syntax, ownership, and borrowing
- references vs `mut` vs `&mut`
- slices and arrays
- (eventually) ML and data tooling in Rust

The idea is to keep **each concept in its own small binary** under `src/bin/`, so I can run any example with a single `cargo run` command.

---

## Prerequisites

You’ll want the standard Rust toolchain:

- `rustc` – Rust compiler  
- `cargo` – package manager & build tool  
- `rustfmt` – formatter  
- `clippy` – linter  
- `rustup` – toolchain manager  

You can check everything with:

```bash
make rust-version
```

(That target just prints the versions of all the Rust tools.)

If you don’t have Rust installed yet, use:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## Project Structure

Rough layout:

```text
rust-learning-lab/
  Cargo.toml
  Makefile
  README.md
  .gitignore
  .devcontainer/
    Dockerfile          # Rust dev image
  .github/
    workflows/
      test.yml

  src/
    main.rs             # small "hub" entrypoint
    bin/
      slices_01_immutable.rs
      slices_02_mutable.rs
      refs_01_mut_vs_binding.rs
      # (add more tutorial files here over time)
```

### `src/main.rs`

This can just act as a tiny hub/reminder:

```rust
fn main() {
    println!("Rust learning lab ✨");
    println!("Run a specific example with:");
    println!("  cargo run --bin slices_01_immutable");
    println!("  cargo run --bin slices_02_mutable");
    println!("  cargo run --bin refs_01_mut_vs_binding");
}
```

---

## Running Examples

Each tutorial/example lives in `src/bin/` as its own mini-program.

For example, if you have:

- `src/bin/slices_01_immutable.rs`
- `src/bin/slices_02_mutable.rs`
- `src/bin/refs_01_mut_vs_binding.rs`

You can run them with:

```bash
cargo run --bin slices_01_immutable
cargo run --bin slices_02_mutable
cargo run --bin refs_01_mut_vs_binding
```

This lets you keep many small, focused examples in a single repo.

---

## Makefile Commands

The `Makefile` provides some convenience targets:

```make
rust-version:
	@echo "Rust command-line utility versions:"
	rustc --version 			# rust compiler
	cargo --version 			# rust package manager
	rustfmt --version			# rust code formatter
	rustup --version			# rust toolchain manager
	clippy-driver --version		# rust linter

format:
	cargo fmt --quiet

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

release:
	cargo build --release

all: format lint test run
```

Typical workflows:

- Format code:

  ```bash
  make format
  ```

- Lint:

  ```bash
  make lint
  ```

- Run tests:

  ```bash
  make test
  ```

- Run the default binary (`src/main.rs`):

  ```bash
  make run
  ```

- Build a release binary:

  ```bash
  make release
  ```

- Do everything (format → lint → test → run):

  ```bash
  make all
  ```

---

## GitHub Actions (CI)

There is a simple CI workflow under `.github/workflows/test.yml`:

```yaml
name: Tests

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v1
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          profile: minimal
          components: clippy, rustfmt
          override: true
      - name: Run tests
        run: make test
```

You can later expand this to:

```yaml
run: make format lint test
```

if you want CI to enforce formatting and clippy as well.

---

## Dev Container

The repo can include a dev container image based on the official Rust dev container, for example:

```Dockerfile
FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y
```

If you open this repo in VS Code with the Dev Containers extension, you’ll get a ready-to-go Rust environment with a fast linker.

---

## Adding New “Lessons”

The idea is to grow this repo as a living tutorial.

To add a new mini-lesson:

1. Create a new file in `src/bin/`, e.g.:

   ```bash
   touch src/bin/ownership_01_moves.rs
   ```

2. Add a `main`:

   ```rust
   fn main() {
       println!("Ownership: basic moves");
       // experiment here...
   }
   ```

3. Run it:

   ```bash
   cargo run --bin ownership_01_moves
   ```

Suggested future files:

- `ownership_01_moves.rs`
- `ownership_02_borrowing.rs`
- `lifetimes_01_intro.rs`
- `collections_01_vec.rs`
- `ml_01_ndarray_basics.rs`
- `ml_02_linreg.rs`

---

## Notes to Future Me

- When I’m confused about `mut` vs `&mut`, remember:
  - `&mut T` → I can change the value through this reference.
  - `mut x: &T` → I can reassign `x` to point to a different `&T`, but not modify the pointee.

- When I’m confused about slices:
  - Python slice: **copy** into a new list.  
  - Rust slice (`&a[start..end]`): **borrowed window** into the same data (pointer + length).

This repo is meant to be a safe place to poke at those ideas repeatedly.
