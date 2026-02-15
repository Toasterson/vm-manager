# Image Management API

The image module handles downloading, caching, format detection, and overlay creation. Located in `crates/vm-manager/src/image.rs`.

## ImageManager

```rust
pub struct ImageManager {
    client: reqwest::Client,
    cache: PathBuf,  // default: ~/.local/share/vmctl/images/
}
```

### new

```rust
ImageManager::new() -> Self
```

Creates an ImageManager with the default cache directory.

### download

```rust
async fn download(&self, url: &str, destination: &Path) -> Result<()>
```

Downloads an image from a URL to a local path. Skips if the destination already exists. Auto-decompresses `.zst`/`.zstd` files. Logs progress every 5%.

### pull

```rust
async fn pull(&self, url: &str, name: Option<&str>) -> Result<PathBuf>
```

Downloads an image to the cache directory and returns the cached path. If `name` is None, extracts the filename from the URL.

### list

```rust
fn list(&self) -> Result<Vec<CachedImage>>
```

Lists all images in the cache with their names, sizes, and paths.

### detect_format

```rust
async fn detect_format(path: &Path) -> Result<String>
```

Runs `qemu-img info --output=json` and returns the format string (e.g., `"qcow2"`, `"raw"`).

### create_overlay

```rust
async fn create_overlay(base: &Path, overlay: &Path, size_gb: Option<u32>) -> Result<()>
```

Creates a QCOW2 overlay with the given base image as a backing file. Optionally resizes to `size_gb`.

### convert

```rust
async fn convert(src: &Path, dst: &Path, format: &str) -> Result<()>
```

Converts an image between formats using `qemu-img convert`.
