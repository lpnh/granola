use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<mark>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/mark)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let mark: HtmlMark = HtmlMark::empty().id("mark_text");
///
/// assert_eq!(mark.bake(),
/// r#"<mark id="mark_text"></mark>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let but_the_clouds: HtmlMark = HtmlMark::new("but the clouds");
///
/// let br: HtmlBr = HtmlBr::new();
///
/// let the_tower = bake_block![
///     bake_inline!["Seem ", but_the_clouds, " of the sky"],
///     br,
///     "When the horizon fades;",
///     br,
///     "Or a bird's sleepy cry",
///     br,
///     "Among the deepening shades."
/// ];
///
/// assert_eq!(the_tower,
/// r#"Seem <mark>but the clouds</mark> of the sky
/// <br />
/// When the horizon fades;
/// <br />
/// Or a bird's sleepy cry
/// <br />
/// Among the deepening shades."#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <mark{{ attrs }}>{{ content | kirei(2) }}</mark>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MarkTag, content = Cow<'static, str>)]
pub struct HtmlMark<M: MarkTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
}

/// Shorthand for `HtmlMark`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let mark = mark!().id("mark_text");
///
/// assert_eq!(mark.bake(),
/// r#"<mark id="mark_text"></mark>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let but_the_clouds = mark!("but the clouds");
///
/// let br = br!();
///
/// let the_tower = bake_block![
///     bake_inline!["Seem ", but_the_clouds, " of the sky"],
///     br,
///     "When the horizon fades;",
///     br,
///     "Or a bird's sleepy cry",
///     br,
///     "Among the deepening shades."
/// ];
///
/// assert_eq!(the_tower,
/// r#"Seem <mark>but the clouds</mark> of the sky
/// <br />
/// When the horizon fades;
/// <br />
/// Or a bird's sleepy cry
/// <br />
/// Among the deepening shades."#);
/// ```
#[macro_export]
macro_rules! mark {
    () => {
        $crate::html::HtmlMark::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlMark::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlMark::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlMark::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlMark::<()>::new($crate::bake_inline![$($content),+])
    };
}
