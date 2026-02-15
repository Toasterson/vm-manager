# vmctl image

Manage VM disk images.

## Synopsis

```
vmctl image <SUBCOMMAND>
```

## Subcommands

### vmctl image pull

Download an image to the local cache.

```
vmctl image pull [OPTIONS] <URL>
```

| Argument/Option | Type | Description |
|---|---|---|
| `URL` | string | URL to download (positional) |
| `--name` | string | Name to save as in the cache |

### vmctl image list

List cached images.

```
vmctl image list
```

Output:

```text
NAME                                     SIZE         PATH
noble-server-cloudimg-amd64.img          0.62 GB      /home/user/.local/share/vmctl/images/noble-server-cloudimg-amd64.img
```

### vmctl image inspect

Show image format and details.

```
vmctl image inspect <PATH>
```

| Argument | Description |
|---|---|
| `PATH` | Path to image file (positional) |

## Examples

```bash
# Download and cache an image
vmctl image pull https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img

# List what's cached
vmctl image list

# Check format of a local image
vmctl image inspect ./my-image.qcow2
```
