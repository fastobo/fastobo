pub trait QuickFind:  {
    fn quickfind(&self, needle: u8) -> Option<usize>;
    fn quickrfind(&self, needle: u8) -> Option<usize>;
    fn quickcount(&self, needle: u8) -> usize;
}

impl<T> QuickFind for T
where
    T: AsRef<str>,
{
    #[cfg(not(feature = "memchr"))]
    fn quickfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().find(char::from(needle))
    }

    #[cfg(not(feature = "memchr"))]
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        self.as_ref().rfind(char::from(needle))
    }

    #[cfg(not(feature = "memchr"))]
    fn quickcount(&self, needle: u8) -> usize {
        let c = char::from(needle);
        self.as_ref().chars().filter(|&x| x == c).count()
    }

    #[cfg(feature = "memchr")]
    fn quickfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memchr(needle, self.as_ref().as_bytes())
    }

    #[cfg(feature = "memchr")]
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memrchr(needle, self.as_ref().as_bytes())
    }

    #[cfg(feature = "memchr")]
    fn quickcount(&self, needle: u8) -> usize {
        ::memchr::memchr_iter(needle, self.as_ref().as_bytes()).count()
    }
}

impl<'a> QuickFind for [u8] {
    #[cfg(not(feature = "memchr"))]
    fn quickfind(&self, needle: u8) -> Option<usize> {
        self.iter().position(|&c| c == needle)
    }

    #[cfg(not(feature = "memchr"))]
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        self.iter().rposition(|&c| c == needle)
    }

    #[cfg(not(feature = "memchr"))]
    fn quickcount(&self, needle: u8) -> usize {
        self.iter().filter(|&&c| c == needle).count()
    }

    #[cfg(feature = "memchr")]
    fn quickfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memchr(needle, self)
    }

    #[cfg(feature = "memchr")]
    fn quickrfind(&self, needle: u8) -> Option<usize> {
        ::memchr::memrchr(needle, self)
    }

    #[cfg(feature = "memchr")]
    fn quickcount(&self, needle: u8) -> usize {
        ::memchr::memchr_iter(needle, self).count()
    }
}
