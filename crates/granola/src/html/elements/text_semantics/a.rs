use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</a>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ATag, content = Cow<'static, str>)]
pub struct HtmlA<R: ATag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// when href attribute is present: button, checkbox, menuitem, menuitemcheckbox,
    /// menuitemradio, option, radio, switch, tab, treeitem
    /// when href attribute is not present: any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: AAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<a>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- href | bake_attr("href") -}}
/// {{- target | bake_attr("target") -}}
/// {{- download | bake_attr("download") -}}
/// {{- hreflang | bake_attr("hreflang") -}}
/// {{- lang | bake_attr("lang") -}}
/// {{- ping | bake_attr("ping") -}}
/// {{- referrerpolicy | bake_attr("referrerpolicy") -}}
/// {{- rel | bake_attr("rel") -}}
/// {{- mime_type | bake_attr("type") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct AAttrs {
    pub href: Option<Cow<'static, str>>,
    pub target: Option<Cow<'static, str>>,
    pub download: Option<Cow<'static, str>>,
    pub hreflang: Option<Cow<'static, str>>,
    pub lang: Option<Cow<'static, str>>,
    pub ping: Option<Cow<'static, str>>,
    pub referrerpolicy: Option<Cow<'static, str>>,
    pub rel: Option<Cow<'static, str>>,
    pub mime_type: Option<Cow<'static, str>>,
}

pub trait HasAAttrs: Sized {
    fn a_attrs_mut(&mut self) -> &mut AAttrs;

    /// Whether to download the resource instead of navigating to it, and its filename if so.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#download)
    fn download(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.a_attrs_mut().download = Some(value.into());
        self
    }

    /// Address of the hyperlink.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#href)
    fn href(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.a_attrs_mut().href = Some(value.into());
        self
    }

    /// Language of the linked resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#hreflang)
    fn hreflang(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.a_attrs_mut().hreflang = Some(value.into());
        self
    }

    /// URLs to ping.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#ping)
    fn ping(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.a_attrs_mut().ping = Some(value.into());
        self
    }

    /// Referrer policy for fetches initiated by the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#referrerpolicy)
    fn referrerpolicy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.a_attrs_mut().referrerpolicy = Some(value.into());
        self
    }

    /// Relationship between the location in the document containing the hyperlink and the destination resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
    fn rel(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.a_attrs_mut().rel = Some(value.into());
        self
    }

    /// Navigable for hyperlink navigation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#target)
    fn target(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.a_attrs_mut().target = Some(value.into());
        self
    }

    /// Hint for the type of the referenced resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/a#type)
    fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.a_attrs_mut().mime_type = Some(value.into());
        self
    }
}

impl HasAAttrs for AAttrs {
    fn a_attrs_mut(&mut self) -> &mut AAttrs {
        self
    }
}

impl HasAAttrs for &mut AAttrs {
    fn a_attrs_mut(&mut self) -> &mut AAttrs {
        self
    }
}

impl<R: ATag> HasAAttrs for HtmlA<R> {
    fn a_attrs_mut(&mut self) -> &mut AAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlA`.
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
