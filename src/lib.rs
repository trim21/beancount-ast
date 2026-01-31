#![allow(clippy::large_enum_variant, clippy::too_many_arguments)]
#![deny(clippy::unwrap_used, clippy::expect_used)]

use beancount_parser::ast;
use beancount_parser::parse_str;
use pyderive::*;
use pyo3::IntoPyObjectExt;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyModule};

#[pymodule(name = "_ast")]
fn _ast(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    m.add("__version__", VERSION)?;

    // Core building blocks
    m.add_class::<PySpan>()?;
    m.add_class::<PyMeta>()?;
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

    // API
    m.add_function(wrap_pyfunction!(parse_string, m)?)?;
    m.add_function(wrap_pyfunction!(parse_file, m)?)?;
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PyNew, PyRepr, PyStr, PyEq)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Span", get_all)]
struct PySpan {
    start: usize,
    end: usize,
}

#[derive(Clone, PartialEq, Eq, Hash, PyNew, PyRepr, PyStr, PyEq)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Meta", get_all)]
struct PyMeta {
    filename: String,
    line: usize,
    column: usize,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedStr", get_all)]
struct PySpannedStr {
    span: Py<PySpan>,
    content: String,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedBool", get_all)]
struct PySpannedBool {
    span: Py<PySpan>,
    content: bool,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "KeyValueValue", get_all)]
struct PyKeyValueValue {
    kind: String,
    string: Option<String>,
    boolean: Option<bool>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedKeyValueValue", get_all)]
struct PySpannedKeyValueValue {
    span: Py<PySpan>,
    content: Py<PyKeyValueValue>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "KeyValue", get_all)]
struct PyKeyValue {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    key: Py<PySpannedStr>,
    value: Option<Py<PySpannedKeyValueValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedBinaryOp", get_all)]
struct PySpannedBinaryOp {
    span: Py<PySpan>,
    content: String,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "NumberExpr", get_all)]
struct PyNumberExpr {
    kind: String,
    span: Py<PySpan>,
    literal: Option<Py<PySpannedStr>>,
    left: Option<Py<PyNumberExpr>>,
    op: Option<Py<PySpannedBinaryOp>>,
    right: Option<Py<PyNumberExpr>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Amount", get_all)]
struct PyAmount {
    raw: Py<PySpannedStr>,
    number: Py<PyNumberExpr>,
    currency: Option<Py<PySpannedStr>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "CostAmount", get_all)]
struct PyCostAmount {
    per: Option<Py<PyNumberExpr>>,
    total: Option<Py<PyNumberExpr>>,
    currency: Option<Py<PySpannedStr>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
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
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "SpannedPriceOperator", get_all)]
struct PySpannedPriceOperator {
    span: Py<PySpan>,
    content: String,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Posting", get_all)]
struct PyPosting {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
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
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "CustomValue", get_all)]
struct PyCustomValue {
    raw: Py<PySpannedStr>,
    kind: String,
    number: Option<Py<PyNumberExpr>>,
    amount: Option<Py<PyAmount>>,
}

// --- Directives ---

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Open", get_all)]
struct PyOpen {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    currencies: Vec<Py<PySpannedStr>>,
    opt_booking: Option<Py<PySpannedStr>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Close", get_all)]
struct PyClose {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Balance", get_all)]
struct PyBalance {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    amount: Py<PyAmount>,
    tolerance: Option<Py<PySpannedStr>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Pad", get_all)]
struct PyPad {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    from_account: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Transaction", get_all)]
struct PyTransaction {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    txn: Option<Py<PySpannedStr>>,
    payee: Option<Py<PySpannedStr>>,
    narration: Option<Py<PySpannedStr>>,
    tags_links: Option<Py<PySpannedStr>>,
    tags: Vec<Py<PySpannedStr>>,
    links: Vec<Py<PySpannedStr>>,
    comment: Option<Py<PySpannedStr>>,
    tags_links_lines: Vec<Py<PySpannedStr>>,
    comments: Vec<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
    postings: Vec<Py<PyPosting>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Commodity", get_all)]
struct PyCommodity {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    currency: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Price", get_all)]
struct PyPrice {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    currency: Py<PySpannedStr>,
    amount: Py<PyAmount>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Event", get_all)]
struct PyEvent {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    event_type: Py<PySpannedStr>,
    desc: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Query", get_all)]
struct PyQuery {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    name: Py<PySpannedStr>,
    query: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Note", get_all)]
struct PyNote {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    note: Py<PySpannedStr>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Document", get_all)]
struct PyDocument {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    account: Py<PySpannedStr>,
    filename: Py<PySpannedStr>,
    tags_links: Option<Py<PySpannedStr>>,
    tags: Vec<Py<PySpannedStr>>,
    links: Vec<Py<PySpannedStr>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Custom", get_all)]
struct PyCustom {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    date: Py<PySpannedStr>,
    name: Py<PySpannedStr>,
    values: Vec<Py<PyCustomValue>>,
    comment: Option<Py<PySpannedStr>>,
    key_values: Vec<Py<PyKeyValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Option", get_all)]
struct PyOption {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    key: Py<PySpannedStr>,
    value: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Include", get_all)]
struct PyInclude {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    filename: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Plugin", get_all)]
struct PyPlugin {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    name: Py<PySpannedStr>,
    config: Option<Py<PySpannedStr>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Tag", get_all)]
struct PyTagDirective {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    tag: Py<PySpannedStr>,
    action: String,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "PushMeta", get_all)]
struct PyPushMeta {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    key: Py<PySpannedStr>,
    value: Option<Py<PySpannedKeyValueValue>>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "PopMeta", get_all)]
struct PyPopMeta {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    key: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Comment", get_all)]
struct PyComment {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    text: Py<PySpannedStr>,
}

#[derive(PyNew, PyRepr, PyStr)]
#[cfg_attr(feature = "stub-gen", pyo3_stub_gen::derive::gen_stub_pyclass)]
#[pyclass(module = "beancount_ast._ast", name = "Headline", get_all)]
struct PyHeadline {
    meta: Py<PyMeta>,
    span: Py<PySpan>,
    text: Py<PySpannedStr>,
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

fn meta_to_py(py: Python<'_>, meta: ast::Meta) -> PyResult<Py<PyMeta>> {
    Py::new(
        py,
        PyMeta {
            filename: meta.filename,
            line: meta.line,
            column: meta.column,
        },
    )
}

fn spanned_str_to_py(py: Python<'_>, ws: ast::WithSpan<&str>) -> PyResult<Py<PySpannedStr>> {
    let span = span_to_py(py, ws.span)?;
    Py::new(
        py,
        PySpannedStr {
            span,
            content: ws.content.to_owned(),
        },
    )
}

fn spanned_bool_to_py(py: Python<'_>, ws: ast::WithSpan<bool>) -> PyResult<Py<PySpannedBool>> {
    let span = span_to_py(py, ws.span)?;
    Py::new(
        py,
        PySpannedBool {
            span,
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
) -> PyResult<Py<PySpannedKeyValueValue>> {
    let span = span_to_py(py, ws.span)?;
    let content = key_value_value_to_py(py, ws.content)?;
    Py::new(py, PySpannedKeyValueValue { span, content })
}

fn key_value_to_py(py: Python<'_>, kv: ast::KeyValue<'_>) -> PyResult<Py<PyKeyValue>> {
    let meta = meta_to_py(py, kv.meta)?;
    let span = span_to_py(py, kv.span)?;
    let key = spanned_str_to_py(py, kv.key)?;
    let value = match kv.value {
        Some(v) => Some(spanned_key_value_value_to_py(py, v)?),
        None => None,
    };

    Py::new(
        py,
        PyKeyValue {
            meta,
            span,
            key,
            value,
        },
    )
}

fn spanned_binary_op_to_py(
    py: Python<'_>,
    ws: ast::WithSpan<ast::BinaryOp>,
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
            content: content.to_owned(),
        },
    )
}

fn number_expr_to_py(py: Python<'_>, expr: ast::NumberExpr<'_>) -> PyResult<Py<PyNumberExpr>> {
    match expr {
        ast::NumberExpr::Missing { span } => {
            let span = span_to_py(py, span)?;
            Py::new(
                py,
                PyNumberExpr {
                    kind: "Missing".to_owned(),
                    span,
                    literal: None,
                    left: None,
                    op: None,
                    right: None,
                },
            )
        }
        ast::NumberExpr::Literal(ws) => {
            let span = span_to_py(py, ws.span)?;
            let literal = Some(spanned_str_to_py(py, ws)?);
            Py::new(
                py,
                PyNumberExpr {
                    kind: "Literal".to_owned(),
                    span,
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
            let left = Some(number_expr_to_py(py, *left)?);
            let op = Some(spanned_binary_op_to_py(py, op)?);
            let right = Some(number_expr_to_py(py, *right)?);
            Py::new(
                py,
                PyNumberExpr {
                    kind: "Binary".to_owned(),
                    span,
                    literal: None,
                    left,
                    op,
                    right,
                },
            )
        }
    }
}

fn amount_to_py(py: Python<'_>, amt: ast::Amount<'_>) -> PyResult<Py<PyAmount>> {
    let raw = spanned_str_to_py(py, amt.raw)?;
    let number = number_expr_to_py(py, amt.number)?;
    let currency = match amt.currency {
        Some(c) => Some(spanned_str_to_py(py, c)?),
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

fn cost_amount_to_py(py: Python<'_>, ca: ast::CostAmount<'_>) -> PyResult<Py<PyCostAmount>> {
    let per = match ca.per {
        Some(p) => Some(number_expr_to_py(py, p)?),
        None => None,
    };
    let total = match ca.total {
        Some(t) => Some(number_expr_to_py(py, t)?),
        None => None,
    };
    let currency = match ca.currency {
        Some(c) => Some(spanned_str_to_py(py, c)?),
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

fn cost_spec_to_py(py: Python<'_>, cs: ast::CostSpec<'_>) -> PyResult<Py<PyCostSpec>> {
    let raw = spanned_str_to_py(py, cs.raw)?;
    let amount = match cs.amount {
        Some(a) => Some(cost_amount_to_py(py, a)?),
        None => None,
    };
    let date = match cs.date {
        Some(d) => Some(spanned_str_to_py(py, d)?),
        None => None,
    };
    let label = match cs.label {
        Some(l) => Some(spanned_str_to_py(py, l)?),
        None => None,
    };
    let merge = match cs.merge {
        Some(m) => Some(spanned_bool_to_py(py, m)?),
        None => None,
    };
    let is_total = spanned_bool_to_py(py, cs.is_total)?;
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
            content: content.to_owned(),
        },
    )
}

fn posting_to_py(py: Python<'_>, p: ast::Posting<'_>) -> PyResult<Py<PyPosting>> {
    let meta = meta_to_py(py, p.meta)?;
    let span = span_to_py(py, p.span)?;
    let opt_flag = match p.opt_flag {
        Some(f) => Some(spanned_str_to_py(py, f)?),
        None => None,
    };
    let account = spanned_str_to_py(py, p.account)?;
    let amount = match p.amount {
        Some(a) => Some(amount_to_py(py, a)?),
        None => None,
    };
    let cost_spec = match p.cost_spec {
        Some(cs) => Some(cost_spec_to_py(py, cs)?),
        None => None,
    };
    let price_operator = match p.price_operator {
        Some(po) => Some(spanned_price_operator_to_py(py, po)?),
        None => None,
    };
    let price_annotation = match p.price_annotation {
        Some(pa) => Some(amount_to_py(py, pa)?),
        None => None,
    };
    let comment = match p.comment {
        Some(c) => Some(spanned_str_to_py(py, c)?),
        None => None,
    };
    let mut key_values = Vec::with_capacity(p.key_values.len());
    for kv in p.key_values {
        key_values.push(key_value_to_py(py, kv)?);
    }

    Py::new(
        py,
        PyPosting {
            meta,
            span,
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

fn custom_value_to_py(py: Python<'_>, v: ast::CustomValue<'_>) -> PyResult<Py<PyCustomValue>> {
    let raw = spanned_str_to_py(py, v.raw)?;
    let kind = match v.kind {
        ast::CustomValueKind::String => "String",
        ast::CustomValueKind::Date => "Date",
        ast::CustomValueKind::Bool => "Bool",
        ast::CustomValueKind::Amount => "Amount",
        ast::CustomValueKind::Number => "Number",
        ast::CustomValueKind::Account => "Account",
    };
    let number = match v.number {
        Some(n) => Some(number_expr_to_py(py, n)?),
        None => None,
    };
    let amount = match v.amount {
        Some(a) => Some(amount_to_py(py, a)?),
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

fn directive_to_py(py: Python<'_>, d: ast::Directive<'_>) -> PyResult<Py<PyAny>> {
    let obj: Py<PyAny> = match d {
        ast::Directive::Open(o) => {
            let meta = meta_to_py(py, o.meta)?;
            let span = span_to_py(py, o.span)?;
            let date = spanned_str_to_py(py, o.date)?;
            let account = spanned_str_to_py(py, o.account)?;
            let currencies = o
                .currencies
                .into_iter()
                .map(|c| spanned_str_to_py(py, c))
                .collect::<PyResult<Vec<_>>>()?;
            let opt_booking = match o.opt_booking {
                Some(b) => Some(spanned_str_to_py(py, b)?),
                None => None,
            };
            let comment = match o.comment {
                Some(c) => Some(spanned_str_to_py(py, c)?),
                None => None,
            };
            let key_values = o
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;

            PyOpen {
                meta,
                span,
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
            let meta = meta_to_py(py, c.meta)?;
            let span = span_to_py(py, c.span)?;
            let date = spanned_str_to_py(py, c.date)?;
            let account = spanned_str_to_py(py, c.account)?;
            let comment = match c.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = c
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyClose {
                meta,
                span,
                date,
                account,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Balance(b) => {
            let meta = meta_to_py(py, b.meta)?;
            let span = span_to_py(py, b.span)?;
            let date = spanned_str_to_py(py, b.date)?;
            let account = spanned_str_to_py(py, b.account)?;
            let amount = amount_to_py(py, b.amount)?;
            let tolerance = match b.tolerance {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let comment = match b.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = b
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyBalance {
                meta,
                span,
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
            let meta = meta_to_py(py, p.meta)?;
            let span = span_to_py(py, p.span)?;
            let date = spanned_str_to_py(py, p.date)?;
            let account = spanned_str_to_py(py, p.account)?;
            let from_account = spanned_str_to_py(py, p.from_account)?;
            let comment = match p.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = p
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyPad {
                meta,
                span,
                date,
                account,
                from_account,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Transaction(t) => {
            let meta = meta_to_py(py, t.meta)?;
            let span = span_to_py(py, t.span)?;
            let date = spanned_str_to_py(py, t.date)?;
            let txn = match t.txn {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let payee = match t.payee {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let narration = match t.narration {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let tags_links = match t.tags_links {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let tags = t
                .tags
                .into_iter()
                .map(|s| spanned_str_to_py(py, s))
                .collect::<PyResult<Vec<_>>>()?;
            let links = t
                .links
                .into_iter()
                .map(|s| spanned_str_to_py(py, s))
                .collect::<PyResult<Vec<_>>>()?;
            let comment = match t.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let tags_links_lines = t
                .tags_links_lines
                .into_iter()
                .map(|s| spanned_str_to_py(py, s))
                .collect::<PyResult<Vec<_>>>()?;
            let comments = t
                .comments
                .into_iter()
                .map(|s| spanned_str_to_py(py, s))
                .collect::<PyResult<Vec<_>>>()?;
            let key_values = t
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            let postings = t
                .postings
                .into_iter()
                .map(|p| posting_to_py(py, p))
                .collect::<PyResult<Vec<_>>>()?;

            PyTransaction {
                meta,
                span,
                date,
                txn,
                payee,
                narration,
                tags_links,
                tags,
                links,
                comment,
                tags_links_lines,
                comments,
                key_values,
                postings,
            }
            .into_py_any(py)?
        }
        ast::Directive::Commodity(c) => {
            let meta = meta_to_py(py, c.meta)?;
            let span = span_to_py(py, c.span)?;
            let date = spanned_str_to_py(py, c.date)?;
            let currency = spanned_str_to_py(py, c.currency)?;
            let comment = match c.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = c
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyCommodity {
                meta,
                span,
                date,
                currency,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Price(p) => {
            let meta = meta_to_py(py, p.meta)?;
            let span = span_to_py(py, p.span)?;
            let date = spanned_str_to_py(py, p.date)?;
            let currency = spanned_str_to_py(py, p.currency)?;
            let amount = amount_to_py(py, p.amount)?;
            let comment = match p.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = p
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyPrice {
                meta,
                span,
                date,
                currency,
                amount,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Event(e) => {
            let meta = meta_to_py(py, e.meta)?;
            let span = span_to_py(py, e.span)?;
            let date = spanned_str_to_py(py, e.date)?;
            let event_type = spanned_str_to_py(py, e.event_type)?;
            let desc = spanned_str_to_py(py, e.desc)?;
            let comment = match e.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = e
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyEvent {
                meta,
                span,
                date,
                event_type,
                desc,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Query(q) => {
            let meta = meta_to_py(py, q.meta)?;
            let span = span_to_py(py, q.span)?;
            let date = spanned_str_to_py(py, q.date)?;
            let name = spanned_str_to_py(py, q.name)?;
            let query = spanned_str_to_py(py, q.query)?;
            let comment = match q.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = q
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyQuery {
                meta,
                span,
                date,
                name,
                query,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Note(n) => {
            let meta = meta_to_py(py, n.meta)?;
            let span = span_to_py(py, n.span)?;
            let date = spanned_str_to_py(py, n.date)?;
            let account = spanned_str_to_py(py, n.account)?;
            let note = spanned_str_to_py(py, n.note)?;
            let comment = match n.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = n
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyNote {
                meta,
                span,
                date,
                account,
                note,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Document(d) => {
            let meta = meta_to_py(py, d.meta)?;
            let span = span_to_py(py, d.span)?;
            let date = spanned_str_to_py(py, d.date)?;
            let account = spanned_str_to_py(py, d.account)?;
            let filename = spanned_str_to_py(py, d.filename)?;
            let tags_links = match d.tags_links {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let tags = d
                .tags
                .into_iter()
                .map(|s| spanned_str_to_py(py, s))
                .collect::<PyResult<Vec<_>>>()?;
            let links = d
                .links
                .into_iter()
                .map(|s| spanned_str_to_py(py, s))
                .collect::<PyResult<Vec<_>>>()?;
            let comment = match d.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = d
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyDocument {
                meta,
                span,
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
            let meta = meta_to_py(py, c.meta)?;
            let span = span_to_py(py, c.span)?;
            let date = spanned_str_to_py(py, c.date)?;
            let name = spanned_str_to_py(py, c.name)?;
            let values = c
                .values
                .into_iter()
                .map(|v| custom_value_to_py(py, v))
                .collect::<PyResult<Vec<_>>>()?;
            let comment = match c.comment {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            let key_values = c
                .key_values
                .into_iter()
                .map(|kv| key_value_to_py(py, kv))
                .collect::<PyResult<Vec<_>>>()?;
            PyCustom {
                meta,
                span,
                date,
                name,
                values,
                comment,
                key_values,
            }
            .into_py_any(py)?
        }
        ast::Directive::Option(o) => {
            let meta = meta_to_py(py, o.meta)?;
            let span = span_to_py(py, o.span)?;
            let key = spanned_str_to_py(py, o.key)?;
            let value = spanned_str_to_py(py, o.value)?;
            PyOption {
                meta,
                span,
                key,
                value,
            }
            .into_py_any(py)?
        }
        ast::Directive::Include(i) => {
            let meta = meta_to_py(py, i.meta)?;
            let span = span_to_py(py, i.span)?;
            let filename = spanned_str_to_py(py, i.filename)?;
            PyInclude {
                meta,
                span,
                filename,
            }
            .into_py_any(py)?
        }
        ast::Directive::Plugin(p) => {
            let meta = meta_to_py(py, p.meta)?;
            let span = span_to_py(py, p.span)?;
            let name = spanned_str_to_py(py, p.name)?;
            let config = match p.config {
                Some(v) => Some(spanned_str_to_py(py, v)?),
                None => None,
            };
            PyPlugin {
                meta,
                span,
                name,
                config,
            }
            .into_py_any(py)?
        }
        ast::Directive::PushTag(t) => {
            let meta = meta_to_py(py, t.meta)?;
            let span = span_to_py(py, t.span)?;
            let tag = spanned_str_to_py(py, t.tag)?;
            PyTagDirective {
                meta,
                span,
                tag,
                action: "Push".to_owned(),
            }
            .into_py_any(py)?
        }
        ast::Directive::PopTag(t) => {
            let meta = meta_to_py(py, t.meta)?;
            let span = span_to_py(py, t.span)?;
            let tag = spanned_str_to_py(py, t.tag)?;
            PyTagDirective {
                meta,
                span,
                tag,
                action: "Pop".to_owned(),
            }
            .into_py_any(py)?
        }
        ast::Directive::PushMeta(pm) => {
            let meta = meta_to_py(py, pm.meta)?;
            let span = span_to_py(py, pm.span)?;
            let key = spanned_str_to_py(py, pm.key)?;
            let value = match pm.value {
                Some(v) => Some(spanned_key_value_value_to_py(py, v)?),
                None => None,
            };
            PyPushMeta {
                meta,
                span,
                key,
                value,
            }
            .into_py_any(py)?
        }
        ast::Directive::PopMeta(pm) => {
            let meta = meta_to_py(py, pm.meta)?;
            let span = span_to_py(py, pm.span)?;
            let key = spanned_str_to_py(py, pm.key)?;
            PyPopMeta { meta, span, key }.into_py_any(py)?
        }
        ast::Directive::Comment(c) => {
            let meta = meta_to_py(py, c.meta)?;
            let span = span_to_py(py, c.span)?;
            let text = spanned_str_to_py(py, c.text)?;
            PyComment { meta, span, text }.into_py_any(py)?
        }
        ast::Directive::Headline(h) => {
            let meta = meta_to_py(py, h.meta)?;
            let span = span_to_py(py, h.span)?;
            let text = spanned_str_to_py(py, h.text)?;
            PyHeadline { meta, span, text }.into_py_any(py)?
        }
    };

    Ok(obj)
}

// --- Python API ---
#[pyfunction]
#[pyo3(signature = (content, filename = "<string>"))]
fn parse_string(py: Python<'_>, content: &str, filename: &str) -> PyResult<Py<PyList>> {
    let directives =
        parse_str(content, filename).map_err(|err| PyValueError::new_err(err.to_string()))?;

    let out = PyList::empty(py);
    for directive in directives {
        out.append(directive_to_py(py, directive)?)?;
    }
    Ok(out.unbind())
}

#[pyfunction]
#[pyo3(signature = (filename))]
fn parse_file(py: Python<'_>, filename: &str) -> PyResult<Py<PyList>> {
    let content = std::fs::read_to_string(filename)
        .map_err(|err| PyValueError::new_err(format!("failed to read {}: {}", filename, err)))?;
    parse_string(py, &content, filename)
}

#[cfg(feature = "stub-gen")]
pyo3_stub_gen::module_variable!("beancount_ast", "__version__", String);

#[cfg(feature = "stub-gen")]
pyo3_stub_gen::derive::gen_type_alias_from_python! {
    "beancount_ast._ast",
    r#"
import builtins
from typing import TypeAlias

Directive: TypeAlias = (
    Open
    | Close
    | Balance
    | Pad
    | Transaction
    | Commodity
    | Price
    | Event
    | Query
    | Note
    | Document
    | Custom
    | Option
    | Include
    | Plugin
    | Tag
    | PushMeta
    | PopMeta
    | Comment
    | Headline
)
"#
}

#[cfg(feature = "stub-gen")]
pyo3_stub_gen::inventory::submit! {
  pyo3_stub_gen::derive::gen_function_from_python! {
    module = "beancount_ast._ast",
    r#"
def parse_string(content: builtins.str, filename: builtins.str = "<string>") -> builtins.list[Directive]: ...
"#
  }
}

#[cfg(feature = "stub-gen")]
pyo3_stub_gen::inventory::submit! {
  pyo3_stub_gen::derive::gen_function_from_python! {
    module = "beancount_ast._ast",
    r#"
import builtins

def parse_file(filename: builtins.str) -> builtins.list[Directive]: ...
"#
  }
}

// #[cfg(feature = "stub-gen")]
// pyo3_stub_gen::reexport_module_members!(
//   "beancount_ast",
//   // "beancount_ast._ast",
//   "__version__",
//   "Directive",
//   "parse_string",
//   "parse_file"
// );

#[cfg(feature = "stub-gen")]
pyo3_stub_gen::define_stub_info_gatherer!(stub_info);
