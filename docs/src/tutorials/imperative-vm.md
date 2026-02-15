# Creating a VM Imperatively

This tutorial walks through the full lifecycle of a VM using individual vmctl commands.

## Create a VM

```bash
vmctl create \
  --name tutorial \
  --image-url https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img \
  --vcpus 2 \
  --memory 2048 \
  --ssh-key ~/.ssh/id_ed25519.pub
```

This downloads the image (cached for future use), creates a QCOW2 overlay, generates a cloud-init ISO with your SSH key, and registers the VM.

## Start It

```bash
vmctl start tutorial
```

## Check Status

```bash
vmctl list
```

```text
NAME             BACKEND  VCPUS   MEM NETWORK     PID      SSH
tutorial         qemu     2       2048 user       12345    10042
```

For detailed info:

```bash
vmctl status tutorial
```

## Connect via SSH

```bash
vmctl ssh tutorial
```

vmctl waits for SSH to become available (cloud-init needs a moment to set up the user), then drops you into a shell.

## Suspend and Resume

Pause the VM without shutting it down:

```bash
vmctl suspend tutorial
```

Resume it:

```bash
vmctl resume tutorial
```

The VM continues from exactly where it was, no reboot needed.

## Stop the VM

```bash
vmctl stop tutorial
```

This sends an ACPI power-down signal. If the guest doesn't shut down within 30 seconds, vmctl sends SIGTERM.

To change the timeout:

```bash
vmctl stop tutorial --timeout 60
```

## Restart

A stopped VM can be started again:

```bash
vmctl start tutorial
```

## Destroy

When you're done, clean up everything:

```bash
vmctl destroy tutorial
```

This stops the VM (if running), removes the overlay, cloud-init ISO, and all work directory files, and unregisters the VM from the store.
