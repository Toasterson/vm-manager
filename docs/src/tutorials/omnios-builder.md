# Real-World: OmniOS Builder VM

This tutorial walks through the real-world VMFile.kdl used in the vm-manager project itself to build software on OmniOS (an illumos distribution).

## The Goal

Build a Rust binary (`forger`) on OmniOS. This requires:
1. An OmniOS cloud VM with development tools.
2. Uploading the source code.
3. Compiling on the guest.

## The VMFile

```kdl
vm "omnios-builder" {
    image-url "https://downloads.omnios.org/media/stable/omnios-r151056.cloud.qcow2"
    vcpus 4
    memory 4096
    disk 20

    cloud-init {
        hostname "omnios-builder"
    }

    ssh {
        user "smithy"
    }

    provision "shell" {
        script "scripts/bootstrap-omnios.sh"
    }

    provision "file" {
        source "scripts/forger-src.tar.gz"
        destination "/tmp/forger-src.tar.gz"
    }

    provision "shell" {
        script "scripts/install-forger.sh"
    }
}
```

## Stage 1: Bootstrap (`bootstrap-omnios.sh`)

This script installs system packages and the Rust toolchain:

- Sets up PATH for GNU tools (OmniOS ships BSD-style tools by default).
- Installs `gcc14`, `gnu-make`, `pkg-config`, `openssl`, `curl`, `git`, and other build dependencies via IPS (`pkg install`).
- Installs Rust via `rustup`.
- Verifies all tools are available.

## Stage 2: Upload Source

The `file` provisioner uploads a pre-packed tarball of the forger source code. This tarball is created beforehand with:

```bash
./scripts/pack-forger.sh
```

The pack script:
- Copies `crates/forger`, `crates/spec-parser`, and `images/` into a staging directory.
- Generates a minimal workspace `Cargo.toml`.
- Includes `Cargo.lock` for reproducible builds.
- Creates `scripts/forger-src.tar.gz`.

## Stage 3: Build and Install (`install-forger.sh`)

- Extracts the tarball to `$HOME/forger`.
- Runs `cargo build -p forger --release`.
- Copies the binary to `/usr/local/bin/forger`.

## The Full Workflow

```bash
# Pack the source on the host
./scripts/pack-forger.sh

# Bring up the VM, provision, and build
vmctl up

# SSH in to test the binary
vmctl ssh
forger --help

# Tear it down when done
vmctl down --destroy
```

## Key Takeaways

- **Multi-stage provisioning** separates concerns: system setup, source upload, build.
- **File provisioners** transfer artifacts to the guest.
- **Script provisioners** are easier to iterate on than inline commands for complex logic.
- **Streaming output** lets you watch the build progress in real-time.
