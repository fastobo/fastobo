pub mod clause;
pub mod frame;

use pyo3::prelude::*;

#[pymodule(term)]
pub fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::frame::TermFrame>()?;
    m.add_class::<self::clause::BaseTermClause>()?;
    Ok(())
}
