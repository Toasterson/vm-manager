# Running in Docker/Podman

vmctl can run inside a container for CI/CD pipelines or isolated environments. The key requirement is access to `/dev/kvm`.

## Dockerfile

```dockerfile
FROM rust:1.85-bookworm AS builder

WORKDIR /build
COPY . .
RUN cargo build --release -p vmctl --features vm-manager/pure-iso

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    qemu-system-x86 \
    qemu-utils \
    openssh-client \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /build/target/release/vmctl /usr/local/bin/vmctl

ENV XDG_DATA_HOME=/data
ENTRYPOINT ["vmctl"]
```

The `pure-iso` feature eliminates the need for `genisoimage` in the container.

## Docker

```bash
docker build -t vmctl .

docker run --rm \
  --device /dev/kvm \
  -v vmctl-data:/data \
  vmctl list
```

The `--device /dev/kvm` flag passes through KVM access. No `--privileged` or special capabilities are needed for user-mode networking.

For TAP networking, you'll need `--cap-add NET_ADMIN` and appropriate bridge configuration.

## Podman

```bash
podman build -t vmctl .

podman run --rm \
  --device /dev/kvm \
  -v vmctl-data:/data \
  vmctl list
```

Podman works identically for user-mode networking.

## Persistent Data

Mount a volume at the `XDG_DATA_HOME` path (`/data` in the Dockerfile above) to persist VM state and cached images across container runs.

## Using VMFiles

Mount your project directory to use VMFile.kdl:

```bash
docker run --rm \
  --device /dev/kvm \
  -v vmctl-data:/data \
  -v $(pwd):/workspace \
  -w /workspace \
  vmctl up
```
