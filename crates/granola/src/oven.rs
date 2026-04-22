//! Macros and primitives for building strings.
//!
//! With [`bake_block`] and [`bake_inline`], [`Template`] types and [`AsRef<str>`] values can be
//! freely mixed and rendered into a single [`String`].
//!
//! The dispatch between the two is resolved at compile time. [`Template`] items render via
//! [`Template::render_into`] while string values fall back to [`String::push_str`].
//!
//! This is inspired by [`askama::FastWritable`] and relies on [autoref-based specialization].
//!
//! [`bake_block`]: crate::bake_block
//! [`bake_inline`]: crate::bake_inline
//! [autoref-based specialization]:
//! https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

use askama::Template;

/// Wrapper type for the inherent `bake_content` impl that handles [`Template`] values.
pub struct Bake<T>(pub T);

impl<T: Template> Bake<&T> {
    pub fn bake_content(&self, buf: &mut String) {
        let _ = self.0.render_into(buf);
    }
}

/// Fallback trait for the `bake_content` impl that handles [`AsRef<str>`] values.
pub trait Roast {
    fn bake_content(&self, buf: &mut String);
}

impl<T: AsRef<str>> Roast for Bake<&T> {
    fn bake_content(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
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

        $crate::oven::Bake(&$first).bake_content(&mut buf);

        $(
            buf.push('\n');
            $crate::oven::Bake(&$rest).bake_content(&mut buf);
        )*

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

        $($crate::oven::Bake(&$item).bake_content(&mut buf);)*

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

        let mut buf = String::from("\n");

        $crate::oven::Bake(&$item).bake_content(&mut buf);

        buf
    }};
}

#[cfg(test)]
mod oven_tests {
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
