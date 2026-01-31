from typing import Any
import pytest
import beancount_ast


def test_parse_string_mixed_directives_snapshot(snapshot: Any):
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


def test_dump_string_mixed_directives_snapshot(snapshot: Any):
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
    dumped = [d.dump() for d in directives]
    assert dumped == snapshot


def test_dump_roundtrip_idempotent_mixed_directives():
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
    dumped1 = [d.dump() for d in directives]

    roundtrip_source = "\n\n".join(dumped1) + "\n"
    directives2 = beancount_ast.parse_string(
        roundtrip_source, filename="roundtrip.bean"
    )
    dumped2 = [d.dump() for d in directives2]

    assert dumped2 == dumped1


def test_parse_string_error_snapshot(snapshot: Any):
    with pytest.raises(ValueError) as exc:
        beancount_ast.parse_string("this is not a directive\n", filename="bad.bean")

    # Error string includes spans/locations; keep the exact message snapshotted.
    assert str(exc.value) == snapshot
