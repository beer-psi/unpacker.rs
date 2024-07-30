use core::str::pattern::{Pattern, Searcher};

pub trait Substring {
    /// Returns a substring before the first occurence of pattern.
    fn substring_before<P: Pattern>(&self, pat: P) -> &str;

    /// Returns a substring after the first occurence of pattern.
    fn substring_after<P: Pattern>(&self, pat: P) -> &str;
}

impl Substring for str {
    #[inline]
    fn substring_before<P: Pattern>(&self, pat: P) -> &str {
        match self.find(pat) {
            Some(i) => &self[..i],
            None => "",
        }
    }

    #[inline]
    fn substring_after<P: Pattern>(&self, pat: P) -> &str {
        match pat.into_searcher(self).next_match() {
            Some((_, end)) => &self[end..],
            None => "",
        }
    }
}
