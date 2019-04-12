pub mod clause;
pub mod frame;

use pyo3::prelude::*;

#[pymodule(typedef)]
pub fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::frame::TypedefFrame>()?;
    Ok(())
}
