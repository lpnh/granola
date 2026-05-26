use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<em>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/em)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let em: HtmlEm = HtmlEm::empty().id("emphasis");
///
/// assert_eq!(em.bake(), r#"<em id="emphasis"></em>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let owned: HtmlEm = HtmlEm::new("owned");
/// let borrow_checker = bake_inline!("I never said he ", owned, " it.");
///
/// assert_eq!(borrow_checker, r#"I never said he <em>owned</em> it."#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <em
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</em>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = EmRecipe, content = Cow<'static, str>)]
pub struct HtmlEm<R: EmRecipe = ()> {
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

/// Shorthand for `HtmlEm`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let em = em!().id("emphasis");
///
/// assert_eq!(em.bake(), r#"<em id="emphasis"></em>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let owned = em!("owned");
/// let borrow_checker = bake_inline!("I never said he ", owned, " it.");
///
/// assert_eq!(borrow_checker, r#"I never said he <em>owned</em> it."#);
/// ```
#[macro_export]
macro_rules! em {
    () => {
        $crate::html::HtmlEm::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlEm::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlEm::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlEm::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlEm::<()>::new($crate::bake_inline![$($content),+])
    };
}
