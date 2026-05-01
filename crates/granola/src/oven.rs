//! Macros and primitives for building strings.
//!
//! With [`bake_block`], [`bake_inline`], and [`bake_newline`], [`Template`] types and
//! [`AsRef<str>`] values can be freely mixed and rendered into a single [`String`].
//!
//! The dispatch between the two is resolved at compile time. [`Template`] items render via
//! [`Template::render_into`] while string values fall back to [`String::push_str`].
//!
//! This is inspired by [`askama::FastWritable`] and relies on [autoref-based specialization].
//!
//! [`bake_block`]: crate::bake_block
//! [`bake_inline`]: crate::bake_inline
//! [`bake_newline`]: crate::bake_newline
//! [autoref-based specialization]:
//! https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

use askama::Template;

/// Wrapper type for the inherent `bake_content` impl that handles [`Template`] values.
pub struct Bake<T>(pub T);

impl<T: Template> Bake<&T> {
    /// Renders the template into a [`String`].
    ///
    /// # Panics
    ///
    /// Panics if [`Template::render_into`] returns an error.
    /// Writing into a [`String`] via [`core::fmt::Write`] is infallible,
    /// so the only way this fails is if the template itself errors.
    /// See [`askama::Error`].
    pub fn bake_content(&self, buf: &mut String) {
        self.0.render_into(buf).unwrap();
    }

    pub fn size_hint(&self) -> usize {
        T::SIZE_HINT
    }
}

/// Fallback trait for the `bake_content` impl that handles [`AsRef<str>`] values.
pub trait Roast {
    fn bake_content(&self, buf: &mut String);

    fn size_hint(&self) -> usize;
}

impl<T: AsRef<str>> Roast for Bake<&T> {
    fn bake_content(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }

    fn size_hint(&self) -> usize {
        self.0.as_ref().len()
    }
}

/// Renders any number of items into a single [`String`], placing each on a new line.
///
/// Accepts [`Template`] types and string-like values (e.g. `&str`, `String`) freely mixed.
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let textarea: HtmlTextarea = HtmlTextarea::new("Exegi monumentum aere perennius").id("ode");
///
/// let content = bake_block!["Notes", textarea];
///
/// let label: HtmlLabel = HtmlLabel::new(content).for_id("ode");
///
/// assert_eq!(label.bake(),
/// r#"<label for="ode">
///   Notes
///   <textarea id="ode">Exegi monumentum aere perennius</textarea>
/// </label>"#);
/// ```
#[macro_export]
macro_rules! bake_block {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::oven::Roast as _;

        let mut buf = String::new();

        {
            let content = $crate::oven::Bake(&$first);
            buf.reserve(content.size_hint());
            content.bake_content(&mut buf);
        }

        $({
            let content = $crate::oven::Bake(&$rest);
            buf.reserve(1 + content.size_hint());
            buf.push('\n');
            content.bake_content(&mut buf);
        })*

        buf
    }};
}

/// Renders any number of items into a single [`String`], concatenated without any separator.
///
/// Accepts [`Template`] types and string-like values (e.g. `&str`, `String`) freely mixed.
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let docs: HtmlA = HtmlA::new("docs").href("https://askama.rs");
///
/// let content = bake_inline!["Read the ", docs, "."];
///
/// let span: HtmlSpan = HtmlSpan::new(content);
///
/// assert_eq!(span.bake(),
/// r#"<span>Read the <a href="https://askama.rs">docs</a>.</span>"#);
/// ```
#[macro_export]
macro_rules! bake_inline {
    ($($item:expr),+ $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::oven::Roast as _;

        let mut buf = String::new();

        $({
            let content = $crate::oven::Bake(&$item);
            buf.reserve(content.size_hint());
            content.bake_content(&mut buf);
        })*

        buf
    }};
}

/// Renders one item into a single [`String`], with a leading newline.
///
/// Accepts [`Template`] types and string-like values (e.g. `&str`, `String`).
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let content = bake_newline!("content");
///
/// let paragraph: HtmlP = HtmlP::new(content);
///
/// assert_eq!(paragraph.bake(),
/// r#"<p>
///   content
/// </p>"#);
/// ```
#[macro_export]
macro_rules! bake_newline {
    ($item:expr $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::oven::Roast as _;

        let content = $crate::oven::Bake(&$item);
        let mut buf = String::with_capacity(1 + content.size_hint());
        buf.push('\n');
        content.bake_content(&mut buf);

        buf
    }};
}

#[cfg(test)]
mod oven_tests {
    #[test]
    fn bake_block_1() {
        assert_eq!(bake_block![""], "");
    }

    #[test]
    fn bake_block_2() {
        assert_eq!(bake_block!["single\nitem"], "single\nitem");
    }

    #[test]
    fn bake_block_3() {
        assert_eq!(bake_block!["hello", "world"], "hello\nworld");
    }

    #[test]
    fn bake_block_4() {
        assert_eq!(
            bake_block!["halloween", "hello\nworld"],
            "halloween\nhello\nworld"
        );
    }

    #[test]
    fn bake_inline_1() {
        assert_eq!(bake_inline![""], "");
    }

    #[test]
    fn bake_inline_2() {
        assert_eq!(bake_inline!["single\nitem"], "single\nitem");
    }

    #[test]
    fn bake_inline_3() {
        assert_eq!(bake_inline!["hallo", "ween"], "halloween");
    }

    #[test]
    fn bake_inline_4() {
        assert_eq!(
            bake_inline!["halloween ", "hello\nworld"],
            "halloween hello\nworld"
        );
    }

    #[test]
    fn bake_newline_1() {
        assert_eq!(bake_newline!(""), "\n");
    }

    #[test]
    fn bake_newline_2() {
        assert_eq!(bake_newline!("halloween"), "\nhalloween");
    }

    #[test]
    fn bake_newline_3() {
        assert_eq!(bake_newline!("hello\nworld"), "\nhello\nworld");
    }
}

/// Recursively right-folds a list of types into a nested tuple.
///
/// A single type passes through unchanged.
/// Two or more types become `(A, (B, (C, …)))`.
///
/// It facilitates recipe composition.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// type SubmitPost = (Submit, Post);
///
/// let input: HtmlInput<SubmitPost> = HtmlInput::from_value("Send");
///
/// assert_eq!(input.bake(),
/// r#"<input type="submit" formmethod="post" value="Send" />"#);
/// ```
#[macro_export]
macro_rules! rec {
    ($a:ty) => { $a };
    ($a:ty, $($rest:ty),+) => { ($a, $crate::rec!($($rest),+)) };
}

// The SIZE_HINT-based preallocation depends on two things:
//
// 1. Askama's `Template::SIZE_HINT` is a tight estimate
//     (scaffold plus a small per-expression headroom).
//
// 2. `bake()`, `bake_block!`, `bake_inline!`, and `bake_newline!`
//     reserve capacity up-front: HINT bytes, or 1 + HINT.
//
// Two fixtures per element:
// - small fixture (fits in HINT): String capacity == HINT
// - larger fixture (exceeds HINT): String capacity <= 2 * HINT
#[cfg(test)]
mod preallocation_tests {
    use crate::prelude::*;
    use askama::Template;

    const IMG_HINT: usize = <HtmlImg as Template>::SIZE_HINT;
    const P_HINT: usize = <HtmlP as Template>::SIZE_HINT;

    fn img_empty() -> HtmlImg {
        HtmlImg::empty()
    }

    fn img_with_src() -> HtmlImg {
        HtmlImg::from_src("https://example.com/")
    }

    fn p_empty() -> HtmlP {
        HtmlP::empty()
    }

    fn p_with_span() -> HtmlP {
        let span: HtmlSpan = HtmlSpan::new("hello, world!");
        HtmlP::new(span)
    }

    #[test]
    fn size_hint_is_tight() {
        let empty: HtmlP = HtmlP::empty();
        let headroom = P_HINT - empty.bake().len();

        let at_boundary: HtmlP = HtmlP::new("x".repeat(headroom));
        assert_eq!(at_boundary.bake().len(), P_HINT);

        let past_boundary: HtmlP = HtmlP::new("x".repeat(headroom + 1));
        assert_eq!(past_boundary.bake().len(), P_HINT + 1);
    }

    #[test]
    fn bake_capacity() {
        assert_eq!(img_empty().bake().capacity(), IMG_HINT);
        assert_eq!(p_empty().bake().capacity(), P_HINT);
        assert!(img_with_src().bake().capacity() <= 2 * IMG_HINT);
        assert!(p_with_span().bake().capacity() <= 2 * P_HINT);
    }

    #[test]
    fn bake_block_capacity() {
        assert_eq!(bake_block![img_empty()].capacity(), IMG_HINT);
        assert_eq!(bake_block![p_empty()].capacity(), P_HINT);
        assert!(bake_block![img_with_src()].capacity() <= 2 * IMG_HINT);
        assert!(bake_block![p_with_span()].capacity() <= 2 * P_HINT);
    }

    #[test]
    fn bake_inline_capacity() {
        assert_eq!(bake_inline![img_empty()].capacity(), IMG_HINT);
        assert_eq!(bake_inline![p_empty()].capacity(), P_HINT);
        assert!(bake_inline![img_with_src()].capacity() <= 2 * IMG_HINT);
        assert!(bake_inline![p_with_span()].capacity() <= 2 * P_HINT);
    }

    #[test]
    fn bake_newline_capacity() {
        assert_eq!(bake_newline!(img_empty()).capacity(), 1 + IMG_HINT);
        assert_eq!(bake_newline!(p_empty()).capacity(), 1 + P_HINT);
        assert!(bake_newline!(img_with_src()).capacity() <= 2 * (1 + IMG_HINT));
        assert!(bake_newline!(p_with_span()).capacity() <= 2 * (1 + P_HINT));
    }
}
