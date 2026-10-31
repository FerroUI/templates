#!/usr/bin/env bash
# Generates a project from one template and builds it, as CI does (.github/workflows/ci.yml).
#
#   scripts/smoke.sh <template> <output directory> [--define name=value ...]
#
# Environment:
#   FERROUI_CHECKOUT  a FerroUI checkout to build against instead of crates.io (patch-ferroui.py;
#                     for the cross-platform template, the checkout the FerroUI/ submodule is
#                     cloned from)
#   SMOKE_STEPS       what to run in the generated project, of: check test (default: check test)
#   CARGO             the cargo command (default: cargo)
set -euo pipefail

template=$1
out=$2
shift 2
root="$(cd "$(dirname "$0")/.." && pwd)"
cargo=${CARGO:-cargo}
steps=${SMOKE_STEPS:-check test}

mkdir -p "$(dirname "$out")"
out="$(cd "$(dirname "$out")" && pwd)/$(basename "$out")"
rm -rf "$out"
name="$(basename "$out")"
( cd "$(dirname "$out")" && cargo generate --path "$root" "$template" --name "$name" --silent --vcs none "$@" )

if [ "$template" = cross-platform ]; then
  # The application repository layout: FerroUI as the submodule at FerroUI/ (docs/apps.md of
  # FerroUI), set up by the script of the generated project.
  ( cd "$out" && FERROUI_URL="${FERROUI_CHECKOUT:-https://github.com/wieslawsoltes/FerroUI.git}" scripts/setup.sh )
elif [ -n "${FERROUI_CHECKOUT:-}" ]; then
  python3 "$root/scripts/patch-ferroui.py" "$out" "$FERROUI_CHECKOUT"
  cp "$FERROUI_CHECKOUT/Cargo.lock" "$out/Cargo.lock"
fi

cd "$out"
for step in $steps; do
  case $step in
    check) $cargo check --workspace --all-targets ;;
    test) $cargo test --workspace ;;
    *) echo "unknown step $step" >&2; exit 2 ;;
  esac
done
