use pyo3::prelude::*;
use std::thread;
use pyo3::types::PyTuple;

#[pyfunction]
fn execute_python_function(python_function: PyObject, args: Vec<PyObject>) -> PyResult<()> {
    Python::with_gil(|py| {
        let args_tuple = PyTuple::new(py, args);

        match python_function.call1(py, args_tuple) {
            Ok(_) => println!("Python function execution completed"),
            Err(err) => eprintln!("Error calling Python function: {:?}", err),
        }
    });

    Ok(())
}

#[pyfunction]
fn create_thread(_py: Python, python_function: PyObject, args: Vec<PyObject>) -> PyResult<()> {
    println!("Start Thread Execution");

    let handle = thread::spawn(|| {
        execute_python_function(python_function, args).expect("Error calling Python function");
    });

    println!("Thread completed");
    Ok(())
}

#[pymodule]
fn rust_vali_thread(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_thread, m)?)?;
    Ok(())
}