# Copilot instructions (beancount-ast)

## Project overview
- This repo provides Python bindings (PyO3) for the Rust `beancount-parser` directive AST.
- The Python API intentionally exposes the **parser AST** (directives + spans + raw tokens), not Beancount’s semantic `beancount.core` model.

## Key layout
- `src/lib.rs`: PyO3 module `beancount_ast._ast`.
  - Registers all exposed Python classes in `#[pymodule(name = "_ast")]`.
  - Converts `beancount_parser::ast::*` nodes into `Py*` wrappers.
  - Exposes `parse_string` and `parse_file` to Python.
- `py-src/beancount_ast/__init__.py`: re-exports compiled `_ast` symbols and the `Directive` ABC for consumers.
- `py-src/beancount_ast/_directive.py`: defines the `Directive` ABC and registers all directive classes.
- `py-src/beancount_ast/_ast.pyi`: canonical type stubs mirroring the compiled `beancount_ast._ast` extension.
- `tests/test_parse_snapshots.py`: snapshot-style API tests (`pytest` + `syrupy`).

## Workflows (local + CI-aligned)
- Python deps / test env are managed with `uv` (see `.github/workflows/tests.yml`).
  - Setup: `uv sync --dev --no-install-project`
  - Run tests: `uv run pytest`
- Rust checks mirror CI (`.github/workflows/ci.yml`):
  - Format: `cargo fmt --all -- --check`
  - Lint: `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Conventions to follow when changing Rust bindings
- Avoid `unwrap()` / `expect()`; the crate denies them (`src/lib.rs` has `#![deny(clippy::unwrap_used, clippy::expect_used)]`).
- Python-facing data structures are thin, mostly immutable “record” types:
  - Define a `Py*` struct with `#[pyclass(..., get_all)]` and `pyderive` derives (`PyNew`, `PyRepr`, `PyStr`, and sometimes `PyEq`).
  - If the type should appear in stubs, add `#[cfg_attr(feature = "stub-gen", ...gen_stub_pyclass)]`.
- When adding a new directive/type:
  1) Add the `Py*` struct.
  2) Register it in the `_ast` module init.
  3) Extend the conversion layer (e.g. `directive_to_py(...)`).
  4) Update the `Directive` type alias used for stubs (see the `stub-gen` block at the end of `src/lib.rs`).

## Formatting / linting
- Non-code config formatting uses `dprint` (see `dprint.json`) and is enforced via pre-commit.
- Python formatting/linting uses `ruff` + `black` (see `.pre-commit-config.yaml`).

## Release notes (when relevant)
- CI builds wheels via `maturin` (see `.github/workflows/_build_wheels.yaml`).
- Tags `v*` publish to PyPI (see `.github/workflows/release.yaml`).
