use pyo3::Python;
use pyo3::PyTypeInfo;
use pyo3::ffi::PyObject;


pub trait AsGILRef<'p, T> {
    fn as_ref(&'p self, py: Python<'p>) -> T;
}


pub unsafe fn ptr_to_ref<'p, T>(py: Python<'p>, t: *mut PyObject) -> &'p T
where
    T: PyTypeInfo,
{
    &*((t as *mut u8).offset(T::OFFSET) as *mut _ as *const _)
}
