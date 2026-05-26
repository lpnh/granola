use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<aside>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/aside)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let aside: HtmlAside = HtmlAside::empty().id("aside");
///
/// assert_eq!(aside.bake(), r#"<aside id="aside"></aside>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let tip: HtmlStrong = HtmlStrong::new("Tip:");
/// let content: HtmlP = HtmlP::new(bake_inline![tip, " trust your senses more than the timer."]);
///
/// let aside: HtmlAside = HtmlAside::new(bake_newline!(content)).role("note");
///
/// assert_eq!(
///     aside.bake(),
///     r#"<aside role="note">
///   <p><strong>Tip:</strong> trust your senses more than the timer.</p>
/// </aside>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <aside
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</aside>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = AsideTag, content = Cow<'static, str>)]
pub struct HtmlAside<R: AsideTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// feed, none, note, presentation, region, search
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlAside`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let aside = aside!().id("aside");
///
/// assert_eq!(aside.bake(), r#"<aside id="aside"></aside>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let tip = strong!("Tip:");
/// let content = p!(@inline tip, " trust your senses more than the timer.");
///
/// let aside = aside!(@newline content).role("note");
///
/// assert_eq!(aside.bake(),
/// r#"<aside role="note">
///   <p><strong>Tip:</strong> trust your senses more than the timer.</p>
/// </aside>"#);
/// ```
#[macro_export]
macro_rules! aside {
    () => {
        $crate::html::HtmlAside::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlAside::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlAside::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlAside::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlAside::<()>::new($crate::bake_inline![$($content),+])
    };
}
