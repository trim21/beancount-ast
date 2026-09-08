# beancount-ast

Parse [Beancount](https://beancount.github.io/) input files into the parser's
directive AST, from Python.

This package exposes the **parser AST** — directives with spans and raw
tokens — not Beancount's semantic `beancount.core` directive model. It is
built with [PyO3](https://github.com/PyO3/pyo3) on top of the Rust
`beancount-parser` crate, so parsing is fast and does not require the
`beancount` Python package.

## Features

- Parse from a string (`parse_string`) or a file (`parse_file`).
- Every node carries its `span` (byte offsets) and the source `File`, so you
  can always map a node back to the original text.
- `dump()` reconstructs the original source text of any node.
- Classes have a dataclass-like constructor and `repr` (via
  [`pyderive`](https://github.com/Progi-1984/pyderive)).
- Fully typed with `py.typed` and stubs, requires Python 3.10+.

## Installation

```bash
pip install beancount-ast
```

## Usage

```python
import beancount_ast

content = """\
option "title" "Demo"

2020-01-01 open Assets:Cash USD

2020-01-03 * "Payee" "Narration"
    Assets:Cash  -10 USD
    Expenses:Food  10 USD
"""

file = beancount_ast.parse_string(content, filename="demo.bean")

for directive in file.directives:
    print(directive.__class__.__name__, directive.dump())
```

Every node exposes a `span` (`start` / `end` byte offsets) plus the owning
`File`, so node text is recoverable from the source:

```python
txn = next(
    d for d in file.directives
    if isinstance(d, beancount_ast.Transaction)
)
print(file.content[txn.span.start:txn.span.end])
```

## API overview

- `parse_file(filename) -> File`
- `parse_string(content, filename="<string>") -> File`
- `File`: `filename`, `content`, `directives`
- `Span`: `start`, `end` (byte offsets into `File.content`)
- Directive classes (all subclass `Directive` and provide `span`, `file` and
  `dump()`): `Open`, `Close`, `Balance`, `Pad`, `Transaction`, `Commodity`,
  `Price`, `Event`, `Query`, `Note`, `Document`, `Custom`, `Option`,
  `Include`, `Plugin`, `Tag`, `PushMeta`, `PopMeta`, `Comment`, `Headline`,
  `Raw`.
- Supporting types: `Posting`, `Amount`, `CostSpec`, `CostAmount`,
  `NumberExpr`, `KeyValue`, `KeyValueValue`, `CustomValue`, `SpannedStr`,
  `SpannedBool`, ...
- `ParseError` (a `ValueError`) with a list of `ParseErrorDetail` carrying
  message, span, line/col and expected tokens.

See [`py-src/beancount_ast/_ast.pyi`](py-src/beancount_ast/_ast.pyi) for the
complete type definitions.

## Development

```bash
uv sync --dev --no-install-project
maturin develop --locked --release --uv   # rebuild after Rust changes
pytest
```

Rust checks (same as CI):

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## License

MIT
