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

use std::{borrow::Cow, fmt};

use askama::{FastWritable, NO_VALUES, Template, Values};

/// Wraps a [`FastWritable`] value so it can be used as a recipe's `type
/// Content`.
///
/// An element's default content type is [`Cow<'static, str>`], and overriding
/// `type Content` requires the override to bake back into it. A foreign type
/// like `u32` can't satisfy that directly, but `BakeFrom` can.
///
/// ```rust
/// use granola::prelude::*;
///
/// #[derive(Default, Debug, Clone)]
/// struct Answer;
///
/// impl OutputRecipe for Answer {
///     type Content = BakeFrom<u32>;
/// }
///
/// let output = HtmlOutput::from(Answer).content(42);
///
/// assert_eq!(output.bake(), "<output>42</output>");
/// ```
pub struct BakeFrom<T>(pub T);

impl<T: FastWritable> FastWritable for BakeFrom<T> {
    fn write_into(&self, dest: &mut dyn fmt::Write, values: &dyn Values) -> askama::Result<()> {
        self.0.write_into(dest, values)
    }
}

impl<T: Default> Default for BakeFrom<T> {
    fn default() -> Self {
        BakeFrom(T::default())
    }
}

impl<T: Clone> Clone for BakeFrom<T> {
    fn clone(&self) -> Self {
        BakeFrom(self.0.clone())
    }
}

impl<T: fmt::Debug> fmt::Debug for BakeFrom<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("BakeFrom").field(&self.0).finish()
    }
}

/// Lets the element's `new(value)` build a `BakeFrom<T>` content directly,
/// without the caller writing `BakeFrom(value)`.
impl<T: FastWritable> From<T> for BakeFrom<T> {
    fn from(value: T) -> Self {
        BakeFrom(value)
    }
}

/// Bakes any [`FastWritable`] content back into the default [`Cow<'static,
/// str>`] content type.
///
/// # Panics
///
/// Panics if [`FastWritable::write_into`] returns an error. See
/// [`askama::Error`].
impl<T: FastWritable> From<BakeFrom<T>> for Cow<'static, str> {
    fn from(wrapped: BakeFrom<T>) -> Self {
        let mut buf = String::new();
        wrapped.0.write_into(&mut buf, NO_VALUES).unwrap();
        Cow::Owned(buf)
    }
}

/// Wrapper type carrying the autoref-based content dispatch for the `bake_*!`
/// macros.
///
/// See [`Roast`] for the tiered dispatch it drives.
pub struct Bake<T>(pub T);

/// Tiered content dispatch for the `bake_*!` macros, resolved at compile time
/// by [autoref-based specialization], in priority order:
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
/// and `FastWritable`) resolves to the highest applicable tier, so strings keep
/// their exact size hint.
///
/// # Panics
///
/// Panics if [`Template::render_into`] or [`FastWritable::write_into`] returns
/// an error. See [`askama::Error`].
///
/// [autoref-based specialization]:
/// https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
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
/// `bake_recipe` moves all fields into `Foo<()>`, applying [`BakeInto`] for any
/// content field.
///
/// This is the canonical way to land a `Foo<R>` into a collection that stores
/// `Foo<()>`. It exists as its own trait because `From<Foo<R>> for Foo<()>`
/// cannot be written: at `R = ()` it overlaps the std reflexive `impl<T>
/// From<T> for T`.
pub trait BakeRecipe {
    type Baked;

    fn bake_recipe(self) -> Self::Baked;
}

/// Marks that a recipe's custom content type can be baked back into the
/// element's default content type.
///
/// You never implement this directly: it has a blanket impl for every `T:
/// Into<D>`. Its only job is to give a guided compiler error when a recipe
/// overrides `type Content` but is missing the matching `From` impl.
#[diagnostic::on_unimplemented(
    message = "recipe content `{Self}` can't bake back into `{D}`",
    label = "try using `BakeFrom<{Self}>`",
    note = "for non-foreign types, consider providing a conversion by implementing `From<{Self}>` for `{D}`"
)]
pub trait BakeInto<D> {
    fn bake_into(self) -> D;
}

#[diagnostic::do_not_recommend]
impl<T, D> BakeInto<D> for T
where
    T: Into<D>,
{
    fn bake_into(self) -> D {
        self.into()
    }
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
    use askama::{FastWritable, Values};
    use std::fmt;

    use crate::prelude::*;

    #[derive(Default, Debug, Clone)]
    struct Number;

    impl PRecipe for Number {
        type Content = BakeFrom<u32>;
    }

    #[derive(Default, Debug, Clone)]
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
        type Content = BakeFrom<Celsius>;
    }

    #[test]
    fn new_accepts_primitive_directly() {
        let p = HtmlP::from(Number).content(42);
        assert_eq!(p.bake(), "<p>42</p>");
    }

    #[test]
    fn new_accepts_explicit_wrapper() {
        let p = HtmlP::from(Number).content(BakeFrom(42u32));
        assert_eq!(p.bake(), "<p>42</p>");
    }

    #[test]
    fn bakes_back_into_default_content() {
        let baked = HtmlP::from(Number).content(42);
        assert_eq!(baked.bake(), "<p>42</p>");
    }

    #[test]
    fn new_accepts_custom_directly() {
        let p = HtmlP::from(Temperature).content(Celsius(26));
        assert_eq!(p.bake(), "<p>26°C</p>");
    }

    #[test]
    fn new_accepts_custom_explicit_wrapper() {
        let p = HtmlP::from(Temperature).content(BakeFrom(Celsius(26)));
        assert_eq!(p.bake(), "<p>26°C</p>");
    }

    #[test]
    fn bakes_back_custom_into_default_content() {
        let baked = HtmlP::from(Temperature).content(Celsius(26));
        assert_eq!(baked.bake(), "<p>26°C</p>");
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
