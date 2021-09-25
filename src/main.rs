use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule, PyType};
use pyo3::py_run;
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

        let locals = PyDict::new(py);
        locals.set_item("chrono_api", chrono_api)?;
        
        let globals = py.import("__main__")?.dict();
        globals.set_item("chrono_api", chrono_api)?;

        py.run(&String::from_utf8(include_bytes!("./script.py").to_vec())?, Some(globals), None)?;
        let script = py.eval("SCRIPT()", Some(globals), None)?;

        locals.set_item("__script_instance", script)?;

        PyResult::Ok(vec![
            pyo3_asyncio::tokio::into_future(py.eval("__script_instance.run()", None, Some(locals))?)?,
            pyo3_asyncio::tokio::into_future(py.eval("__script_instance.run()", None, Some(locals))?)?,
            pyo3_asyncio::tokio::into_future(py.eval("__script_instance.run()", None, Some(locals))?)?,
            pyo3_asyncio::tokio::into_future(py.eval("__script_instance.run()", None, Some(locals))?)?,
        ])
    })?;

    futures::future::join_all(fut).await;

    Ok(())
}
