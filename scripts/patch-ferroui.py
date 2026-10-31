#!/usr/bin/env python3
"""Builds a generated project against a FerroUI checkout instead of crates.io.

    python3 scripts/patch-ferroui.py <generated project> <FerroUI checkout>

Appends a `[patch.crates-io]` section to the `Cargo.toml` of the project (its workspace root) that
points every FerroUI crate of the checkout's `[workspace.dependencies]` (the entries with both a
version and a path) at its directory in the checkout. For trying the templates against FerroUI
main or a release that is not on crates.io yet; the versions the templates name must be the
version of the checkout. Running it twice changes nothing more.
"""

import pathlib
import re
import sys

MARKER = "# >>> FerroUI checkout (scripts/patch-ferroui.py)"


def main() -> int:
    if len(sys.argv) != 3:
        print(__doc__.strip(), file=sys.stderr)
        return 2
    project = pathlib.Path(sys.argv[1]).resolve()
    ferroui = pathlib.Path(sys.argv[2]).resolve()
    manifest = project / "Cargo.toml"
    text = manifest.read_text()
    if MARKER in text:
        return 0
    workspace = (ferroui / "Cargo.toml").read_text()
    section = re.search(r"^\[workspace\.dependencies\]\n(.*?)(?=^\[)", workspace, re.S | re.M)
    if not section:
        print(f"{ferroui}/Cargo.toml has no [workspace.dependencies]", file=sys.stderr)
        return 1
    lines = [MARKER, "[patch.crates-io]"]
    for match in re.finditer(r'^([A-Za-z0-9_-]+) = \{ version = "=[^"]+", path = "([^"]+)"', section.group(1), re.M):
        name, path = match.groups()
        lines.append(f'{name} = {{ path = "{(ferroui / path).as_posix()}" }}')
    if len(lines) == 2:
        print("no FerroUI crates found in the checkout", file=sys.stderr)
        return 1
    manifest.write_text(text.rstrip("\n") + "\n\n" + "\n".join(lines) + "\n")
    print(f"{manifest}: {len(lines) - 2} FerroUI crates from {ferroui}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
