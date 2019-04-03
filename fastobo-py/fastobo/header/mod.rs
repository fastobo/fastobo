pub mod clause;
pub mod frame;

#[doc(hidden)]
pub use self::frame::HeaderFrame;
#[doc(hidden)]
pub use self::clause::HeaderClause;


use pyo3::prelude::*;

pub fn module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<self::frame::HeaderFrame>()?;
    m.add_class::<self::clause::HeaderClause>()?;
    m.add_class::<self::clause::FormatVersionClause>()?;
    m.add_class::<self::clause::DataVersionClause>()?;
    Ok(())
}
