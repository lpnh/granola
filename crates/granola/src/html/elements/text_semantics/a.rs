use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait ATag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles:
    ///     when href attribute is present: button, checkbox, menuitem, menuitemcheckbox,
    ///     menuitemradio, option, radio, switch, tab, treeitem
    ///     when href attribute is not present: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl ATag for () {}

/// The HTML `<a>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let a: HtmlA = HtmlA::empty().id("anchor");
///
/// assert_eq!(a.bake(),
/// r#"<a id="anchor"></a>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let a: HtmlA = HtmlA::new("docs").href("https://askama.rs");
///
/// assert_eq!(a.bake(),
/// r#"<a href="https://askama.rs">docs</a>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <a
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</a>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlA<M: ATag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ATag> HtmlA<M> {
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

    /// Whether to download the resource instead of navigating to it, and its filename if so.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#download)
    pub fn download(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("download", value);
        self
    }

    /// Address of the hyperlink.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#href)
    pub fn href(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("href", value);
        self
    }

    /// Language of the linked resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#hreflang)
    pub fn hreflang(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("hreflang", value);
        self
    }

    /// URLs to ping.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#ping)
    pub fn ping(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("ping", value);
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#referrerpolicy)
    pub fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("referrerpolicy", value);
        self
    }

    /// Relationship between the location in the document containing the hyperlink and the destination resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
    pub fn rel(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("rel", value);
        self
    }

    /// Navigable for hyperlink navigation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#target)
    pub fn target(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("target", value);
        self
    }

    /// Hint for the type of the referenced resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#type)
    pub fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value);
        self
    }
}

/// Shorthand for `HtmlA<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let a = a!().id("anchor");
///
/// assert_eq!(a.bake(),
/// r#"<a id="anchor"></a>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let a = a!("docs").href("https://askama.rs");
///
/// assert_eq!(a.bake(),
/// r#"<a href="https://askama.rs">docs</a>"#);
/// ```
#[macro_export]
macro_rules! a {
    () => {
        $crate::html::HtmlA::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlA::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlA::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlA::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlA::<()>::new($crate::bake_inline![$($content),+])
    };
}
