use askama::{FastWritable, NO_VALUES, Template, Values};
use std::{borrow::Cow, fmt};

/// [`Cow<'static, str>`] with extra steps.
#[derive(Hash, Debug, Clone, Default, PartialEq, Eq)]
pub struct Bake(Cow<'static, str>);

impl Bake {
    /// Creates [`Bake`] from [`Template`].
    ///
    /// # Panics
    ///
    /// Panics if [`FastWritable::write_into`] returns an error. See
    /// [`askama::Error`].
    pub fn new<T: Template + FastWritable>(template: &T) -> Self {
        let mut buf = String::with_capacity(T::SIZE_HINT);
        FastWritable::write_into(template, &mut buf, NO_VALUES).unwrap();
        Self(Cow::Owned(buf))
    }

    /// Appends a [`FastWritable`] item in place.
    ///
    /// # Panics
    ///
    /// Panics if [`FastWritable::write_into`] returns an error. See
    /// [`askama::Error`].
    pub fn fold_in(&mut self, item: impl FastWritable) {
        item.write_into(self.0.to_mut(), NO_VALUES).unwrap();
    }

    /// Appends a [`FastWritable`] item in place, separated from the existing
    /// content by `sep`. If either half is empty, no separator is written.
    ///
    /// # Panics
    ///
    /// Panics if [`FastWritable::write_into`] returns an error. See
    /// [`askama::Error`].
    pub fn fold_in_with(&mut self, sep: &str, item: impl FastWritable) {
        let buf = self.0.to_mut();
        let start = buf.len();
        if start > 0 {
            buf.push_str(sep);
        }
        item.write_into(&mut *buf, NO_VALUES).unwrap();
        if start > 0 && buf.len() == start + sep.len() {
            buf.truncate(start);
        }
    }

    /// Appends a [`FastWritable`] item in place, separated from the existing
    /// content by a single space. If either half is empty, no separator is
    /// written.
    ///
    /// # Panics
    ///
    /// Panics if [`FastWritable::write_into`] returns an error. See
    /// [`askama::Error`].
    pub fn fold_in_ws(&mut self, item: impl FastWritable) {
        self.fold_in_with(" ", item);
    }

    /// Returns `true` if the content is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Creates an empty [`Bake`] with at least `capacity` bytes of capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Cow::Owned(String::with_capacity(capacity)))
    }
}

impl FastWritable for Bake {
    fn write_into(&self, dest: &mut dyn fmt::Write, values: &dyn Values) -> askama::Result<()> {
        self.0.write_into(dest, values)
    }
}

impl fmt::Display for Bake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.write_into(f, NO_VALUES).map_err(Into::into)
    }
}

impl AsRef<str> for Bake {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for Bake {
    fn from(s: &'static str) -> Self {
        Cow::<'static, str>::Borrowed(s).into()
    }
}

impl From<String> for Bake {
    fn from(s: String) -> Self {
        Cow::<'static, str>::Owned(s).into()
    }
}

impl From<Cow<'static, str>> for Bake {
    fn from(c: Cow<'static, str>) -> Self {
        Self(c)
    }
}

macro_rules! impl_from_primitive {
    ($($ty:ty),+ $(,)?) => {$(
        impl From<$ty> for Bake {
            fn from(value: $ty) -> Self {
                value.to_string().into()
            }
        }
    )+};
}

impl_from_primitive!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char,
);

impl PartialEq<str> for Bake {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for Bake {
    fn eq(&self, other: &&str) -> bool {
        PartialEq::<str>::eq(self, other)
    }
}

impl From<Bake> for Cow<'static, str> {
    fn from(c: Bake) -> Self {
        c.0
    }
}

impl From<Bake> for String {
    fn from(c: Bake) -> Self {
        c.0.into_owned()
    }
}

impl<T: Into<Bake>, const N: usize> From<[T; N]> for Bake {
    fn from(items: [T; N]) -> Self {
        let mut bake = Bake::default();
        for item in items {
            bake.fold_in(item.into());
        }
        bake
    }
}

impl<T: Into<Bake>> From<Vec<T>> for Bake {
    fn from(items: Vec<T>) -> Self {
        let mut bake = Bake::default();
        for item in items {
            bake.fold_in(item.into());
        }
        bake
    }
}

// Provide an upfront size estimate for `bake!` and `bake_ws!` macros.
//
// The macros call `(&&BakeSize(item)).bake_size()`.
// Method resolution picks the first applicable impl by autoref:
//
// - `TemplateBakeSize` (on `&BakeSize`, reads `T::SIZE_HINT`).
// - `StrBakeSize` (on `&&BakeSize`, reads the string length).
// - `AnyBakeSize` (on `BakeSize`, returns 0).
//
// See:
// <https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html>
#[doc(hidden)]
pub struct BakeSize<'a, T: ?Sized>(pub &'a T);

#[doc(hidden)]
pub trait TemplateBakeSize {
    fn bake_size(&self) -> usize;
}

impl<T: Template + ?Sized> TemplateBakeSize for &BakeSize<'_, T> {
    fn bake_size(&self) -> usize {
        T::SIZE_HINT
    }
}

#[doc(hidden)]
pub trait StrBakeSize {
    fn bake_size(&self) -> usize;
}

impl<T: AsRef<str> + ?Sized> StrBakeSize for &&BakeSize<'_, T> {
    fn bake_size(&self) -> usize {
        self.0.as_ref().len()
    }
}

#[doc(hidden)]
pub trait AnyBakeSize {
    fn bake_size(&self) -> usize;
}

impl<T: ?Sized> AnyBakeSize for BakeSize<'_, T> {
    fn bake_size(&self) -> usize {
        0
    }
}

/// Creates [`Bake`] by concatenating [`Template`],
/// string-like values, and primitives, freely mixed.
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let docs = HtmlA::new().content("docs").href("https://askama.rs");
///
/// let content = bake!["Read the ", docs, "."];
///
/// let span = HtmlSpan::new().content(content);
///
/// assert_eq!(
///     span.bake(),
///     r#"<span>Read the <a href="https://askama.rs">docs</a>.</span>"#
/// );
/// ```
#[macro_export]
macro_rules! bake {
    (@bind [$($bound:ident)*] $head:expr $(, $tail:expr)*) => {{
        let item = &$head;
        $crate::bake!(@bind [$($bound)* item] $($tail),*)
    }};
    (@bind [$($bound:ident)*]) => {{
        #[allow(unused_imports)]
        use $crate::oven::{AnyBakeSize as _, StrBakeSize as _, TemplateBakeSize as _};
        let capacity = 0usize $(+ (&&$crate::oven::BakeSize($bound)).bake_size())*;
        let mut content = $crate::oven::Bake::with_capacity(capacity);
        $(
            content.fold_in($bound);
        )*
        content
    }};
    ($($item:expr),+ $(,)?) => {
        $crate::bake!(@bind [] $($item),+)
    };
}

/// Creates [`Bake`] by concatenating [`Template`],
/// string-like values, and primitives, freely mixed, separated by a single
/// space.
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let textarea = HtmlTextarea::new()
///     .content("Exegi monumentum aere perennius")
///     .id("ode");
///
/// let content = bake_ws!["Notes", textarea];
///
/// let label = HtmlLabel::new().content(content).for_id("ode");
///
/// assert_eq!(
///     label.bake(),
///     r#"<label for="ode">Notes <textarea id="ode">Exegi monumentum aere perennius</textarea></label>"#
/// );
/// ```
#[macro_export]
macro_rules! bake_ws {
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $crate::bake!($first $(, " ", $rest)*)
    };
}

/// Creates [`Bake`] by concatenating [`Template`],
/// string-like values, and primitives, freely mixed, separated by a comma and a
/// single space.
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let headings = bake_comma!["h1", "h2", "h3", "h4"];
///
/// assert_eq!(headings, "h1, h2, h3, h4");
/// ```
#[macro_export]
macro_rules! bake_comma {
    ($first:expr $(, $rest:expr)* $(,)?) => {
        $crate::bake!($first $(, ", ", $rest)*)
    };
}

#[cfg(test)]
mod oven_tests {
    #[test]
    fn bake_1() {
        assert_eq!(bake![""], "");
    }

    #[test]
    fn bake_2() {
        assert_eq!(bake!["single\nitem"], "single\nitem");
    }

    #[test]
    fn bake_3() {
        assert_eq!(bake!["hallo", "ween"], "halloween");
    }

    #[test]
    fn bake_4() {
        assert_eq!(
            bake!["halloween ", "hello\nworld"],
            "halloween hello\nworld"
        );
    }

    #[test]
    fn bake_5() {
        assert_eq!(bake![1, 2, 3], "123");
    }

    #[test]
    fn bake_ws_1() {
        assert_eq!(bake_ws![""], "");
    }

    #[test]
    fn bake_ws_2() {
        assert_eq!(bake_ws!["single\nitem"], "single\nitem");
    }

    #[test]
    fn bake_ws_3() {
        assert_eq!(bake_ws!["hello", "world"], "hello world");
    }

    #[test]
    fn bake_ws_4() {
        assert_eq!(
            bake_ws!["halloween", "hello world"],
            "halloween hello world"
        );
    }

    #[test]
    fn bake_ws_5() {
        use crate::prelude::HtmlSpan;

        let span = HtmlSpan::new().content("bar");

        assert_eq!(bake_ws!["foo", span, 42], "foo <span>bar</span> 42");
    }

    #[test]
    fn bake_comma_1() {
        assert_eq!(bake_comma![""], "");
    }

    #[test]
    fn bake_comma_2() {
        assert_eq!(bake_comma!["single\nitem"], "single\nitem");
    }

    #[test]
    fn bake_comma_3() {
        assert_eq!(bake_comma!["hello", "world"], "hello, world");
    }

    #[test]
    fn bake_comma_4() {
        assert_eq!(
            bake_comma!["halloween", "hello world"],
            "halloween, hello world"
        );
    }

    #[test]
    fn bake_comma_5() {
        use crate::prelude::HtmlSpan;

        let span = HtmlSpan::new().content("bar");

        assert_eq!(bake_comma!["foo", span, 42], "foo, <span>bar</span>, 42");
    }
}
