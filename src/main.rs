use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;


#[pyclass(subclass)]
struct Entity {
    #[pyo3(get)]
    x: i32,
    #[pyo3(get)]
    y: i32,
    #[pyo3(get)]
    name: String,
}

#[pymethods]
impl Entity {
    #[new]
    fn new() -> Self {
        Self { x: 11, y: 22, name: String::from("Nice") }
    }
}

#[pyclass(extends=Entity, subclass)]
struct LivingEntity {
    #[pyo3(get)]
    hp: i32,
    #[pyo3(get)]
    max_hp: i32,
    #[pyo3(get)]
    mp: i32,
    #[pyo3(get)]
    max_mp: i32,
    #[pyo3(get)]
    level: i32,
}

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

#[pyfunction]
fn get_character(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        Ok(Python::with_gil(|py| {
            let obj: Py<_> = Py::new(py, (LivingEntity { hp: 1, max_hp: 100, mp: 68, max_mp: 111, level: 20 }, Entity { x: 11, y: 22, name: String::from("Nice") })).unwrap();
            obj.to_object(py)
        }))
    })
}

#[pyo3_asyncio::tokio::main]
async fn main() -> PyResult<()> {

    let fut = Python::with_gil(|py| {
        let chrono_api = PyModule::new(py, "chrono_api")?;
        chrono_api.add_function(wrap_pyfunction!(walk, chrono_api)?)?;
        chrono_api.add_function(wrap_pyfunction!(get_character, chrono_api)?)?;
        chrono_api.add_class::<Entity>()?;

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
