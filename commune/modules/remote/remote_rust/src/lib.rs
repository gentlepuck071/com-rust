use rand::Rng;
use pyo3::prelude::*;

#[pyfunction]
pub fn c_random_color() -> String {
    let colors = c_colors();
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..colors.len());
    colors[index].to_string()
}
#[pyfunction]
pub fn c_colors() -> Vec<&'static str> {
    vec![
        "black", "red", "green", "yellow", "blue", "magenta", "cyan", "white", 
        "bright_black", "bright_red", "bright_green", "bright_yellow", "bright_blue", 
        "bright_magenta", "bright_cyan", "bright_white"
    ]
}

#[pymodule]
fn c_random_color_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(c_random_color, m)?)?;
    Ok(())
}
