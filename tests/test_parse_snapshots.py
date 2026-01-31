import pytest
import beancount_ast


def test_parse_string_mixed_directives_snapshot(snapshot):
    content = """\
; comment line
option \"title\" \"Demo\"

2020-01-01 open Assets:Cash USD
2020-01-02 balance Assets:Cash 100 USD
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

    directives = beancount_ast.parse_string(content, filename="mixed.bean")
    assert directives == snapshot


def test_parse_string_error_snapshot(snapshot):
    with pytest.raises(ValueError) as exc:
        beancount_ast.parse_string("this is not a directive\n", filename="bad.bean")

    # Error string includes spans/locations; keep the exact message snapshotted.
    assert str(exc.value) == snapshot
