use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<address>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/address)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let address = HtmlAddress::new().id("contact_address");
///
/// assert_eq!(
///     address.bake(),
///     r#"<address id="contact_address"></address>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let mail = HtmlA::new()
///     .content("contact@holmes.co.uk")
///     .href("mailto:contact@holmes.co.uk");
///
/// let content = bake!["221B Baker St, London NW1 6XE ·", mail];
///
/// let address = HtmlAddress::new().content(content);
///
/// assert_eq!(
///     address.bake_pretty(),
///     r#"<address>
///   221B Baker St, London NW1 6XE ·<a href="mailto:contact@holmes.co.uk"
///   >contact@holmes.co.uk</a>
/// </address>
/// "#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <address
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</address>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = AddressRecipe, content = Bake)]
pub struct HtmlAddress<R: AddressRecipe = ()> {
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

impl<R: AddressRecipe<Content = Bake>> HtmlAddress<R> {
    pub fn fold_in(mut self, content: impl FastWritable) -> Self {
        self.content.fold_in(content);
        self
    }
}

/// Shorthand for `HtmlAddress`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let address = address!().id("contact_address");
///
/// assert_eq!(
///     address.bake(),
///     r#"<address id="contact_address"></address>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let mail = a!("contact@holmes.co.uk").href("mailto:contact@holmes.co.uk");
///
/// let address = address!("221B Baker St, London NW1 6XE ·", mail);
///
/// assert_eq!(
///     address.bake_pretty(),
///     r#"<address>
///   221B Baker St, London NW1 6XE ·<a href="mailto:contact@holmes.co.uk"
///   >contact@holmes.co.uk</a>
/// </address>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! address {
    () => {
        $crate::html::HtmlAddress::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlAddress::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlAddress::new().content($crate::bake![$first $(, $rest)*])
    };

}
