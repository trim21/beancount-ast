import pytest
from syrupy.assertion import SnapshotAssertion

import beancount_ast

MIXED_CONTENT = """\
; comment line
option \"title\" \"Demo\"

2020-01-01 open Assets:Cash USD
2020-01-02 balance Assets:Cash 100 USD
2020-01-03 * \"Payee\" \"Narration\"
    Assets:Cash  -10 USD
    Expenses:Food  10 USD

2020-01-03 * \"Payee\" \"Narration\"
  Assets:Cash  -10 USD
  Expenses:Food  10 USD

2020-01-04 price USD 1.23 EUR
2020-01-05 event \"location\" \"NYC\"
2020-01-06 note Assets:Cash \"ATM withdrawal\"
2020-01-07 custom \"x\" \"y\"

plugin \"beancount.plugins.auto_accounts\"
include \"other.bean\"

2020-12-31 close Assets:Cash
"""


def test_parse_string_mixed_directives_snapshot(snapshot: SnapshotAssertion):
    file = beancount_ast.parse_string(MIXED_CONTENT, filename="mixed.bean")
    directive_dumps = [
        (directive.__class__.__name__, directive.dump())
        for directive in file.directives
    ]
    assert directive_dumps == snapshot


def test_parse_string_error_snapshot(snapshot: SnapshotAssertion):
    with pytest.raises(ValueError) as exc:
        beancount_ast.parse_string("this is not a directive\n", filename="bad.bean")

    # Error string includes spans/locations; keep the exact message snapshotted.
    assert str(exc.value) == snapshot


def test_dump_reconstructs_source():
    file = beancount_ast.parse_string(MIXED_CONTENT, filename="mixed.bean")
    directives = file.directives

    for directive in directives:
        expected = file.content[directive.span.start : directive.span.end]
        assert directive.dump() == expected

    transaction = next(
        d for d in directives if isinstance(d, beancount_ast.Transaction)
    )
    for posting in transaction.postings:
        expected = file.content[posting.span.start : posting.span.end]
        assert posting.dump() == expected

        if posting.amount is not None:
            amount_span = posting.amount.raw.span
            expected_amount = file.content[amount_span.start : amount_span.end]
            assert posting.amount.dump() == expected_amount

        if posting.cost_spec is not None:
            spec_span = posting.cost_spec.raw.span
            expected_spec = file.content[spec_span.start : spec_span.end]
            assert posting.cost_spec.dump() == expected_spec

    custom = next(d for d in directives if isinstance(d, beancount_ast.Custom))
    for value in custom.values:
        raw_span = value.raw.span
        expected_value = file.content[raw_span.start : raw_span.end]
        assert value.dump() == expected_value


def test_indent():
    content = """\
; comment line
2020-01-03 * \"Payee\" \"Narration\"
    Assets:Cash  -10 USD
    Expenses:Food        10 USD
""".strip()

    file = beancount_ast.parse_string(content, filename="mixed.bean")
    assert "\n".join([r.dump() for r in file.directives]).strip() == content


def test_indent2():
    content = """\
; comment line
2020-01-03 * \"Payee\" \"Narration\"
  Assets:Cash             -10 USD
  Expenses:Food  10 USD
""".strip()

    file = beancount_ast.parse_string(content, filename="mixed.bean")
    assert "\n".join([r.dump() for r in file.directives]).strip() == content
