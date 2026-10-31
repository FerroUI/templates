#!/usr/bin/env bash
# Sets up the repository of the application once, after `cargo generate`: FerroUI as the git
# submodule at FerroUI/, the FerroUI blocks of Cargo.toml and the lock file (FerroUI/docs/apps.md).
#
#   scripts/setup.sh
#
# FERROUI_URL names another FerroUI repository or a local clone to add as the submodule.
set -euo pipefail
cd "$(dirname "$0")/.."

url="${FERROUI_URL:-https://github.com/wieslawsoltes/FerroUI.git}"
# A repository of its own (cargo generate makes one unless --vcs none), not one around it.
if [ "$(git rev-parse --show-toplevel 2> /dev/null)" != "$(pwd -P)" ]; then
  git init -q
fi
if [ ! -f FerroUI/Cargo.toml ]; then
  git -c protocol.file.allow=always submodule add "$url" FerroUI
fi
python3 FerroUI/scripts/apps/sync-workspace.py
# The lock file starts from FerroUI's, so that the application resolves the versions the framework
# is tested with; cargo then prunes and completes it.
[ -f Cargo.lock ] || cp FerroUI/Cargo.lock Cargo.lock
cargo metadata --format-version 1 > /dev/null
echo "Ready: cargo run -p {{project-name}}-desktop"
