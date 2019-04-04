pub mod clause;
pub mod frame;

use pyo3::prelude::*;

use self::clause::*;
use self::frame::*;

#[pymodule(header)]
pub fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::frame::HeaderFrame>()?;
    m.add_class::<self::clause::BaseHeaderClause>()?;
    m.add_class::<self::clause::FormatVersionClause>()?;
    m.add_class::<self::clause::DataVersionClause>()?;
    m.add_class::<self::clause::SubsetdefClause>()?;
    Ok(())
}
