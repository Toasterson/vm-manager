# Architecture Overview

vm-manager is structured as a two-crate Cargo workspace.

## High-Level Design

```text
┌─────────────────────────────────────────┐
│                vmctl CLI                │
│         (crates/vmctl)                  │
│                                         │
│  Commands → VMFile parser → Hypervisor  │
└──────────────────┬──────────────────────┘
                   │
┌──────────────────┴──────────────────────┐
│            vm-manager library           │
│        (crates/vm-manager)              │
│                                         │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │  Hypervisor  │  │  Image Manager   │  │
│  │    Trait     │  │                  │  │
│  └──────┬──────┘  └──────────────────┘  │
│         │                               │
│  ┌──────┴──────────────────────┐        │
│  │     RouterHypervisor        │        │
│  │  ┌──────┐ ┌────────┐ ┌────┐│        │
│  │  │ QEMU │ │Propolis│ │Noop││        │
│  │  └──────┘ └────────┘ └────┘│        │
│  └─────────────────────────────┘        │
│                                         │
│  ┌───────────┐  ┌──────────────────┐    │
│  │    SSH     │  │   Cloud-Init     │    │
│  │  Module    │  │   Generator      │    │
│  └───────────┘  └──────────────────┘    │
│                                         │
│  ┌───────────┐  ┌──────────────────┐    │
│  │ Provision  │  │    VMFile        │    │
│  │  Runner    │  │    Parser        │    │
│  └───────────┘  └──────────────────┘    │
└─────────────────────────────────────────┘
```

## Async Runtime

vmctl uses Tokio with the multi-threaded runtime. Most operations are async, with one exception: SSH operations use `ssh2` (libssh2 bindings), which is blocking. These are wrapped in `tokio::task::spawn_blocking` to avoid blocking the async executor.

## Platform Abstraction

The `Hypervisor` trait defines a platform-agnostic interface. The `RouterHypervisor` dispatches calls to the correct backend based on the `BackendTag` stored in each `VmHandle`:

- **Linux** builds include `QemuBackend`.
- **illumos** builds include `PropolisBackend`.
- **All platforms** include `NoopBackend` for testing.

Conditional compilation (`#[cfg(target_os = ...)]`) ensures only the relevant backend is compiled.
