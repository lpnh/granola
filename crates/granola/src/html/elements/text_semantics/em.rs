use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let em = HtmlEm::new().id("emphasis");
///
/// assert_eq!(em.bake(), r#"<em id="emphasis"></em>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let owned = HtmlEm::new().content("owned");
/// let borrow_checker = bake!("I never said he ", owned, " it.");
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
/// >{{ content | kirei }}</em>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = EmRecipe, content = Bake)]
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
/// let borrow_checker = bake!("I never said he ", owned, " it.");
///
/// assert_eq!(borrow_checker, r#"I never said he <em>owned</em> it."#);
/// ```
#[macro_export]
macro_rules! em {
    () => {
        $crate::html::HtmlEm::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlEm::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlEm::new().content($crate::bake![$first $(, $rest)*])
    };
}
