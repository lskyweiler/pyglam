#[cfg(feature = "pyo3")]
fn main() -> pyo3_stub_gen::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().filter_or("RUST_LOG", "info")).init();
    let stub = pyglam::stub_info()?;
    stub.generate()?;
    Ok(())
}
#[cfg(not(feature = "pyo3"))]
fn main() -> Result<(), ()> {
    Ok(())
}
