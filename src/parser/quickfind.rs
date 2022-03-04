/// A trait to quickly find a single byte needle into a given type.
///
/// Uses [`memchr`](https://docs.rs) when compiled with the `memchr` feature
/// enabled.
pub trait QuickFind {
    fn quickfind(&self, needle: u8) -> Option<usize>;
    fn quickrfind(&self, needle: u8) -> Option<usize>;
    fn quickcount(&self, needle: u8) -> usize;
}

impl<T: AsRef<[u8]>> QuickFind for T {
    fn quickfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memchr(needle, self.as_ref())
    }

    fn quickrfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memrchr(needle, self.as_ref())
    }

    fn quickcount(&self, needle: u8) -> usize {
        ::memchr::memchr_iter(needle, self.as_ref()).count()
    }
}
