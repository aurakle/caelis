use std::{
    fmt::Display,
    str::{CharIndices, Chars},
};

use nom::{AsBytes, Compare, Input, Offset, ParseTo};
use nom_recursive::{HasRecursiveInfo, RecursiveInfo};

#[derive(Debug, Clone)]
pub(crate) struct Span<'a>(pub(crate) &'a str, pub(crate) RecursiveInfo);

impl<'a> Input for Span<'a> {
    type Item = char;
    type Iter = Chars<'a>;
    type IterIndices = CharIndices<'a>;

    fn input_len(&self) -> usize {
        self.0.input_len()
    }

    fn take(&self, index: usize) -> Self {
        Self(self.0.take(index), self.1)
    }

    fn take_from(&self, index: usize) -> Self {
        Self(self.0.take_from(index), self.1)
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let (left, right) = self.0.take_split(index);
        (Self(left, self.1), Self(right, self.1))
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.0.position(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.0.iter_elements()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.0.iter_indices()
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        self.0.slice_index(count)
    }
}

impl Display for Span<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl AsBytes for Span<'_> {
    fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl HasRecursiveInfo for Span<'_> {
    fn get_recursive_info(&self) -> RecursiveInfo {
        self.1
    }

    fn set_recursive_info(self, info: RecursiveInfo) -> Self {
        Self(self.0, info)
    }
}

impl Compare<&str> for Span<'_> {
    fn compare(&self, t: &str) -> nom::CompareResult {
        self.0.compare(t)
    }

    fn compare_no_case(&self, t: &str) -> nom::CompareResult {
        self.0.compare_no_case(t)
    }
}

impl Offset for Span<'_> {
    fn offset(&self, second: &Self) -> usize {
        self.0.offset(second.0)
    }
}

impl ParseTo<i64> for Span<'_> {
    fn parse_to(&self) -> Option<i64> {
        self.0.parse_to()
    }
}

impl ParseTo<f64> for Span<'_> {
    fn parse_to(&self) -> Option<f64> {
        self.0.parse_to()
    }
}

impl<'a> Compare<&'a [u8]> for Span<'_> {
    fn compare(&self, t: &'a [u8]) -> nom::CompareResult {
        self.0.compare(t)
    }

    fn compare_no_case(&self, t: &'a [u8]) -> nom::CompareResult {
        self.0.compare_no_case(t)
    }
}
