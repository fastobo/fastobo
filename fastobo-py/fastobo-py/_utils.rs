use pyo3::AsPyPointer;
use pyo3::AsPyRef;
use pyo3::Python;
use pyo3::Py;
use pyo3::PyTypeInfo;
use pyo3::ffi::PyObject;


pub trait AsGILRef<'p, T>: 'p {
    fn as_gil_ref(&'p self, py: Python<'p>) -> T;
}


impl<'p, T> AsGILRef<'p, &'p T> for Py<T>
where
    T: PyTypeInfo,
{
    fn as_gil_ref(&'p self, py: Python<'p>) -> &'p T {
        unsafe { ptr_to_ref(py, self.as_ref(py).as_ptr()) }
    }
}


unsafe fn ptr_to_ref<'p, T>(_py: Python<'p>, t: *mut PyObject) -> &'p T
where
    T: PyTypeInfo,
{
    &*((t as *mut u8).offset(T::OFFSET) as *mut _ as *const _)
}
