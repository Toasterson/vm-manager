# Full Example

A complete VMFile.kdl demonstrating every available feature:

```kdl
// Development VM with all options specified
vm "full-example" {
    // Image source: URL (auto-cached)
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"

    // Resources
    vcpus 4
    memory 4096
    disk 40

    // Networking: user-mode (default, no root needed)
    network "user"

    // Cloud-init guest configuration
    cloud-init {
        hostname "full-example"
        // ssh-key and user-data are omitted, so vmctl auto-generates an Ed25519 keypair
    }

    // SSH connection settings
    ssh {
        user "ubuntu"
        // private-key is omitted, so vmctl uses the auto-generated key
    }

    // Provisioners run in order after boot
    provision "shell" {
        inline "sudo apt-get update && sudo apt-get install -y build-essential curl git"
    }

    provision "file" {
        source "config/bashrc"
        destination "/home/ubuntu/.bashrc"
    }

    provision "shell" {
        script "scripts/setup-dev-tools.sh"
    }
}

// Second VM demonstrating TAP networking and explicit keys
vm "tap-example" {
    image "~/images/debian-12-generic-amd64.qcow2"

    vcpus 2
    memory 2048

    network "tap" {
        bridge "br0"
    }

    cloud-init {
        hostname "tap-vm"
        ssh-key "~/.ssh/id_ed25519.pub"
    }

    ssh {
        user "debian"
        private-key "~/.ssh/id_ed25519"
    }
}

// Minimal VM: just an image and defaults
vm "minimal" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"
}
```

## What Happens

Running `vmctl up` with this VMFile:

1. **full-example**: Downloads the Ubuntu image, creates a 40GB overlay, auto-generates SSH keys, boots with 4 vCPUs / 4GB RAM, runs three provisioners.
2. **tap-example**: Uses a local Debian image, sets up TAP networking on `br0`, injects your existing SSH key.
3. **minimal**: Downloads the same Ubuntu image (cache hit), boots with defaults (1 vCPU, 1GB RAM, user networking), no cloud-init, no provisioning.

Use `--name` to target specific VMs:

```bash
vmctl up --name full-example
vmctl ssh --name tap-example
vmctl down --name minimal
```
