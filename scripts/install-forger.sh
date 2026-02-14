#!/usr/bin/env bash
set -euo pipefail

# Extract forger source, build, and install the binary.

# ---------------------------------------------------------------------------
# Cargo environment
# ---------------------------------------------------------------------------
if [ -f "$HOME/.cargo/env" ]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi
export PATH="$HOME/.cargo/bin:$PATH"

ARCHIVE="/tmp/forger-src.tar.gz"
DEST="$HOME/forger"

if [ ! -f "$ARCHIVE" ]; then
  echo "[install-forger] Archive not found: $ARCHIVE" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Extract
# ---------------------------------------------------------------------------
echo "[install-forger] Extracting $ARCHIVE -> $DEST ..."
rm -rf "$DEST"
mkdir -p "$DEST"
tar xzf "$ARCHIVE" -C "$DEST"

# ---------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------
echo "[install-forger] Building forger (release) ..."
cd "$DEST"
cargo build -p forger --release

# ---------------------------------------------------------------------------
# Install
# ---------------------------------------------------------------------------
echo "[install-forger] Installing /usr/local/bin/forger ..."
sudo install -m 755 -d /usr/local/bin
sudo install -m 755 "$DEST/target/release/forger" /usr/local/bin/forger

echo "[install-forger] Done."
forger --version || true
