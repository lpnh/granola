use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait StyleTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl StyleTag for () {}

/// The HTML `<style>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let style: HtmlStyle = HtmlStyle::empty().id("style_information");
///
/// assert_eq!(style.bake(),
/// r#"<style id="style_information"></style>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css = r#"
/// p {
///   color: violet;
///   font-weight: lighter;
/// }"#;
///
/// let style: HtmlStyle = HtmlStyle::new(css);
///
/// assert_eq!(style.bake(),
/// r#"<style>
///   p {
///     color: violet;
///     font-weight: lighter;
///   }
/// </style>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <style
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</style>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlStyle<M: StyleTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: StyleTag> HtmlStyle<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    /// Whether the element is potentially render-blocking.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style#blocking)
    pub fn blocking(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("blocking", value);
        self
    }

    /// Applicable media.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style#media)
    pub fn media(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("media", value);
        self
    }
}

/// Shorthand for `HtmlStyle<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let style = style!().id("style_information");
///
/// assert_eq!(style.bake(),
/// r#"<style id="style_information"></style>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css = r#"
/// p {
///   color: violet;
///   font-weight: lighter;
/// }"#;
///
/// let style = style!(css);
///
/// assert_eq!(style.bake(),
/// r#"<style>
///   p {
///     color: violet;
///     font-weight: lighter;
///   }
/// </style>"#);
/// ```
#[macro_export]
macro_rules! style {
    () => {
        $crate::html::HtmlStyle::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlStyle::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlStyle::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlStyle::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlStyle::<()>::new($crate::bake_inline![$($content),+])
    };
}
