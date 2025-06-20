# Nimbus

**Nimbus** is the official package manager and build system for the [Cirrus programming language](https://github.com/cirrus-lang/cirrus),  aiming to be the all-in-one solution for managing Cirrus projects.

## Features (Planned)

- `nimbus new` — Scaffold a new Cirrus project
- `nimbus build` — Compile a Cirrus project using your compiler
- `nimbus run` — Build and run the project in one step
- Future support for:
  - Package/dependency management
  - Workspaces and project configuration
  - Compiler and toolchain integration

## Status

🛠️ **Nimbus is currently under active development.**  
Only basic commands (`new`, `build`, `run`) are planned for the initial version.  
More advanced features will be added progressively as the Cirrus ecosystem grows.

## Getting Started

```bash
# Clone and build
git clone https://github.com/cirrus-lang/nimbus
cd nimbus
cargo build --release

# Create a new project
./target/release/nimbus new my_project
```

## License

MIT © [Cirrus Project](https://github.com/cirrus-lang)
