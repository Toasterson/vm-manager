# Image Management

vmctl can work with local disk images or download them from URLs. Downloaded images are cached for reuse.

## Image Cache

Downloaded images are stored in `~/.local/share/vmctl/images/`. If an image already exists in the cache, it won't be re-downloaded.

## Supported Formats

vmctl uses `qemu-img` to detect and convert image formats. Common formats:

- **qcow2** - QEMU's native format, supports snapshots and compression.
- **raw** - Plain disk image.

The format is auto-detected from the file header.

## Zstd Decompression

If a URL ends in `.zst` or `.zstd`, vmctl automatically decompresses the image after downloading. This is common for distribution cloud images.

## Overlay System

vmctl never boots from the base image directly. Instead:

1. The base image is stored in the cache (or at a local path you provide).
2. A QCOW2 overlay is created on top, pointing to the base as a backing file.
3. All writes go to the overlay. The base stays untouched.
4. Destroying a VM just removes the overlay.

This means multiple VMs can share the same base image efficiently.

## Disk Resizing

If you specify `disk` (in GB) in your VMFile or `--disk` on the CLI, the overlay is created with that size. The guest OS can then grow its filesystem to fill the available space (most cloud images do this automatically via cloud-init's `growpart` module).

## Managing Images with the CLI

```bash
# Download an image to the cache
vmctl image pull https://cloud-images.ubuntu.com/noble/current/noble-server-cloudimg-amd64.img

# List cached images
vmctl image list

# Inspect a local image
vmctl image inspect ./my-image.qcow2
```
