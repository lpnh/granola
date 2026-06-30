use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<u>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/u)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let u = HtmlU::new().id("unarticulated_annotation");
///
/// assert_eq!(u.bake(), r#"<u id="unarticulated_annotation"></u>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let wowwd = HtmlU::new().content("world");
///
/// let hewwo_wowwd = bake!["hewwo, ", wowwd, "!"];
///
/// assert_eq!(hewwo_wowwd, r#"hewwo, <u>world</u>!"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <u
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</u>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = URecipe, content = Cow<'static, str>)]
pub struct HtmlU<R: URecipe = ()> {
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

/// Shorthand for `HtmlU`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let u = u!().id("unarticulated_annotation");
///
/// assert_eq!(u.bake(), r#"<u id="unarticulated_annotation"></u>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let wowwd = u!("world");
///
/// let hewwo_wowwd = bake!["hewwo, ", wowwd, "!"];
///
/// assert_eq!(hewwo_wowwd, r#"hewwo, <u>world</u>!"#);
/// ```
#[macro_export]
macro_rules! u {
    () => {
        $crate::html::HtmlU::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlU::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlU::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlU::<($($r,)+)>::from_cookbook()
    };
    (@cookbook $r:ty ; $content:expr $(,)?) => {
        $crate::html::HtmlU::<($($r,)+)>::from_cookbook().content($content)
    };
    (@cookbook $r:ty ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlU::<($($r,)+)>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
