#!/usr/bin/env python3
"""Checks the conventions of the templates, or of projects generated from them.

    python3 scripts/check-conventions.py              the repository (CI, job "Conventions")
    python3 scripts/check-conventions.py <directory>  a generated project (its FerroUI/ submodule
                                                      and target/ are left out)

- no file names the upstream .NET framework of FerroUI;
- thread-local variables are declared with `ferro_thread_local!`, never with `thread_local!`;
- no emoji;
- every FerroUI crate is named at one version (the repository only);
- the copies of the shared files are the same as `shared/` (the repository only).
"""

import pathlib
import re
import subprocess
import sys

# The pattern of the upstream name, written in parts so that this file does not match it.
UPSTREAM = re.compile("aval" + "onia|a" + "xaml|ava" + "res|\\ba" + "vn", re.I)
THREAD_LOCAL = re.compile(r"(^|[^_A-Za-z0-9:$])((::)?std::)?thread_local!")
EMOJI = re.compile("[%s-%s%s-%s%s%s]" % (chr(0x1F000), chr(0x1FAFF), chr(0x2600), chr(0x27BF), chr(0xFE0F), chr(0x200D)))
VERSION = re.compile(r'^(ferroui[a-z0-9-]*) = (?:\{ version = )?"=([^"]+)"', re.M)
SKIPPED = {"FerroUI", "target", ".git", "node_modules"}


def files(root: pathlib.Path, repository: bool):
    if repository:
        listed = subprocess.run(["git", "ls-files", "-z"], cwd=root, capture_output=True, check=True).stdout
        for name in listed.decode().split("\0"):
            if name:
                yield root / name
        return
    for path in sorted(root.rglob("*")):
        if path.is_file() and not SKIPPED.intersection(path.relative_to(root).parts):
            yield path


def main() -> int:
    repository = len(sys.argv) == 1
    root = pathlib.Path(__file__).resolve().parent.parent if repository else pathlib.Path(sys.argv[1]).resolve()
    errors = []
    versions = {}
    for path in files(root, repository):
        relative = path.relative_to(root).as_posix()
        if UPSTREAM.search(relative):
            errors.append(f"{relative}: the path names the upstream framework")
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, FileNotFoundError):
            continue
        for number, line in enumerate(text.splitlines(), 1):
            if UPSTREAM.search(line):
                errors.append(f"{relative}:{number}: names the upstream framework")
            if EMOJI.search(line):
                errors.append(f"{relative}:{number}: an emoji")
            if path.suffix == ".rs" and THREAD_LOCAL.search(line):
                errors.append(f"{relative}:{number}: thread_local! (declare it with ferroui_base::ferro_thread_local!)")
        if repository and path.name == "Cargo.toml":
            for crate, version in VERSION.findall(text):
                versions.setdefault(version, []).append(f"{relative} ({crate})")
    if repository:
        if len(versions) > 1:
            for version, users in sorted(versions.items()):
                errors.append(f"FerroUI {version}: {', '.join(users[:4])}{' ...' if len(users) > 4 else ''}")
            errors.append("the templates name more than one FerroUI version")
        shared = root / "shared"
        for source in sorted(shared.iterdir()):
            for copy in sorted(root.glob(f"*/{source.name}")) + sorted(root.glob(f"*/*/{source.name}")):
                if copy.parent == shared or "smoke" in copy.parts:
                    continue
                if copy.read_bytes() != source.read_bytes():
                    errors.append(f"{copy.relative_to(root)} differs from shared/{source.name}")
    for error in errors:
        print(error)
    if errors:
        print(f"{len(errors)} problem(s)", file=sys.stderr)
        return 1
    print(f"{root}: conventions hold")
    return 0


if __name__ == "__main__":
    sys.exit(main())
