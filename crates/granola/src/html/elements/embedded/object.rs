use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait ObjectTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: application, document, img
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl ObjectTag for () {}

/// The HTML `<object>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let object: HtmlObject = HtmlObject::empty().id("external_object");
///
/// assert_eq!(object.bake(),
/// r#"<object id="external_object"></object>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let object: HtmlObject = HtmlObject::empty()
///     .mime_type("video/mp4")
///     .data("/videos/flower.mp4")
///     .width(420)
///     .height(420);
///
/// assert_eq!(object.bake(),
/// r#"<object type="video/mp4" data="/videos/flower.mp4" width="420" height="420"></object>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <object
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</object>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlObject<M: ObjectTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ObjectTag> HtmlObject<M> {
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

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#data)
    pub fn data(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("data", value);
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    pub fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("form", value);
        self
    }

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#height)
    pub fn height(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("height", value.to_string());
        self
    }

    /// Name of content navigable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }

    /// Type of embedded resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#type)
    pub fn mime_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value);
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/object#width)
    pub fn width(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("width", value.to_string());
        self
    }
}

/// Shorthand for `HtmlObject<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let object = object!().id("external_object");
///
/// assert_eq!(object.bake(),
/// r#"<object id="external_object"></object>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let object = object!()
///     .mime_type("video/mp4")
///     .data("/videos/flower.mp4")
///     .width(420)
///     .height(420);
///
/// assert_eq!(object.bake(),
/// r#"<object type="video/mp4" data="/videos/flower.mp4" width="420" height="420"></object>"#);
/// ```
#[macro_export]
macro_rules! object {
    () => {
        $crate::html::HtmlObject::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlObject::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlObject::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlObject::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlObject::<()>::new($crate::bake_inline![$($content),+])
    };
}
