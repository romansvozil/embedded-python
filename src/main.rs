use pyo3::prelude::*;
use pyo3::types::PyDict;
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

#[pyfunction(name = "_walk")]
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
            let obj: Py<_> = Py::new(py, (LivingEntity { hp: 1, max_hp: 100, mp: 68, max_mp: 111, level: 20 }, Entity { x: 11, y: 22, name: String::from("Nice") })).unwrap();
            obj.to_object(py)
        }))
    })
}

fn register_module_to_sys(py: Python, path: &str, module: &PyModule) -> PyResult<()> {
    let sys = py.import("sys")?;
    let modules = sys.getattr("modules").unwrap();
    modules.set_item(path, module)
}

#[pyo3_asyncio::tokio::main]
async fn main() -> PyResult<()> {

    let fut = Python::with_gil(|py| {
        let chrono_api = PyModule::new(py, "chrono_api")?;

        let core = PyModule::new(py, "core")?;
        core.add_function(wrap_pyfunction!(walk, chrono_api)?)?;
        core.add_function(wrap_pyfunction!(get_character, chrono_api)?)?;
        
        chrono_api.add_submodule(core)?;
                
        register_module_to_sys(py, "chrono_api", chrono_api)?;
        register_module_to_sys(py, "chrono_api.core", chrono_api.getattr("core")?.downcast()?)?;

        let utils = PyModule::from_code(py, &String::from_utf8(include_bytes!("./chrono_api/utils.py").to_vec())?, "utils.py", "utils")?;
        chrono_api.add_submodule(utils)?;

        register_module_to_sys(py, "chrono_api.utils", chrono_api.getattr("utils")?.downcast()?)?;

        let script_module = PyModule::from_code(py, &String::from_utf8(include_bytes!("./script.py").to_vec())?, "script.py", "script")?;
        let script = script_module.getattr("SCRIPT")?.call0()?;
        pyo3_asyncio::tokio::into_future(script.call_method0("run")?)
    })?;

    let handle1 = tokio::spawn(fut);

    handle1.await.unwrap()?;
    
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;

    Ok(())
}
