r"""Generate `beancount_ast` Python stubs via `cargo run`.

Why this exists:
- On Windows, running the Rust `stub_gen` binary directly can fail with
  missing `python3.dll`.
- When this script is invoked using the project's Python (e.g. `.venv`), we can
  reliably locate the Python DLL directory and prepend it to `PATH` before
  running `cargo`.

Usage (PowerShell):
  .\.venv\Scripts\python.exe tools\generate_ast_stubs.py

This runs:
  cargo run --manifest-path crates/ast-py/Cargo.toml --bin stub_gen --features stub-gen
"""

from __future__ import annotations

import os
import subprocess
import sys
from pathlib import Path


def _prepend_to_path(env: dict[str, str]) -> None:
    current = env["PATH"]
    parts = [p for p in current.split(os.pathsep) if p]

    env["PATH"] = os.pathsep.join(sys.path + parts)


def main() -> int:
    repo_root = Path(__file__).resolve().parent.parent
    manifest = repo_root / "Cargo.toml"

    if not manifest.exists():
        raise SystemExit(f"Missing manifest: {manifest}")

    env = os.environ.copy()
    _prepend_to_path(env)

    cmd = [
        "cargo",
        "run",
        "--manifest-path",
        str(manifest),
        "--bin",
        "stub_gen",
        "--features",
        "stub-gen",
    ]

    print("Running:", " ".join(cmd), file=sys.stderr)
    return subprocess.check_call(cmd, cwd=str(repo_root), env=env)


if __name__ == "__main__":
    raise SystemExit(main())
