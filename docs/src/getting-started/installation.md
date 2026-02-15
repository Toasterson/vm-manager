# Installation

vmctl is built from source using Rust's Cargo build system.

## Requirements

- Rust 1.85 or later (edition 2024)
- A working C compiler (for native dependencies like libssh2)

## Building from Source

Clone the repository and build the release binary:

```bash
git clone https://github.com/user/vm-manager.git
cd vm-manager
cargo build --release -p vmctl
```

The binary will be at `target/release/vmctl`. Copy it somewhere in your `$PATH`:

```bash
sudo cp target/release/vmctl /usr/local/bin/
```

## Feature Flags

The `vm-manager` library crate has one optional feature:

| Feature | Description |
|---|---|
| `pure-iso` | Use a pure-Rust ISO 9660 generator (`isobemak`) instead of shelling out to `genisoimage`/`mkisofs`. Useful in minimal or containerized environments. |

To build with it:

```bash
cargo build --release -p vmctl --features vm-manager/pure-iso
```

## Verify Installation

```bash
vmctl --help
```

You should see the list of available subcommands.
