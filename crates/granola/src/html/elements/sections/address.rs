use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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
/// let address: HtmlAddress = HtmlAddress::empty().id("contact_address");
///
/// assert_eq!(address.bake(),
/// r#"<address id="contact_address"></address>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let mail: HtmlA = HtmlA::new("contact@holmes.co.uk").href("mailto:contact@holmes.co.uk");
///
/// let content = bake_block!["221B Baker St, London NW1 6XE ·", mail];
///
/// let address: HtmlAddress = HtmlAddress::new(content);
///
/// assert_eq!(address.bake(),
/// r#"<address>
///   221B Baker St, London NW1 6XE ·
///   <a href="mailto:contact@holmes.co.uk">contact@holmes.co.uk</a>
/// </address>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <address{{ attrs }}>{{ content | kirei(2) }}</address>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = AddressTag, content = Cow<'static, str>)]
pub struct HtmlAddress<M: AddressTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
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
/// assert_eq!(address.bake(),
/// r#"<address id="contact_address"></address>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let mail = a!("contact@holmes.co.uk").href("mailto:contact@holmes.co.uk");
///
/// let address = address!("221B Baker St, London NW1 6XE ·", mail);
///
/// assert_eq!(address.bake(),
/// r#"<address>
///   221B Baker St, London NW1 6XE ·
///   <a href="mailto:contact@holmes.co.uk">contact@holmes.co.uk</a>
/// </address>"#);
/// ```
#[macro_export]
macro_rules! address {
    () => {
        $crate::html::HtmlAddress::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlAddress::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlAddress::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlAddress::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlAddress::<()>::new($crate::bake_inline![$($content),+])
    };
}
