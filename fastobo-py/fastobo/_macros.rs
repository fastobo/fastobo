macro_rules! extend_slice_lifetime {
    ($slice:expr) => (unsafe {
        let r = $slice;
        std::slice::from_raw_parts(r.as_ptr(), r.len())
    };);
}
