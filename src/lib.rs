#![allow(clippy::large_enum_variant, clippy::too_many_arguments)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use beancount_parser::ast;
use beancount_parser::parse_str;
use pyderive::*;
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::fmt;

#[pymodule(name = "_ast")]
fn _ast(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // File container
    m.add_class::<PyFile>()?;

    // Core building blocks
    m.add_class::<PySpan>()?;
    m.add_class::<PySpannedStr>()?;
    m.add_class::<PySpannedBool>()?;
    m.add_class::<PyKeyValueValue>()?;
    m.add_class::<PySpannedKeyValueValue>()?;
    m.add_class::<PyKeyValue>()?;
    m.add_class::<PySpannedBinaryOp>()?;
    m.add_class::<PyNumberExpr>()?;
    m.add_class::<PyAmount>()?;
    m.add_class::<PyCostAmount>()?;
    m.add_class::<PyCostSpec>()?;
    m.add_class::<PySpannedPriceOperator>()?;
    m.add_class::<PyPosting>()?;
    m.add_class::<PyCustomValue>()?;

    // Directives
    m.add_class::<PyOpen>()?;
    m.add_class::<PyClose>()?;
    m.add_class::<PyBalance>()?;
    m.add_class::<PyPad>()?;
    m.add_class::<PyTransaction>()?;
    m.add_class::<PyCommodity>()?;
    m.add_class::<PyPrice>()?;
    m.add_class::<PyEvent>()?;
    m.add_class::<PyQuery>()?;
    m.add_class::<PyNote>()?;
    m.add_class::<PyDocument>()?;
    m.add_class::<PyCustom>()?;
    m.add_class::<PyOption>()?;
    m.add_class::<PyInclude>()?;
    m.add_class::<PyPlugin>()?;
    m.add_class::<PyTagDirective>()?;
    m.add_class::<PyPushMeta>()?;
    m.add_class::<PyPopMeta>()?;
    m.add_class::<PyComment>()?;
    m.add_class::<PyHeadline>()?;
    m.add_class::<PyRaw>()?;

    // API
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_file, m)?)?;
    Ok(())
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "File", get_all)]
struct PyFile {
    filename: String,
    content: String,
    directives: Vec<Py<PyAny>>, // mixed directive types
}

impl fmt::Debug for PyFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("File")
            .field("filename", &self.filename)
            .field("content_len", &self.content.len())
            .field("directives_len", &self.directives.len())
            .finish()
    }
}

#[derive(PartialEq, Eq, Hash, PyNew, PyRepr, PyStr, PyEq)]
#[pyclass(module = "beancount_ast._ast", name = "Span", get_all)]
struct PySpan {
    start: usize,
    end: usize,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedStr", get_all)]
struct PySpannedStr {
    span: Py<PySpan>,
    file: Py<PyFile>,
    content: String,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedBool", get_all)]
struct PySpannedBool {
    span: Py<PySpan>,
    file: Py<PyFile>,
    content: bool,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "KeyValueValue", get_all)]
struct PyKeyValueValue {
    kind: String,
    string: Option<String>,
    boolean: Option<bool>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedKeyValueValue", get_all)]
struct PySpannedKeyValueValue {
    span: Py<PySpan>,
    file: Py<PyFile>,
    content: Py<PyKeyValueValue>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "KeyValue", get_all)]
struct PyKeyValue {
    span: Py<PySpan>,
    file: Py<PyFile>,
    key: Py<PySpannedStr>,
    value: Option<Py<PySpannedKeyValueValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedBinaryOp", get_all)]
struct PySpannedBinaryOp {
    span: Py<PySpan>,
    file: Py<PyFile>,
    content: String,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "NumberExpr", get_all)]
struct PyNumberExpr {
    kind: String,
    span: Py<PySpan>,
    file: Py<PyFile>,
    literal: Option<Py<PySpannedStr>>,
    left: Option<Py<PyNumberExpr>>,
    op: Option<Py<PySpannedBinaryOp>>,
    right: Option<Py<PyNumberExpr>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Amount", get_all)]
struct PyAmount {
    raw: Py<PySpannedStr>,
    number: Py<PyNumberExpr>,
    currency: Option<Py<PySpannedStr>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "CostAmount", get_all)]
struct PyCostAmount {
    per: Option<Py<PyNumberExpr>>,
    total: Option<Py<PyNumberExpr>>,
    currency: Option<Py<PySpannedStr>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "CostSpec", get_all)]
struct PyCostSpec {
    raw: Py<PySpannedStr>,
    amount: Option<Py<PyCostAmount>>,
    date: Option<Py<PySpannedStr>>,
    label: Option<Py<PySpannedStr>>,
    merge: Option<Py<PySpannedBool>>,
    is_total: Py<PySpannedBool>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedPriceOperator", get_all)]
struct PySpannedPriceOperator {
    span: Py<PySpan>,
    file: Py<PyFile>,
    content: String,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Posting", get_all)]
struct PyPosting {
    span: Py<PySpan>,
    file: Py<PyFile>,
    opt_flag: Option<Py<PySpannedStr>>,
    account: Py<PySpannedStr>,
    amount: Option<Py<PyAmount>>,
    cost_spec: Option<Py<PyCostSpec>>,
    price_operator: Option<Py<PySpannedPriceOperator>>,
    price_annotation: Option<Py<PyAmount>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "CustomValue", get_all)]
struct PyCustomValue {
    raw: Py<PySpannedStr>,
    kind: String,
    number: Option<Py<PyNumberExpr>>,
    amount: Option<Py<PyAmount>>,
}

// --- Directives ---

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Open", get_all)]
struct PyOpen {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    currencies: Vec<Py<PySpannedStr>>,
    opt_booking: Option<Py<PySpannedStr>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Close", get_all)]
struct PyClose {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Balance", get_all)]
struct PyBalance {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    amount: Py<PyAmount>,
    tolerance: Option<Py<PySpannedStr>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Pad", get_all)]
struct PyPad {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    from_account: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Transaction", get_all)]
struct PyTransaction {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    txn: Option<Py<PySpannedStr>>,
    payee: Option<Py<PySpannedStr>>,
    narration: Option<Py<PySpannedStr>>,
    tags_links: Option<Vec<Py<PySpannedStr>>>,
    tags: Vec<Py<PySpannedStr>>,
    links: Vec<Py<PySpannedStr>>,
    comment: Option<Py<PySpannedStr>>,
    comments: Vec<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
    postings: Vec<Py<PyPosting>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Commodity", get_all)]
struct PyCommodity {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    currency: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Price", get_all)]
struct PyPrice {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    currency: Py<PySpannedStr>,
    amount: Py<PyAmount>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Event", get_all)]
struct PyEvent {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    event_type: Py<PySpannedStr>,
    desc: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Query", get_all)]
struct PyQuery {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    name: Py<PySpannedStr>,
    query: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Note", get_all)]
struct PyNote {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    note: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Document", get_all)]
struct PyDocument {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    filename: Py<PySpannedStr>,
    tags_links: Option<Vec<Py<PySpannedStr>>>,
    tags: Vec<Py<PySpannedStr>>,
    links: Vec<Py<PySpannedStr>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Custom", get_all)]
struct PyCustom {
    span: Py<PySpan>,
    file: Py<PyFile>,
    date: Py<PySpannedStr>,
    name: Py<PySpannedStr>,
    values: Vec<Py<PyCustomValue>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Option", get_all)]
struct PyOption {
    span: Py<PySpan>,
    file: Py<PyFile>,
    key: Py<PySpannedStr>,
    value: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Include", get_all)]
struct PyInclude {
    span: Py<PySpan>,
    file: Py<PyFile>,
    filename: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Plugin", get_all)]
struct PyPlugin {
    span: Py<PySpan>,
    file: Py<PyFile>,
    name: Py<PySpannedStr>,
    config: Option<Py<PySpannedStr>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Tag", get_all)]
struct PyTagDirective {
    span: Py<PySpan>,
    file: Py<PyFile>,
    tag: Py<PySpannedStr>,
    action: String,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "PushMeta", get_all)]
struct PyPushMeta {
    span: Py<PySpan>,
    file: Py<PyFile>,
    key: Py<PySpannedStr>,
    value: Option<Py<PySpannedKeyValueValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "PopMeta", get_all)]
struct PyPopMeta {
    span: Py<PySpan>,
    file: Py<PyFile>,
    key: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Comment", get_all)]
struct PyComment {
    span: Py<PySpan>,
    file: Py<PyFile>,
    text: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Headline", get_all)]
struct PyHeadline {
    span: Py<PySpan>,
    file: Py<PyFile>,
    text: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[pyclass(module = "beancount_ast._ast", name = "Raw", get_all)]
struct PyRaw {
    span: Py<PySpan>,
    file: Py<PyFile>,
    text: String,
}

// --- Conversions ---

fn span_to_py(py: Python<'_>, span: ast::Span) -> PyResult<Py<PySpan>> {
    Py::new(
        py,
        PySpan {
            start: span.start,
            end: span.end,
        },
    )
}

fn spanned_str_to_py(
    py: Python<'_>,
    ws: ast::WithSpan<&str>,
    file: &Py<PyFile>,
) -> PyResult<Py<PySpannedStr>> {
    let span = span_to_py(py, ws.span)?;
    Py::new(
        py,
        PySpannedStr {
            span,
            file: file.clone_ref(py),
            content: ws.content.to_owned(),
        },
    )
}

fn spanned_bool_to_py(
    py: Python<'_>,
    ws: ast::WithSpan<bool>,
    file: &Py<PyFile>,
) -> PyResult<Py<PySpannedBool>> {
    let span = span_to_py(py, ws.span)?;
    Py::new(
        py,
        PySpannedBool {
            span,
            file: file.clone_ref(py),
            content: ws.content,
        },
    )
}

fn key_value_value_to_py(
    py: Python<'_>,
    v: ast::KeyValueValue<'_>,
) -> PyResult<Py<PyKeyValueValue>> {
    match v {
        ast::KeyValueValue::Bool(b) => Py::new(
            py,
            PyKeyValueValue {
                kind: "Bool".to_owned(),
                string: None,
                boolean: Some(b),
            },
        ),
        ast::KeyValueValue::String(s) => Py::new(
            py,
            PyKeyValueValue {
                kind: "String".to_owned(),
                string: Some(s.to_owned()),
                boolean: None,
            },
        ),
        ast::KeyValueValue::UnquotedString(s) => Py::new(
            py,
            PyKeyValueValue {
                kind: "UnquotedString".to_owned(),
                string: Some(s.to_owned()),
                boolean: None,
            },
        ),
        ast::KeyValueValue::Date(s) => Py::new(
            py,
            PyKeyValueValue {
                kind: "Date".to_owned(),
                string: Some(s.to_owned()),
                boolean: None,
            },
        ),
        ast::KeyValueValue::Raw(s) => Py::new(
            py,
            PyKeyValueValue {
                kind: "Raw".to_owned(),
                string: Some(s.to_owned()),
                boolean: None,
            },
        ),
    }
}

fn spanned_key_value_value_to_py(
    py: Python<'_>,
    ws: ast::WithSpan<ast::KeyValueValue<'_>>,
    file: &Py<PyFile>,
) -> PyResult<Py<PySpannedKeyValueValue>> {
    let span = span_to_py(py, ws.span)?;
    let content = key_value_value_to_py(py, ws.content)?;
    Py::new(
        py,
        PySpannedKeyValueValue {
            span,
            file: file.clone_ref(py),
            content,
        },
    )
}

fn key_value_to_py(
    py: Python<'_>,
    kv: ast::KeyValue<'_>,
    file: &Py<PyFile>,
) -> PyResult<Py<PyKeyValue>> {
    let span = span_to_py(py, kv.span)?;
    let key = spanned_str_to_py(py, kv.key, file)?;
    let value = match kv.value {
        Some(v) => Some(spanned_key_value_value_to_py(py, v, file)?),
        None => None,
    };

    Py::new(
        py,
        PyKeyValue {
            span,
            file: file.clone_ref(py),
            key,
            value,
        },
    )
}

fn spanned_binary_op_to_py(
    py: Python<'_>,
    ws: ast::WithSpan<ast::BinaryOp>,
    file: &Py<PyFile>,
) -> PyResult<Py<PySpannedBinaryOp>> {
    let span = span_to_py(py, ws.span)?;
    let content = match ws.content {
        ast::BinaryOp::Add => "Add",
        ast::BinaryOp::Sub => "Sub",
        ast::BinaryOp::Mul => "Mul",
        ast::BinaryOp::Div => "Div",
    };
    Py::new(
        py,
        PySpannedBinaryOp {
            span,
            file: file.clone_ref(py),
            content: content.to_owned(),
        },
    )
}

fn number_expr_to_py(
    py: Python<'_>,
    expr: ast::NumberExpr<'_>,
    file: &Py<PyFile>,
) -> PyResult<Py<PyNumberExpr>> {
    match expr {
        ast::NumberExpr::Missing { span } => {
            let span = span_to_py(py, span)?;
            Py::new(
                py,
                PyNumberExpr {
                    kind: "Missing".to_owned(),
                    span,
                    file: file.clone_ref(py),
                    literal: None,
                    left: None,
                    op: None,
                    right: None,
                },
            )
        }
        ast::NumberExpr::Literal(ws) => {
            let span = span_to_py(py, ws.span)?;
            let literal = Some(spanned_str_to_py(py, ws, file)?);
            Py::new(
                py,
                PyNumberExpr {
                    kind: "Literal".to_owned(),
                    span,
                    file: file.clone_ref(py),
                    literal,
                    left: None,
                    op: None,
                    right: None,
                },
            )
        }
        ast::NumberExpr::Binary {
            span,
            left,
            op,
            right,
        } => {
            let span = span_to_py(py, span)?;
            let left = Some(number_expr_to_py(py, *left, file)?);
            let op = Some(spanned_binary_op_to_py(py, op, file)?);
            let right = Some(number_expr_to_py(py, *right, file)?);
            Py::new(
                py,
                PyNumberExpr {
                    kind: "Binary".to_owned(),
                    span,
                    file: file.clone_ref(py),
                    literal: None,
                    left,
                    op,
                    right,
                },
            )
        }
    }
}

fn amount_to_py(py: Python<'_>, amt: ast::Amount<'_>, file: &Py<PyFile>) -> PyResult<Py<PyAmount>> {
    let raw = spanned_str_to_py(py, amt.raw, file)?;
    let number = number_expr_to_py(py, amt.number, file)?;
    let currency = match amt.currency {
        Some(c) => Some(spanned_str_to_py(py, c, file)?),
        None => None,
    };
    Py::new(
        py,
        PyAmount {
            raw,
            number,
            currency,
        },
    )
}

fn cost_amount_to_py(
    py: Python<'_>,
    ca: ast::CostAmount<'_>,
    file: &Py<PyFile>,
) -> PyResult<Py<PyCostAmount>> {
    let per = match ca.per {
        Some(p) => Some(number_expr_to_py(py, p, file)?),
        None => None,
    };
    let total = match ca.total {
        Some(t) => Some(number_expr_to_py(py, t, file)?),
        None => None,
    };
    let currency = match ca.currency {
        Some(c) => Some(spanned_str_to_py(py, c, file)?),
        None => None,
    };
    Py::new(
        py,
        PyCostAmount {
            per,
            total,
            currency,
        },
    )
}

fn cost_spec_to_py(
    py: Python<'_>,
    cs: ast::CostSpec<'_>,
    file: &Py<PyFile>,
) -> PyResult<Py<PyCostSpec>> {
    let raw = spanned_str_to_py(py, cs.raw, file)?;
    let amount = match cs.amount {
        Some(a) => Some(cost_amount_to_py(py, a, file)?),
        None => None,
    };
    let date = match cs.date {
        Some(d) => Some(spanned_str_to_py(py, d, file)?),
        None => None,
    };
    let label = match cs.label {
        Some(l) => Some(spanned_str_to_py(py, l, file)?),
        None => None,
    };
    let merge = match cs.merge {
        Some(m) => Some(spanned_bool_to_py(py, m, file)?),
        None => None,
    };
    let is_total = spanned_bool_to_py(py, cs.is_total, file)?;
    Py::new(
        py,
        PyCostSpec {
            raw,
            amount,
            date,
            label,
            merge,
            is_total,
        },
    )
}

fn spanned_price_operator_to_py(
    py: Python<'_>,
    ws: ast::WithSpan<ast::PriceOperator>,
    file: &Py<PyFile>,
) -> PyResult<Py<PySpannedPriceOperator>> {
    let span = span_to_py(py, ws.span)?;
    let content = match ws.content {
        ast::PriceOperator::PerUnit => "PerUnit",
        ast::PriceOperator::Total => "Total",
    };
    Py::new(
        py,
        PySpannedPriceOperator {
            span,
            file: file.clone_ref(py),
            content: content.to_owned(),
        },
    )
}

fn posting_to_py(
    py: Python<'_>,
    p: ast::Posting<'_>,
    file: &Py<PyFile>,
) -> PyResult<Py<PyPosting>> {
    let span = span_to_py(py, p.span)?;
    let opt_flag = match p.opt_flag {
        Some(f) => Some(spanned_str_to_py(py, f, file)?),
        None => None,
    };
    let account = spanned_str_to_py(py, p.account, file)?;
    let amount = match p.amount {
        Some(a) => Some(amount_to_py(py, a, file)?),
        None => None,
    };
    let cost_spec = match p.cost_spec {
        Some(cs) => Some(cost_spec_to_py(py, cs, file)?),
        None => None,
    };
    let price_operator = match p.price_operator {
        Some(po) => Some(spanned_price_operator_to_py(py, po, file)?),
        None => None,
    };
    let price_annotation = match p.price_annotation {
        Some(pa) => Some(amount_to_py(py, pa, file)?),
        None => None,
    };
    let comment = match p.comment {
        Some(c) => Some(spanned_str_to_py(py, c, file)?),
        None => None,
    };
    let mut key_values = Vec::with_capacity(p.key_values.len());
    for kv in p.key_values {
        key_values.push(key_value_to_py(py, kv, file)?);
    }

    Py::new(
        py,
        PyPosting {
            span,
            file: file.clone_ref(py),
            opt_flag,
            account,
            amount,
            cost_spec,
            price_operator,
            price_annotation,
            comment,
            key_values,
        },
    )
}

fn custom_value_to_py(
    py: Python<'_>,
    v: ast::CustomValue<'_>,
    file: &Py<PyFile>,
) -> PyResult<Py<PyCustomValue>> {
    let raw = spanned_str_to_py(py, v.raw, file)?;
    let kind = match v.kind {
        ast::CustomValueKind::String => "String",
        ast::CustomValueKind::Date => "Date",
        ast::CustomValueKind::Bool => "Bool",
        ast::CustomValueKind::Amount => "Amount",
        ast::CustomValueKind::Number => "Number",
        ast::CustomValueKind::Account => "Account",
    };
    let number = match v.number {
        Some(n) => Some(number_expr_to_py(py, n, file)?),
        None => None,
    };
    let amount = match v.amount {
        Some(a) => Some(amount_to_py(py, a, file)?),
        None => None,
    };
    Py::new(
        py,
        PyCustomValue {
            raw,
            kind: kind.to_owned(),
            number,
            amount,
        },
    )
}

fn directive_to_py(
    py: Python<'_>,
    d: ast::Directive<'_>,
    file: &Py<PyFile>,
) -> PyResult<Py<PyAny>> {
    let obj: Py<PyAny> = match d {
        ast::Directive::Open(o) => {
            let span = span_to_py(py, o.span)?;
            let date = spanned_str_to_py(py, o.date, file)?;
            let account = spanned_str_to_py(py, o.account, file)?;
            let currencies = o
                .currencies
                .into_iter()
                .map(|c| spanned_str_to_py(py, c, file))
                .collect::<PyResult<Vec<_>>>()?;
            let opt_booking = match o.opt_booking {
                Some(b) => Some(spanned_str_to_py(py, b, file)?),
                None => None,
            };
            let comment = match o.comment {
                Some(c) => Some(spanned_str_to_py(py, c, file)?),
                None => None,
            };
            let key_values = o
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;

            PyOpen {
                span,
                file: file.clone_ref(py),
                date,
                account,
                currencies,
                opt_booking,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Close(c) => {
            let span = span_to_py(py, c.span)?;
            let date = spanned_str_to_py(py, c.date, file)?;
            let account = spanned_str_to_py(py, c.account, file)?;
            let comment = match c.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = c
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyClose {
                span,
                file: file.clone_ref(py),
                date,
                account,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Balance(b) => {
            let span = span_to_py(py, b.span)?;
            let date = spanned_str_to_py(py, b.date, file)?;
            let account = spanned_str_to_py(py, b.account, file)?;
            let amount = amount_to_py(py, b.amount, file)?;
            let tolerance = match b.tolerance {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let comment = match b.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = b
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyBalance {
                span,
                file: file.clone_ref(py),
                date,
                account,
                amount,
                tolerance,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Pad(p) => {
            let span = span_to_py(py, p.span)?;
            let date = spanned_str_to_py(py, p.date, file)?;
            let account = spanned_str_to_py(py, p.account, file)?;
            let from_account = spanned_str_to_py(py, p.from_account, file)?;
            let comment = match p.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = p
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyPad {
                span,
                file: file.clone_ref(py),
                date,
                account,
                from_account,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Transaction(t) => {
            let span = span_to_py(py, t.span)?;
            let date = spanned_str_to_py(py, t.date, file)?;
            let txn = match t.txn {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let payee = match t.payee {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let narration = match t.narration {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let tags_links = match t.tags_links {
                Some(values) => Some(
                    values
                        .into_iter()
                        .map(|s| spanned_str_to_py(py, s, file))
                        .collect::<PyResult<Vec<_>>>()?,
                ),
                None => None,
            };
            let tags = t
                .tags
                .into_iter()
                .map(|s| spanned_str_to_py(py, s, file))
                .collect::<PyResult<Vec<_>>>()?;
            let links = t
                .links
                .into_iter()
                .map(|s| spanned_str_to_py(py, s, file))
                .collect::<PyResult<Vec<_>>>()?;
            let comment = match t.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let comments = t
                .comments
                .into_iter()
                .map(|s| spanned_str_to_py(py, s, file))
                .collect::<PyResult<Vec<_>>>()?;
            let key_values = t
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            let postings = t
                .postings
                .into_iter()
                .map(|p| posting_to_py(py, p, file))
                .collect::<PyResult<Vec<_>>>()?;

            PyTransaction {
                span,
                file: file.clone_ref(py),
                date,
                txn,
                payee,
                narration,
                tags_links,
                tags,
                links,
                comment,
                comments,
                key_values,
                postings,
            }
            .into_py_any(py)?
        }
        ast::Directive::Commodity(c) => {
            let span = span_to_py(py, c.span)?;
            let date = spanned_str_to_py(py, c.date, file)?;
            let currency = spanned_str_to_py(py, c.currency, file)?;
            let comment = match c.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = c
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyCommodity {
                span,
                file: file.clone_ref(py),
                date,
                currency,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Price(p) => {
            let span = span_to_py(py, p.span)?;
            let date = spanned_str_to_py(py, p.date, file)?;
            let currency = spanned_str_to_py(py, p.currency, file)?;
            let amount = amount_to_py(py, p.amount, file)?;
            let comment = match p.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = p
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyPrice {
                span,
                file: file.clone_ref(py),
                date,
                currency,
                amount,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Event(e) => {
            let span = span_to_py(py, e.span)?;
            let date = spanned_str_to_py(py, e.date, file)?;
            let event_type = spanned_str_to_py(py, e.event_type, file)?;
            let desc = spanned_str_to_py(py, e.desc, file)?;
            let comment = match e.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = e
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyEvent {
                span,
                file: file.clone_ref(py),
                date,
                event_type,
                desc,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Query(q) => {
            let span = span_to_py(py, q.span)?;
            let date = spanned_str_to_py(py, q.date, file)?;
            let name = spanned_str_to_py(py, q.name, file)?;
            let query = spanned_str_to_py(py, q.query, file)?;
            let comment = match q.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = q
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyQuery {
                span,
                file: file.clone_ref(py),
                date,
                name,
                query,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Note(n) => {
            let span = span_to_py(py, n.span)?;
            let date = spanned_str_to_py(py, n.date, file)?;
            let account = spanned_str_to_py(py, n.account, file)?;
            let note = spanned_str_to_py(py, n.note, file)?;
            let comment = match n.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = n
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyNote {
                span,
                file: file.clone_ref(py),
                date,
                account,
                note,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Document(d) => {
            let span = span_to_py(py, d.span)?;
            let date = spanned_str_to_py(py, d.date, file)?;
            let account = spanned_str_to_py(py, d.account, file)?;
            let filename = spanned_str_to_py(py, d.filename, file)?;
            let tags_links = match d.tags_links {
                Some(values) => Some(
                    values
                        .into_iter()
                        .map(|s| spanned_str_to_py(py, s, file))
                        .collect::<PyResult<Vec<_>>>()?,
                ),
                None => None,
            };
            let tags = d
                .tags
                .into_iter()
                .map(|s| spanned_str_to_py(py, s, file))
                .collect::<PyResult<Vec<_>>>()?;
            let links = d
                .links
                .into_iter()
                .map(|s| spanned_str_to_py(py, s, file))
                .collect::<PyResult<Vec<_>>>()?;
            let comment = match d.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = d
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyDocument {
                span,
                file: file.clone_ref(py),
                date,
                account,
                filename,
                tags_links,
                tags,
                links,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Custom(c) => {
            let span = span_to_py(py, c.span)?;
            let date = spanned_str_to_py(py, c.date, file)?;
            let name = spanned_str_to_py(py, c.name, file)?;
            let values = c
                .values
                .into_iter()
                .map(|v| custom_value_to_py(py, v, file))
                .collect::<PyResult<Vec<_>>>()?;
            let comment = match c.comment {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            let key_values = c
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv, file))
                .collect::<PyResult<Vec<_>>>()?;
            PyCustom {
                span,
                file: file.clone_ref(py),
                date,
                name,
                values,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Option(o) => {
            let span = span_to_py(py, o.span)?;
            let key = spanned_str_to_py(py, o.key, file)?;
            let value = spanned_str_to_py(py, o.value, file)?;
            PyOption {
                span,
                file: file.clone_ref(py),
                key,
                value,
            }
            .into_py_any(py)?
        }
        ast::Directive::Include(i) => {
            let span = span_to_py(py, i.span)?;
            let filename = spanned_str_to_py(py, i.filename, file)?;
            PyInclude {
                span,
                file: file.clone_ref(py),
                filename,
            }
            .into_py_any(py)?
        }
        ast::Directive::Plugin(p) => {
            let span = span_to_py(py, p.span)?;
            let name = spanned_str_to_py(py, p.name, file)?;
            let config = match p.config {
                Some(v) => Some(spanned_str_to_py(py, v, file)?),
                None => None,
            };
            PyPlugin {
                span,
                file: file.clone_ref(py),
                name,
                config,
            }
            .into_py_any(py)?
        }
        ast::Directive::PushTag(t) => {
            let span = span_to_py(py, t.span)?;
            let tag = spanned_str_to_py(py, t.tag, file)?;
            PyTagDirective {
                span,
                file: file.clone_ref(py),
                tag,
                action: "Push".to_owned(),
            }
            .into_py_any(py)?
        }
        ast::Directive::PopTag(t) => {
            let span = span_to_py(py, t.span)?;
            let tag = spanned_str_to_py(py, t.tag, file)?;
            PyTagDirective {
                span,
                file: file.clone_ref(py),
                tag,
                action: "Pop".to_owned(),
            }
            .into_py_any(py)?
        }
        ast::Directive::PushMeta(pm) => {
            let span = span_to_py(py, pm.span)?;
            let key = spanned_str_to_py(py, pm.key, file)?;
            let value = match pm.value {
                Some(v) => Some(spanned_key_value_value_to_py(py, v, file)?),
                None => None,
            };
            PyPushMeta {
                span,
                file: file.clone_ref(py),
                key,
                value,
            }
            .into_py_any(py)?
        }
        ast::Directive::PopMeta(pm) => {
            let span = span_to_py(py, pm.span)?;
            let key = spanned_str_to_py(py, pm.key, file)?;
            PyPopMeta {
                span,
                file: file.clone_ref(py),
                key,
            }
            .into_py_any(py)?
        }
        ast::Directive::Comment(c) => {
            let span = span_to_py(py, c.span)?;
            let text = spanned_str_to_py(py, c.text, file)?;
            PyComment {
                span,
                file: file.clone_ref(py),
                text,
            }
            .into_py_any(py)?
        }
        ast::Directive::Headline(h) => {
            let span = span_to_py(py, h.span)?;
            let text = spanned_str_to_py(py, h.text, file)?;
            PyHeadline {
                span,
                file: file.clone_ref(py),
                text,
            }
            .into_py_any(py)?
        }
        ast::Directive::Raw(r) => {
            let span = span_to_py(py, r.span)?;
            PyRaw {
                span,
                file: file.clone_ref(py),
                text: r.text.to_owned(),
            }
            .into_py_any(py)?
        }
    };

    Ok(obj)
}

// --- Dump helpers ---
fn slice_by_span(source: &str, start: usize, end: usize) -> PyResult<String> {
    if start > end {
        return Err(PyValueError::new_err(format!(
            "invalid span: start {} > end {}",
            start, end
        )));
    }

    let len = source.len();
    if end > len {
        return Err(PyValueError::new_err(format!(
            "span end {} exceeds source length {}",
            end, len
        )));
    }

    if !source.is_char_boundary(start) || !source.is_char_boundary(end) {
        return Err(PyValueError::new_err(
            "span boundaries are not aligned to char boundaries",
        ));
    }

    Ok(source[start..end].to_owned())
}

fn dump_span_from_file(py: Python<'_>, file: &Py<PyFile>, span: &PySpan) -> PyResult<String> {
    let file_ref = file.bind(py);
    let file_borrow = file_ref.borrow();
    slice_by_span(&file_borrow.content, span.start, span.end)
}

macro_rules! impl_dump_via_span_field {
    ($($ty:ident),* $(,)?) => {
        $(
            #[pymethods]
            impl $ty {
                fn dump(&self, py: Python<'_>) -> PyResult<String> {
                    let span = self.span.bind(py).borrow();
                    dump_span_from_file(py, &self.file, &span)
                }
            }
        )*
    };
}

impl_dump_via_span_field!(
    PySpannedStr,
    PySpannedBool,
    PySpannedKeyValueValue,
    PySpannedBinaryOp,
    PyNumberExpr,
    PyKeyValue,
    PyPosting,
    PyOpen,
    PyClose,
    PyBalance,
    PyPad,
    PyTransaction,
    PyCommodity,
    PyPrice,
    PyEvent,
    PyQuery,
    PyNote,
    PyDocument,
    PyCustom,
    PyOption,
    PyInclude,
    PyPlugin,
    PyTagDirective,
    PyPushMeta,
    PyPopMeta,
    PyComment,
    PyHeadline,
    PyRaw,
    PySpannedPriceOperator,
);

#[pymethods]
impl PyAmount {
    fn dump(&self, py: Python<'_>) -> PyResult<String> {
        let raw = self.raw.bind(py);
        let raw = raw.borrow();
        let span = raw.span.bind(py).borrow();
        dump_span_from_file(py, &raw.file, &span)
    }
}

#[pymethods]
impl PyCostSpec {
    fn dump(&self, py: Python<'_>) -> PyResult<String> {
        let raw = self.raw.bind(py);
        let raw = raw.borrow();
        let span = raw.span.bind(py).borrow();
        dump_span_from_file(py, &raw.file, &span)
    }
}

#[pymethods]
impl PyCustomValue {
    fn dump(&self, py: Python<'_>) -> PyResult<String> {
        let raw = self.raw.bind(py);
        let raw = raw.borrow();
        let span = raw.span.bind(py).borrow();
        dump_span_from_file(py, &raw.file, &span)
    }
}

// --- Python API ---
#[pyfunction]
#[pyo3(signature = (content, filename = "<string>"))]
fn parse_string(py: Python<'_>, content: &str, filename: &str) -> PyResult<Py<PyFile>> {
    let directives =
        parse_str(content, filename).map_err(|err| PyValueError::new_err(err.to_string()))?;

    let file = Py::new(
        py,
        PyFile {
            filename: filename.to_owned(),
            content: content.to_owned(),
            directives: Vec::with_capacity(directives.len()),
        },
    )?;

    let mut py_directives = Vec::with_capacity(directives.len());
    for directive in directives {
        py_directives.push(directive_to_py(py, directive, &file)?);
    }

    {
        let mut file_ref = file.bind(py).borrow_mut();
        file_ref.directives = py_directives;
    }

    Ok(file)
}

#[pyfunction]
#[pyo3(signature = (filename))]
fn parse_file(py: Python<'_>, filename: &str) -> PyResult<Py<PyFile>> {
    let content = std::fs::read_to_string(filename)
        .map_err(|err| PyValueError::new_err(format!("failed to read {}: {}", filename, err)))?;
    parse_string(py, &content, filename)
}
