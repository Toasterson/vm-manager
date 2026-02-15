# Image Sources

Every VM must specify exactly one image source. The two options are mutually exclusive.

## Local Image

```kdl
image "path/to/image.qcow2"
```

Points to a disk image on the host filesystem. The path is resolved relative to the VMFile directory, with tilde expansion.

The file must exist at parse time. Supported formats are auto-detected by `qemu-img` (qcow2, raw, etc.).

## Remote Image

```kdl
image-url "https://example.com/image.qcow2"
```

Downloads the image and caches it in `~/.local/share/vmctl/images/`. If the image is already cached, it won't be re-downloaded.

URLs ending in `.zst` or `.zstd` are automatically decompressed after download.

## Validation

- Exactly one of `image` or `image-url` must be specified.
- Specifying both is an error.
- Specifying neither is an error.
