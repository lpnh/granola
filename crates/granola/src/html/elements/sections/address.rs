use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait AddressTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl AddressTag for () {}

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
/// <address
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</address>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlAddress<M: AddressTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: AddressTag> HtmlAddress<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }
}

/// Shorthand for `HtmlAddress<()>`.
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
