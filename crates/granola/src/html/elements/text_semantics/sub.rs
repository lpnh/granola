use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<sub>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sub)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let sub = HtmlSub::new().id("subscript");
///
/// assert_eq!(sub.bake(), r#"<sub id="subscript"></sub>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let sub = HtmlSub::new().content("2");
///
/// let water = bake!["H", sub, "O"];
///
/// assert_eq!(water, r#"H<sub>2</sub>O"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <sub
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</sub>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SubRecipe, content = Cow<'static, str>)]
pub struct HtmlSub<R: SubRecipe = ()> {
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

/// Shorthand for `HtmlSub`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sub = sub!().id("subscript");
///
/// assert_eq!(sub.bake(), r#"<sub id="subscript"></sub>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sub = sub!("2");
///
/// let water = bake!["H", sub, "O"];
///
/// assert_eq!(water, r#"H<sub>2</sub>O"#);
/// ```
#[macro_export]
macro_rules! sub {
    () => {
        $crate::html::HtmlSub::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlSub::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSub::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlSub::<($($r,)+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlSub::<($($r,)+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlSub::<($($r,)+)>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
