# beancount_ast

Parse Beancount input into the Rust parser's directive AST from Python.

This package intentionally exposes the *parser AST* (directives + spans + raw tokens),
not Beancount's semantic `beancount.core` directive model.

Notes:
- Classes have a dataclass-like constructor and repr (via `pyderive`).
