fn main() -> pyo3_stub_gen::Result<()> {
    // `stub_info` is defined in the library crate by `define_stub_info_gatherer!`.
    let stub = beancount_ast_py::stub_info()?;
    stub.generate()?;
    Ok(())
}
