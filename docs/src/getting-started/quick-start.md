# Quick Start

This guide walks you through creating your first VM in under a minute.

## Imperative (One-Off)

Create and start a VM from an Ubuntu cloud image:

```bash
vmctl create \
  --name demo \
  --image-url https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img \
  --vcpus 2 \
  --memory 2048 \
  --start
```

Wait a moment for the image to download and the VM to boot, then connect:

```bash
vmctl ssh demo
```

When you're done:

```bash
vmctl destroy demo
```

## Declarative (Reproducible)

Create a `VMFile.kdl` in your project directory:

```kdl
vm "demo" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"
    vcpus 2
    memory 2048

    cloud-init {
        hostname "demo"
    }

    ssh {
        user "ubuntu"
    }
}
```

Bring it up:

```bash
vmctl up
```

vmctl will download the image (cached for future use), create a QCOW2 overlay, generate an Ed25519 SSH keypair, build a cloud-init ISO, and boot the VM.

Connect:

```bash
vmctl ssh
```

Tear it down:

```bash
vmctl down
```

Or destroy it completely (removes all VM files):

```bash
vmctl down --destroy
```

## Next Steps

- [Concepts: How vmctl Works](../concepts/how-it-works.md) for an understanding of what happens under the hood.
- [Tutorials: Declarative Workflow](../tutorials/declarative-workflow.md) for a complete walkthrough with provisioning.
- [VMFile.kdl Reference](../vmfile/overview.md) for the full configuration format.
