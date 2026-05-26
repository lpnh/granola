use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// assert_eq!(style.bake(), r#"<style id="style_information"></style>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_rule: CssRule = CssRule::new("p", [("color", "violet"), ("font-weight", "lighter")]);
///
/// let style: HtmlStyle = HtmlStyle::new(css_rule);
///
/// assert_eq!(
///     style.bake(),
///     r#"<style>
///   p {
///     color: violet;
///     font-weight: lighter;
///   }
/// </style>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <style
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</style>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = StyleRecipe, content = CssStylesheet)]
pub struct HtmlStyle<R: StyleRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: StyleAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<style>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- blocking | bake_attr("blocking") -}}
/// {{- media | bake_attr("media") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct StyleAttrs {
    pub blocking: Option<Cow<'static, str>>,
    pub media: Option<Cow<'static, str>>,
}

pub trait HasStyleAttrs: Sized {
    fn style_attrs_mut(&mut self) -> &mut StyleAttrs;

    /// Whether the element is potentially render-blocking.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style#blocking)
    fn blocking(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.style_attrs_mut().blocking = Some(value.into());
        self
    }

    /// Applicable media.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/style#media)
    fn media(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.style_attrs_mut().media = Some(value.into());
        self
    }
}

impl HasStyleAttrs for StyleAttrs {
    fn style_attrs_mut(&mut self) -> &mut StyleAttrs {
        self
    }
}

impl HasStyleAttrs for &mut StyleAttrs {
    fn style_attrs_mut(&mut self) -> &mut StyleAttrs {
        self
    }
}

impl<R: StyleRecipe> HasStyleAttrs for HtmlStyle<R> {
    fn style_attrs_mut(&mut self) -> &mut StyleAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlStyle`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let style = style!().id("style_information");
///
/// assert_eq!(style.bake(), r#"<style id="style_information"></style>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let css_rule = rule!(
///     "p";
///     ("color", "violet"),
///     ("font-weight", "lighter"),
/// );
///
/// let style = style!(css_rule);
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
