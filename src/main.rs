use pyo3::prelude::*;
use pyo3::types::PyCFunction;
use pyo3::types::PyModule;
use pyo3::wrap_pyfunction;
use pyo3::wrap_pymodule;


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
    pyo3_asyncio::tokio::cancellable_future_into_py(py, async move {
        for i in 0..duration {
            println!("Walking tick: {}", i);
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
        Ok(Python::with_gil(|py| py.None()))
    })
}

#[pyfunction]
fn get_character(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::cancellable_future_into_py(py, async move {
        Ok(Python::with_gil(|py| {
            let obj: Py<_> = Py::new(py, (LivingEntity { hp: 1, max_hp: 100, mp: 68, max_mp: 111, level: 20 }, Entity { x: 11, y: 22, name: String::from("Nice") })).unwrap().to_object(py);
            obj.to_object(py)
        }))
    })
}

fn register_module_to_sys(py: Python, path: &str, module: &PyModule) -> PyResult<()> {
    let sys = py.import("sys")?;
    let modules = sys.getattr("modules").unwrap();
    modules.set_item(path, module)
}

fn register_async_fn(py: Python, module: &PyModule, fun: &PyCFunction, name: &str) -> PyResult<()> {
    let wrap_rust_future = py.import("chrono_api.wrap_rust_future")?;
    let wrap_rust_future_func = wrap_rust_future.getattr("wrap_rust_future")?;
    module.setattr(name, wrap_rust_future_func.call1((fun,))?)
}

#[pyo3_asyncio::tokio::main]
async fn main() -> PyResult<()> {

    let fut = Python::with_gil(|py| {
        let chrono_api = PyModule::new(py, "chrono_api")?;
        let wrap_rust_future = PyModule::from_code(py, &String::from_utf8(include_bytes!("./chrono_api/wrap_rust_future.py").to_vec())?, "wrap_rust_future.py", "wrap_rust_future")?;
        
        chrono_api.add_submodule(wrap_rust_future)?;
        register_module_to_sys(py, "chrono_api.wrap_rust_future", wrap_rust_future)?;
     
        let core = PyModule::new(py, "core")?;

        chrono_api.add_submodule(core)?;

        register_module_to_sys(py, "chrono_api", chrono_api)?;
        register_module_to_sys(py, "chrono_api.core", chrono_api.getattr("core")?.downcast()?)?;

        register_async_fn(py, core, wrap_pyfunction!(walk, core)?, "walk")?;
        register_async_fn(py, core, wrap_pyfunction!(get_character, core)?, "get_character")?;

        let utils = PyModule::from_code(py, &String::from_utf8(include_bytes!("./chrono_api/utils.py").to_vec())?, "utils.py", "utils")?;
        let script = PyModule::from_code(py, &String::from_utf8(include_bytes!("./chrono_api/script.py").to_vec())?, "script.py", "script")?;
        
        chrono_api.add_submodule(utils)?;
        chrono_api.add_submodule(script)?;

        register_module_to_sys(py, "chrono_api.utils", chrono_api.getattr("utils")?.downcast()?)?;
        register_module_to_sys(py, "chrono_api.script", chrono_api.getattr("script")?.downcast()?)?;

        let script_module = PyModule::from_code(py, &String::from_utf8(include_bytes!("./script.py").to_vec())?, "script.py", "script")?;
        let script = script_module.getattr("Script")?.call0()?;
        
        pyo3_asyncio::tokio::into_future(script.call_method0("execute")?)
    })?;

    // let handle1 = tokio::spawn(fut);
    let _ = tokio::time::timeout(std::time::Duration::from_millis(3000), fut).await;
    println!("Canceled");
    // handle1.await.unwrap()?;
    
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    Ok(())
}
