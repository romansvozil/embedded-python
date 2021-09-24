use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule, PyType};
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
        
        py.run(&String::from_utf8(include_bytes!("./script.py").to_vec())?, None, Some(locals))?;
        
        // let script_class = locals.get_item("SCRIPT").unwrap().downcast::<PyType>()?;
        // let script_instance = script_class.call1((chrono_api, ))?;
        println!("Locals: {:?}", locals);
        pyo3_asyncio::tokio::into_future(py.eval("SCRIPT(chrono_api, locals()).run()", None, Some(locals))?)

        // pyo3_asyncio::tokio::into_future(
        //     script_instance.call_method0("run")?)
    })?;

    fut.await?;

    Ok(())
}
