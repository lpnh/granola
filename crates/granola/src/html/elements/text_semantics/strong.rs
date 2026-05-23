use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<strong>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/strong)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let strong: HtmlStrong = HtmlStrong::empty().id("strong_importance");
///
/// assert_eq!(strong.bake(),
/// r#"<strong id="strong_importance"></strong>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let strong: HtmlStrong = HtmlStrong::new("Do not feed the trolls.");
///
/// assert_eq!(strong.bake(),
/// r#"<strong>Do not feed the trolls.</strong>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <strong
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</strong>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = StrongTag, content = Cow<'static, str>)]
pub struct HtmlStrong<R: StrongTag = ()> {
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

/// Shorthand for `HtmlStrong`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let strong = strong!().id("strong_importance");
///
/// assert_eq!(strong.bake(),
/// r#"<strong id="strong_importance"></strong>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let strong = strong!("Do not feed the trolls.");
///
/// assert_eq!(strong.bake(),
/// r#"<strong>Do not feed the trolls.</strong>"#);
/// ```
#[macro_export]
macro_rules! strong {
    () => {
        $crate::html::HtmlStrong::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlStrong::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlStrong::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlStrong::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlStrong::<()>::new($crate::bake_inline![$($content),+])
    };
}
