use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<sup>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/sup)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let sup = HtmlSup::new().id("superscript");
///
/// assert_eq!(sup.bake(), r#"<sup id="superscript"></sup>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let sup = HtmlSup::new().content("e");
///
/// let anniv = bake_inline!["100", sup, " anniversaire"];
///
/// assert_eq!(anniv, r#"100<sup>e</sup> anniversaire"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <sup
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</sup>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SupRecipe, content = Cow<'static, str>)]
pub struct HtmlSup<R: SupRecipe = ()> {
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

/// Shorthand for `HtmlSup`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sup = sup!().id("superscript");
///
/// assert_eq!(sup.bake(), r#"<sup id="superscript"></sup>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sup = sup!("e");
///
/// let anniv = bake_inline!["100", sup, " anniversaire"];
///
/// assert_eq!(anniv, r#"100<sup>e</sup> anniversaire"#);
/// ```
#[macro_export]
macro_rules! sup {
    () => {
        $crate::html::HtmlSup::new()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSup::new().content($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSup::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSup::new().content($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSup::new().content($crate::bake_inline![$($content),+])
    };
}
