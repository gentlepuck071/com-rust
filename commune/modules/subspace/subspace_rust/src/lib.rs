use pyo3::prelude::*;
use substrate_subxt::{ClientBuilder, DefaultNodeRuntime};

#[pyfunction]
fn create_substrate_interface(
    url: String,
    websocket: Option<String>,
    ss58_format: Option<u32>,
    type_registry: Option<String>,
    type_registry_preset: Option<String>,
    cache_region: Option<String>,
    runtime_config: Option<String>,
    ws_options: Option<String>,
    auto_discover: Option<bool>,
    auto_reconnect: Option<bool>,
) -> PyResult<()> {
    let substrate = ClientBuilder::<DefaultNodeRuntime>::new()
        .set_url(url)
        .set_websocket(websocket)
        .set_ss58_format(ss58_format)
        .set_type_registry(type_registry)
        .set_type_registry_preset(type_registry_preset)
        .set_cache_region(cache_region)
        .set_runtime_config(runtime_config)
        .set_ws_options(ws_options)
        .set_auto_discover(auto_discover)
        .set_auto_reconnect(auto_reconnect)
        .build();

    println!("{:?}", substrate);

    Ok(())
}

#[pymodule]
fn my_module(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_substrate_interface, m)?)?;

    Ok(())
}
