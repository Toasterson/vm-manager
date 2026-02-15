# Declarative Workflow with VMFile.kdl

This tutorial shows how to define VMs in a configuration file and manage them with `vmctl up`/`down`.

## Write a VMFile

Create `VMFile.kdl` in your project directory:

```kdl
vm "webserver" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"
    vcpus 2
    memory 2048
    disk 20

    cloud-init {
        hostname "webserver"
    }

    ssh {
        user "ubuntu"
    }

    provision "shell" {
        inline "sudo apt-get update && sudo apt-get install -y nginx"
    }

    provision "shell" {
        inline "echo 'Hello from vmctl!' | sudo tee /var/www/html/index.html"
    }
}
```

## Bring It Up

```bash
vmctl up
```

vmctl will:
1. Discover `VMFile.kdl` in the current directory.
2. Download the Ubuntu image (or use the cached copy).
3. Generate an Ed25519 SSH keypair for this VM.
4. Create a QCOW2 overlay with 20GB disk.
5. Build a cloud-init ISO with the hostname and generated SSH key.
6. Boot the VM.
7. Wait for SSH to become available.
8. Run the provision steps in order, streaming output to your terminal.

## Connect

```bash
vmctl ssh
```

When there's only one VM in the VMFile, you don't need to specify the name.

## Make Changes

Edit `VMFile.kdl` to add another provisioner, then reload:

```bash
vmctl reload
```

This destroys the existing VM and recreates it from scratch with the updated definition.

To re-run just the provisioners without recreating:

```bash
vmctl provision
```

## Bring It Down

Stop the VM:

```bash
vmctl down
```

Or stop and destroy:

```bash
vmctl down --destroy
```

## Filtering by Name

If your VMFile defines multiple VMs, use `--name` to target a specific one:

```bash
vmctl up --name webserver
vmctl ssh --name webserver
vmctl down --name webserver
```
