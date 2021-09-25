use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;


#[pyfunction]
fn walk(py: Python, duration: i32) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        for i in 0..duration {
            println!("Walking tick: {}", i);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        Ok(Python::with_gil(|py| py.None()))
    })
}

#[pyo3_asyncio::tokio::main]
async fn main() -> PyResult<()> {

    let fut = Python::with_gil(|py| {
        let chrono_api = PyModule::new(py, "chrono_api")?;
        chrono_api.add_function(wrap_pyfunction!(walk, chrono_api)?)?;

        let globals = py.import("__main__")?.dict();
        globals.set_item("chrono_api", chrono_api)?;

        py.run(&String::from_utf8(include_bytes!("./script.py").to_vec())?, Some(globals), None)?;
        let script = py.eval("SCRIPT()", Some(globals), None)?;

        globals.set_item("__script_instance", script)?;

        pyo3_asyncio::tokio::into_future(py.eval("__script_instance.run()", Some(globals), None)?)
    })?;

    let handle = tokio::spawn(fut);

    handle.await.unwrap()?;

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    Ok(())
}
