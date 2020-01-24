/// A trait to quickly find a single byte needle into a given type.
///
/// Uses [`memchr`](https://docs.rs) when compiled with the `memchr` feature
/// enabled.
pub trait QuickFind {
    fn quickfind(&self, needle: u8) -> Option<usize>;
    fn quickrfind(&self, needle: u8) -> Option<usize>;
    fn quickcount(&self, needle: u8) -> usize;
}

#[cfg(not(feature = "memchr"))]
impl<T: AsRef<str>> QuickFind for T {
    fn quickfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().find(char::from(needle))
    }

    fn quickrfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().rfind(char::from(needle))
    }

    fn quickcount(&self, needle: u8) -> usize {
        let c = char::from(needle);
        self.as_ref().chars().filter(|&x| x == c).count()
    }
}

#[cfg(feature = "memchr")]
impl<T: AsRef<str>> QuickFind for T {
    fn quickfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memchr(needle, self.as_ref().as_bytes())
    }

    fn quickrfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memrchr(needle, self.as_ref().as_bytes())
    }

    fn quickcount(&self, needle: u8) -> usize {
        ::memchr::memchr_iter(needle, self.as_ref().as_bytes()).count()
    }
}

#[cfg(not(feature = "memchr"))]
impl<'a> QuickFind for [u8] {
    fn quickfind(&self, needle: u8) -> Option<usize> {
        self.iter().position(|&c| c == needle)
    }

    fn quickrfind(&self, needle: u8) -> Option<usize> {
        self.iter().rposition(|&c| c == needle)
    }

    fn quickcount(&self, needle: u8) -> usize {
        self.iter().filter(|&&c| c == needle).count()
    }
}

#[cfg(feature = "memchr")]
impl<'a> QuickFind for [u8] {
    fn quickfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memchr(needle, self)
    }

    fn quickrfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memrchr(needle, self)
    }

    fn quickcount(&self, needle: u8) -> usize {
        ::memchr::memchr_iter(needle, self).count()
    }
}
