//! String-building primitives and recipe machinery.
//!
//! [`bake_block!`](crate::bake_block), [`bake_inline!`](crate::bake_inline),
//! and [`bake_newline!`](crate::bake_newline) render [`Template`] types,
//! [`AsRef<str>`] values, and any other [`FastWritable`] type (e.g. primitives)
//! freely mixed into a single [`String`]. The dispatch is resolved at compile
//! time via [autoref-based specialization]; see [`Roast`] for the priority
//! order.
//!
//! [`BakeRecipe`] converts a built `Foo<R>` into `Foo<()>` for storage in typed
//! collections. [`cookbook!`](crate::cookbook_type!) composes multiple recipes
//! into `(A, (B, C))` in type position; [`cookbook!`](crate::cookbook!) is its
//! value-level counterpart.
//!
//! [autoref-based specialization]:
//! https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

use askama::{FastWritable, NO_VALUES, Template};

/// Implements `bake_content` for a recipe's content map-back.
///
/// `recipe_boilerplate!()` keeps `type Content` at its default, so mapping back
/// into the default content type is a no-op.
///
/// ```rust
/// use granola::prelude::*;
///
/// #[derive(Default, Debug, Clone)]
/// struct Yell;
///
/// impl SpanRecipe for Yell {
///     recipe_boilerplate!();
///
///     fn content_recipe(content: &mut Self::Content) {
///         *content = content.to_uppercase().into();
///     }
/// }
///
/// let span = HtmlSpan::from(Yell).content("oh, hi!");
///
/// assert_eq!(span.bake(), "<span>OH, HI!</span>");
/// ```
///
/// `recipe_boilerplate!(@from T; @into U)` sets `type Content = T` and maps
/// back via `T: Into<U>`, where `U` is the default content type.
///
/// ```rust
/// use askama::Template;
/// use granola::prelude::*;
/// use std::borrow::Cow;
///
/// #[derive(Default, Debug, Clone, Template)]
/// #[template(ext = "html", source = "hi!")]
/// struct Hi;
///
/// impl From<Hi> for Cow<'static, str> {
///     fn from(hi: Hi) -> Self {
///         Cow::Owned(hi.render().unwrap())
///     }
/// }
///
/// impl SpanRecipe for Hi {
///     recipe_boilerplate!(@from Hi; @into Cow<'static, str>);
/// }
///
/// let span = HtmlSpan::from(Hi);
///
/// assert_eq!(span.bake(), "<span>hi!</span>");
/// ```
#[macro_export]
macro_rules! recipe_boilerplate {
    () => {
        fn bake_content(content: Self::Content) -> Self::Content {
            content
        }
    };
    (@from $new_content_type:ty ; @into $default_content_type:ty) => {
        type Content = $new_content_type;

        fn bake_content(content: $new_content_type) -> $default_content_type {
            content.into()
        }
    };
}

/// Wrapper type carrying the autoref-based content dispatch.
///
/// See [`Roast`].
pub struct Bake<T>(pub T);

/// Tiered content dispatch.
///
/// The priority order:
///
/// 1. `T: Template` — rendered via [`Template::render_into`] with an exact
///    [`Template::SIZE_HINT`].
/// 2. `T: AsRef<str>` — appended via [`String::push_str`] with an exact `len`
///    size hint.
/// 3. any other `T: FastWritable` (e.g. primitives) — written via
///    [`FastWritable::write_into`]; no size hint is available, so it reports
///    `0`.
///
/// A type matching several bounds (e.g. `String`, which is both `AsRef<str>`
/// and `FastWritable`) resolves to the highest applicable tier. `String` takes
/// the `AsRef<str>` tier, with its `len` size hint.
///
/// # Panics
///
/// Panics if [`Template::render_into`] or [`FastWritable::write_into`] returns
/// an error. See [`askama::Error`].
pub trait Roast {
    fn bake_content(&self, buf: &mut String);

    fn size_hint(&self) -> usize;
}

impl<T: Template> Roast for &&Bake<&T> {
    fn bake_content(&self, buf: &mut String) {
        self.0.render_into(buf).unwrap();
    }

    fn size_hint(&self) -> usize {
        T::SIZE_HINT
    }
}

impl<T: AsRef<str>> Roast for &Bake<&T> {
    fn bake_content(&self, buf: &mut String) {
        buf.push_str(self.0.as_ref());
    }

    fn size_hint(&self) -> usize {
        self.0.as_ref().len()
    }
}

impl<T: FastWritable> Roast for Bake<&T> {
    fn bake_content(&self, buf: &mut String) {
        self.0.write_into(buf, NO_VALUES).unwrap();
    }

    fn size_hint(&self) -> usize {
        0
    }
}

/// Converts `Foo<R>` into `Foo`.
///
/// `PhantomData<R>` selects which recipe runs during construction.
/// `bake_recipe` moves all fields into `Foo<()>`, calling the recipe's
/// `bake_content` to map any content field back into the default content type.
///
/// This is the canonical way to land a `Foo<R>` into a collection that stores
/// `Foo<()>`. It exists as its own trait because `From<Foo<R>> for Foo<()>`
/// cannot be written: at `R = ()` it overlaps the std reflexive `impl<T>
/// From<T> for T`.
pub trait BakeRecipe {
    type Baked;

    fn bake_recipe(self) -> Self::Baked;
}

/// Renders any number of items into a single [`String`], placing each on a new
/// line.
///
/// Accepts [`Template`] types and string-like values (e.g. `&str`, `String`)
/// freely mixed.
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
/// let content = bake_block!["Notes", textarea];
///
/// let label = HtmlLabel::new().content(content).for_id("ode");
///
/// assert_eq!(
///     label.bake(),
///     r#"<label for="ode">
///   Notes
///   <textarea id="ode">Exegi monumentum aere perennius</textarea>
/// </label>"#
/// );
/// ```
#[macro_export]
macro_rules! bake_block {
    ($first:expr $(, $rest:expr)* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::oven::Roast as _;

        let mut buf = String::new();

        {
            let content = $crate::oven::Bake(&$first);
            buf.reserve((&&&content).size_hint());
            (&&&content).bake_content(&mut buf);
        }

        $({
            let content = $crate::oven::Bake(&$rest);
            buf.reserve(1 + (&&&content).size_hint());
            buf.push('\n');
            (&&&content).bake_content(&mut buf);
        })*

        buf
    }};
}

/// Renders any number of items into a single [`String`], concatenated without
/// any separator.
///
/// Accepts [`Template`] types and string-like values (e.g. `&str`, `String`)
/// freely mixed.
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let docs = HtmlA::new().content("docs").href("https://askama.rs");
///
/// let content = bake_inline!["Read the ", docs, "."];
///
/// let span = HtmlSpan::new().content(content);
///
/// assert_eq!(
///     span.bake(),
///     r#"<span>Read the <a href="https://askama.rs">docs</a>.</span>"#
/// );
/// ```
#[macro_export]
macro_rules! bake_inline {
    ($($item:expr),+ $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::oven::Roast as _;

        let mut buf = String::new();

        $({
            let content = $crate::oven::Bake(&$item);
            buf.reserve((&&&content).size_hint());
            (&&&content).bake_content(&mut buf);
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
/// let paragraph = HtmlP::new().content(content);
///
/// assert_eq!(
///     paragraph.bake(),
///     r#"<p>
///   content
/// </p>"#
/// );
/// ```
#[macro_export]
macro_rules! bake_newline {
    ($item:expr $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::oven::Roast as _;

        let content = $crate::oven::Bake(&$item);
        let mut buf = String::with_capacity(1 + (&&&content).size_hint());
        buf.push('\n');
        (&&&content).bake_content(&mut buf);

        buf
    }};
}

/// Composes recipe **types** into a single nested-tuple type.
///
/// A single type passes through unchanged; two or more fold right into
/// `(A, (B, (C, …)))`. Use it wherever a composed recipe is needed in type
/// position, such as a type alias or a generic argument.
///
/// See [`cookbook!`](crate::cookbook!) for the value-level counterpart.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// type SubmitPost = cookbook_type!(TypeSubmit, FormmethodPost);
///
/// let input: HtmlInput<SubmitPost> = HtmlInput::from_cookbook().value("Send");
///
/// assert_eq!(
///     input.bake(),
///     r#"<input type="submit" value="Send" formmethod="post" />"#
/// );
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// type InlineFlex = cookbook_type!(Inline, Flex);
///
/// let display: CssDisplay<InlineFlex> = CssDisplay::from_cookbook();
///
/// assert_eq!(display.bake(), "display: inline flex;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// type InlineFlexImportant = cookbook_type!(Inline, Flex, Important);
///
/// let display: CssDisplay<InlineFlexImportant> = CssDisplay::from_cookbook();
///
/// assert_eq!(display.bake(), "display: inline flex !important;");
/// ```
#[macro_export]
macro_rules! cookbook_type {
    ($a:ty) => { $a };
    ($a:ty, $($rest:ty),+) => { ($a, $crate::cookbook_type!($($rest),+)) };
}

/// Composes recipe **values** into a single nested-tuple value.
///
/// A single value passes through unchanged; two or more fold right into
/// `(a, (b, (c, …)))`. It is the value-level counterpart of
/// [`cookbook_type!`](crate::cookbook_type), for passing a composed recipe to
/// a value-form constructor such as `Type::from(...)`.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let submit_post = cookbook!(TypeSubmit, FormmethodPost);
///
/// let input = HtmlInput::from(submit_post).value("Send");
///
/// assert_eq!(
///     input.bake(),
///     r#"<input type="submit" value="Send" formmethod="post" />"#
/// );
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let inline_flex = cookbook!(Inline, Flex);
///
/// let display = CssDisplay::from(inline_flex);
///
/// assert_eq!(display.bake(), "display: inline flex;");
/// ```
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let inline_flex_important = cookbook!(Inline, Flex, Important);
///
/// let display = CssDisplay::from(inline_flex_important);
///
/// assert_eq!(display.bake(), "display: inline flex !important;");
/// ```
#[macro_export]
macro_rules! cookbook {
    ($a:expr) => { $a };
    ($a:expr, $($rest:expr),+) => { ($a, $crate::cookbook!($($rest),+)) };
}

#[cfg(test)]
mod from_content_type_tests {
    use askama::{FastWritable, NO_VALUES, Values};
    use std::{borrow::Cow, fmt};

    use crate::prelude::*;

    #[derive(Default, Debug, Clone)]
    struct Number;

    impl PRecipe for Number {
        type Content = u8;

        fn bake_content(content: Self::Content) -> Cow<'static, str> {
            content.to_string().into()
        }
    }

    #[derive(Default, Debug, Clone, PartialEq)]
    struct Celsius(i32);

    impl FastWritable for Celsius {
        fn write_into(&self, dest: &mut dyn fmt::Write, _: &dyn Values) -> askama::Result<()> {
            write!(dest, "{}°C", self.0)?;
            Ok(())
        }
    }

    #[derive(Default, Debug, Clone)]
    struct Temperature;

    impl PRecipe for Temperature {
        type Content = Celsius;

        fn bake_content(content: Self::Content) -> Cow<'static, str> {
            let mut buf = String::new();
            content.write_into(&mut buf, NO_VALUES).unwrap();
            Cow::Owned(buf)
        }
    }

    #[test]
    fn primitive() {
        let p = HtmlP::from(Number).content(42);
        assert_eq!(p.bake(), "<p>42</p>");

        let content: u8 = p.content;
        assert_eq!(content, 42);
    }

    #[test]
    fn primitive_baked() {
        let baked = HtmlP::from(Number).content(42).bake_recipe();
        assert_eq!(baked.bake(), "<p>42</p>");

        let content: Cow<_> = baked.content;
        assert_eq!(content, "42");
    }

    #[test]
    fn custom() {
        let p = HtmlP::from(Temperature).content(Celsius(26));
        assert_eq!(p.bake(), "<p>26°C</p>");

        let content: Celsius = p.content;
        assert_eq!(content, Celsius(26));
    }

    #[test]
    fn custom_baked() {
        let baked = HtmlP::from(Temperature).content(Celsius(26)).bake_recipe();
        assert_eq!(baked.bake(), "<p>26°C</p>");

        let content: Cow<_> = baked.content;
        assert_eq!(content, "26°C");
    }
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
    fn bake_block_5() {
        use crate::prelude::HtmlSpan;

        let span = HtmlSpan::new().content("bar");

        assert_eq!(bake_block!["foo", span, 42], "foo\n<span>bar</span>\n42");
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
    fn bake_inline_5() {
        assert_eq!(bake_inline![1, 2, 3], "123");
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

    #[test]
    fn bake_newline_4() {
        assert_eq!(bake_newline!(42), "\n42");
    }
}

// The SIZE_HINT-based preallocation depends on two things:
//
// 1. Askama's `Template::SIZE_HINT` is a tight estimate (scaffold plus a small
//    per-expression headroom).
//
// 2. `bake()`, `bake_block!`, `bake_inline!`, and `bake_newline!` reserve
//    capacity up-front: HINT bytes, or 1 + HINT.
//
// Two fixtures per element:
// - small fixture (fits in HINT): String capacity == HINT
// - larger fixture (exceeds HINT): String capacity <= 2 * HINT
#[cfg(test)]
mod preallocation_tests {
    use askama::Template;

    use crate::prelude::*;

    const IMG_HINT: usize = <HtmlImg as Template>::SIZE_HINT;
    const P_HINT: usize = <HtmlP as Template>::SIZE_HINT;

    fn img_empty() -> HtmlImg {
        HtmlImg::new()
    }

    fn img_with_src() -> HtmlImg {
        HtmlImg::from_src("https://example.com/")
    }

    fn p_empty() -> HtmlP {
        HtmlP::new()
    }

    fn p_with_span() -> HtmlP {
        let span = HtmlSpan::new().content("hello, world!");
        HtmlP::new().content(span)
    }

    #[test]
    fn size_hint_is_tight() {
        let empty = HtmlP::new();
        let headroom = P_HINT - empty.bake().len();

        let at_boundary = HtmlP::new().content("x".repeat(headroom));
        assert_eq!(at_boundary.bake().len(), P_HINT);

        let past_boundary = HtmlP::new().content("x".repeat(headroom + 1));
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
