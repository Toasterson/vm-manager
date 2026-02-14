#!/usr/bin/env bash
set -euo pipefail

# Pack the forger + spec-parser crates (and OmniOS image specs) into a tarball
# that can be uploaded to the OmniOS builder VM.

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
FORGER_ROOT="$PROJECT_ROOT/other-codes/refraction-forger"
OUTPUT="$SCRIPT_DIR/forger-src.tar.gz"

if [ ! -d "$FORGER_ROOT" ]; then
  echo "refraction-forger not found at $FORGER_ROOT" >&2
  exit 1
fi

STAGING="$(mktemp -d)"
trap 'rm -rf "$STAGING"' EXIT

echo "[pack-forger] Staging forger source ..."

mkdir -p "$STAGING/crates"

# Copy crates
cp -a "$FORGER_ROOT/crates/forger"      "$STAGING/crates/forger"
cp -a "$FORGER_ROOT/crates/spec-parser"  "$STAGING/crates/spec-parser"

# Copy image specs
cp -a "$FORGER_ROOT/images" "$STAGING/images"

# Copy lockfile if present (reproducible builds)
if [ -f "$FORGER_ROOT/Cargo.lock" ]; then
  cp "$FORGER_ROOT/Cargo.lock" "$STAGING/Cargo.lock"
fi

# Generate a minimal workspace Cargo.toml (only forger + spec-parser)
cat > "$STAGING/Cargo.toml" <<'TOML'
[workspace]
resolver = "2"
members = [
    "crates/forger",
    "crates/spec-parser",
]

[workspace.dependencies]
clap = { version = "4.5", features = ["derive", "env"] }
miette = { version = "7", features = ["fancy"] }
thiserror = "2"
knuffel = "3.2"
tracing = "0.1"
tracing-subscriber = "0.3"
serde = { version = "1.0", features = ["derive"] }
TOML

echo "[pack-forger] Creating $OUTPUT ..."
tar czf "$OUTPUT" -C "$STAGING" .

echo "[pack-forger] Done ($(du -h "$OUTPUT" | cut -f1))."
