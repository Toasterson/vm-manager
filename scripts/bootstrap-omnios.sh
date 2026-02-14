#!/usr/bin/env bash
set -euo pipefail

# OmniOS bootstrap: install system packages and Rust toolchain.
# Adapted from refraction-forger's bootstrap-illumos.sh.

# ---------------------------------------------------------------------------
# Prefer GNU userland (find, xargs, etc.) on OmniOS. Native /usr/bin tools
# lack options like -maxdepth and xargs -r.
# ---------------------------------------------------------------------------
for d in \
  /usr/gnu/bin \
  /usr/local/gnu/bin \
  /opt/csw/gnu \
  /opt/csw/bin \
  /usr/sfw/bin
do
  if [ -d "$d" ]; then
    case ":$PATH:" in
      *":$d:"*) ;;
      *) PATH="$d:$PATH" ;;
    esac
  fi
done
export PATH

# ---------------------------------------------------------------------------
# System packages via IPS (pkg)
# ---------------------------------------------------------------------------
if ! command -v pkg >/dev/null 2>&1; then
  echo "[bootstrap] pkg not found — expected OmniOS with IPS." >&2
  exit 1
fi

echo "[bootstrap] Refreshing package catalog ..."
sudo pkg refresh --full

echo "[bootstrap] Installing system prerequisites ..."
sudo pkg install -v \
  file/gnu-findutils \
  developer/gcc14 \
  developer/build/gnu-make \
  developer/pkg-config \
  library/security/openssl \
  web/curl \
  compress/unzip \
  developer/versioning/git \
  web/ca-bundle || true

# ---------------------------------------------------------------------------
# Rust toolchain via rustup
# ---------------------------------------------------------------------------
if ! command -v cargo >/dev/null 2>&1; then
  echo "[bootstrap] Installing Rust toolchain via rustup ..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
    | sh -s -- -y --profile minimal
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi

export PATH="$HOME/.cargo/bin:$PATH"

# ---------------------------------------------------------------------------
# Verify critical tools
# ---------------------------------------------------------------------------
echo "[bootstrap] Verifying installed tools ..."
for tool in gcc cargo rustc pkg-config git curl; do
  if ! command -v "$tool" >/dev/null 2>&1; then
    echo "[bootstrap] ERROR: $tool not found after installation." >&2
    exit 1
  fi
  echo "  $tool: $(command -v "$tool")"
done

echo "[bootstrap] OmniOS bootstrap complete."
