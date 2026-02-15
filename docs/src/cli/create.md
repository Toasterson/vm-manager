# vmctl create

Create a new VM and optionally start it.

## Synopsis

```
vmctl create [OPTIONS] --name <NAME>
```

## Options

| Option | Type | Default | Description |
|---|---|---|---|
| `--name` | string | *required* | VM name |
| `--image` | path | | Path to a local disk image |
| `--image-url` | string | | URL to download an image from |
| `--vcpus` | integer | `1` | Number of virtual CPUs |
| `--memory` | integer | `1024` | Memory in MB |
| `--disk` | integer | | Disk size in GB (overlay resize) |
| `--bridge` | string | | Bridge name for TAP networking |
| `--cloud-init` | path | | Path to cloud-init user-data file |
| `--ssh-key` | path | | Path to SSH public key file |
| `--start` | flag | `false` | Start the VM after creation |

## Details

One of `--image` or `--image-url` must be provided. If `--image-url` is given, the image is downloaded and cached.

When `--bridge` is specified, TAP networking is used. Otherwise, user-mode (SLIRP) networking is used.

When `--ssh-key` is provided, a cloud-init ISO is generated that injects the public key. The SSH user defaults to `"vm"`.

## Examples

```bash
# Create from a URL with defaults
vmctl create --name myvm --image-url https://example.com/image.img

# Create with custom resources and start immediately
vmctl create --name myvm \
  --image-url https://example.com/image.img \
  --vcpus 4 --memory 4096 --disk 40 \
  --ssh-key ~/.ssh/id_ed25519.pub \
  --start

# Create from local image with TAP networking
vmctl create --name myvm --image ./ubuntu.qcow2 --bridge br0
```

## See Also

[vmctl start](./start.md), [vmctl up](./up.md)
