use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<dfn>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dfn)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dfn: HtmlDfn = HtmlDfn::empty().id("definition");
///
/// assert_eq!(dfn.bake(), r#"<dfn id="definition"></dfn>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let corro: HtmlDfn = HtmlDfn::new("Corro");
///
/// let about = bake_inline![corro, " the Unsafe Rusturchin"];
/// let paragraph: HtmlP = HtmlP::new(about);
///
/// assert_eq!(
///     paragraph.bake(),
///     r#"<p><dfn>Corro</dfn> the Unsafe Rusturchin</p>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <dfn
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</dfn>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DfnRecipe, content = Cow<'static, str>)]
pub struct HtmlDfn<R: DfnRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlDfn`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dfn = dfn!().id("definition");
///
/// assert_eq!(dfn.bake(), r#"<dfn id="definition"></dfn>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let corro = dfn!("Corro");
///
/// let paragraph = p!(@inline corro, " the Unsafe Rusturchin");
///
/// assert_eq!(paragraph.bake(),
/// r#"<p><dfn>Corro</dfn> the Unsafe Rusturchin</p>"#);
/// ```
#[macro_export]
macro_rules! dfn {
    () => {
        $crate::html::HtmlDfn::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDfn::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDfn::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDfn::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDfn::<()>::new($crate::bake_inline![$($content),+])
    };
}
