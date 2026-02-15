# Provisioning

Provisioners run commands and upload files to a VM after it boots. They execute in order and stop on the first failure.

## Provision Types

### Shell with Inline Command

Execute a command directly on the guest:

```kdl
provision "shell" {
    inline "sudo apt-get update && sudo apt-get install -y curl"
}
```

### Shell with Script File

Upload and execute a local script:

```kdl
provision "shell" {
    script "scripts/setup.sh"
}
```

The script is uploaded to `/tmp/vmctl-provision-<step>.sh` on the guest, made executable, and run. Paths are relative to the directory containing `VMFile.kdl`.

### File Upload

Upload a file to the guest via SFTP:

```kdl
provision "file" {
    source "config/nginx.conf"
    destination "/tmp/nginx.conf"
}
```

## Execution Details

- Shell provisioners stream stdout/stderr to your terminal in real-time.
- A non-zero exit code aborts the entire provisioning sequence.
- Output is logged to `provision.log` in the VM's work directory.
- vmctl waits up to 120 seconds for SSH to become available before provisioning starts.

## Multi-Stage Example

A common pattern is to combine file uploads with shell commands:

```kdl
vm "builder" {
    image-url "https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img"
    vcpus 4
    memory 4096

    cloud-init {
        hostname "builder"
    }

    ssh {
        user "ubuntu"
    }

    // Stage 1: Install dependencies
    provision "shell" {
        inline "sudo apt-get update && sudo apt-get install -y build-essential"
    }

    // Stage 2: Upload source code
    provision "file" {
        source "src.tar.gz"
        destination "/tmp/src.tar.gz"
    }

    // Stage 3: Build
    provision "shell" {
        inline "cd /tmp && tar xzf src.tar.gz && cd src && make"
    }
}
```

## Re-Running Provisioners

To re-run provisioners on an already-running VM:

```bash
vmctl provision
```

Or for a specific VM:

```bash
vmctl provision --name builder
```

## Viewing Provision Logs

```bash
vmctl log builder --provision
```
