//! String-building primitives and recipe machinery.
//!
//! [`bake!`](crate::bake) and [`bake_block!`](crate::bake_block), render
//! [`Template`] types, [`AsRef<str>`] values, and any other [`FastWritable`]
//! type (e.g. primitives) freely mixed into a single [`String`]. The dispatch
//! is resolved at compile time via [autoref-based specialization]; see
//! [`Roast`] for the priority order.
//!
//! [`BakeRecipe`] converts a built `Foo<R>` into `Foo<()>` for storage in typed
//! collections.
//!
//! [autoref-based specialization]:
//! https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

use std::borrow::Cow;

use askama::{FastWritable, NO_VALUES, Template};

/// Implements `type Content` and `bake_content`.
///
/// `recipe_boilerplate!(R)` sets `Content` to `R`'s default content type.
///
/// ```rust
/// use granola::prelude::*;
///
/// #[derive(Default, Debug, Clone)]
/// struct Greeting;
///
/// impl SpanRecipe for Greeting {
///     recipe_boilerplate!(SpanRecipe);
///
///     fn content_recipe(content: &mut Self::Content) {
///         *content = "hello!".into();
///     }
/// }
///
/// let span = HtmlSpan::from(Greeting);
/// assert_eq!(span.bake(), "<span>hello!</span>");
///
/// let span = HtmlSpan::from(Greeting).content("bye!");
/// assert_eq!(span.bake(), "<span>bye!</span>");
/// ```
///
/// `recipe_boilerplate!(R, T)` sets `Content = T`.
/// Requires `T: Into<C>`, where `C` is `R`'s default content type.
///
/// ```rust
/// use askama::Template;
/// use std::borrow::Cow;
///
/// use granola::prelude::*;
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
///     recipe_boilerplate!(SpanRecipe, Hi);
/// }
///
/// let span = HtmlSpan::from(Hi);
///
/// assert_eq!(span.bake(), "<span>hi!</span>");
/// ```
#[macro_export]
macro_rules! recipe_boilerplate {
    ($recipe:path) => {
        type Content = <() as $recipe>::Content;

        fn bake_content(content: Self::Content) -> Self::Content {
            content
        }
    };
    ($recipe:path , $custom_type:ty) => {
        type Content = $custom_type;

        fn bake_content(content: $custom_type) -> <() as $recipe>::Content {
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

/// Content append in-place.
///
/// Folds new content into the existing content, growing it. The bound marks a
/// content type as appendable.
pub trait FoldIn {
    fn fold_in(&mut self, content: Self);
}

impl FoldIn for Cow<'static, str> {
    fn fold_in(&mut self, content: Self) {
        if content.is_empty() {
            return;
        }
        if self.is_empty() {
            *self = content;
        } else {
            self.to_mut().push_str(&content);
        }
    }
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
///     r#"<label for="ode">Notes <textarea id="ode">Exegi monumentum aere perennius</textarea></label>"#
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
            buf.push(' ');
            (&&&content).bake_content(&mut buf);
        })*

        buf
    }};
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
    fn bake_block_1() {
        assert_eq!(bake_block![""], "");
    }

    #[test]
    fn bake_block_2() {
        assert_eq!(bake_block!["single\nitem"], "single\nitem");
    }

    #[test]
    fn bake_block_3() {
        assert_eq!(bake_block!["hello", "world"], "hello world");
    }

    #[test]
    fn bake_block_4() {
        assert_eq!(
            bake_block!["halloween", "hello world"],
            "halloween hello world"
        );
    }

    #[test]
    fn bake_block_5() {
        use crate::prelude::HtmlSpan;

        let span = HtmlSpan::new().content("bar");

        assert_eq!(bake_block!["foo", span, 42], "foo <span>bar</span> 42");
    }
}
