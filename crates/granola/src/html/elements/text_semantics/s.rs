use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<s>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/s)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let s = HtmlS::new().id("strikethrough");
///
/// assert_eq!(s.bake(), r#"<s id="strikethrough"></s>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let nine = HtmlS::new().content("nine");
///
/// let solar_system = bake!["Our solar system has ", nine, " eight planets"];
///
/// assert_eq!(
///     solar_system,
///     r#"Our solar system has <s>nine</s> eight planets"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <s
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</s>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SRecipe, content = Cow<'static, str>)]
pub struct HtmlS<R: SRecipe = ()> {
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

/// Shorthand for `HtmlS`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let s = s!().id("strikethrough");
///
/// assert_eq!(s.bake(), r#"<s id="strikethrough"></s>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let nine = s!("nine");
///
/// let solar_system = bake!["Our solar system has ", nine, " eight planets"];
///
/// assert_eq!(
///     solar_system,
///     r#"Our solar system has <s>nine</s> eight planets"#
/// );
/// ```
#[macro_export]
macro_rules! s {
    () => {
        $crate::html::HtmlS::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlS::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlS::new().content($crate::bake![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlS::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlS::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlS::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
