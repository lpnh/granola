use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The SVG `<text>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/text)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let text = SvgText::new().id("text");
///
/// assert_eq!(text.bake(), r#"<text id="text"></text>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let text = SvgText::new().content("Hello, world!").x("20").y("20");
///
/// assert_eq!(text.bake(), r#"<text x="20" y="20">Hello, world!</text>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <text
///   {{- global_svg_attrs -}}
///   {{- specific_attrs -}}
///   {{- paint_attrs -}}
///   {{- text_content_attrs -}}
/// >{{ content | kirei }}</text>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TextRecipe, content = Bake)]
pub struct SvgText<R: TextRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_svg_attrs: GlobalSvgAttrs,
    pub specific_attrs: TextAttrs,
    pub paint_attrs: PaintAttrs,
    pub text_content_attrs: TextContentAttrs,
}

/// The SVG `<text>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/text#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- x | bake_attr("x") -}}
/// {{- y | bake_attr("y") -}}
/// {{- dx | bake_attr("dx") -}}
/// {{- dy | bake_attr("dy") -}}
/// {{- rotate | bake_attr("rotate") -}}
/// {{- length_adjust | bake_attr("lengthAdjust") -}}
/// {{- text_length | bake_attr("textLength") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TextAttrs {
    pub x: Option<Cow<'static, str>>,
    pub y: Option<Cow<'static, str>>,
    pub dx: Option<Cow<'static, str>>,
    pub dy: Option<Cow<'static, str>>,
    pub rotate: Option<Cow<'static, str>>,
    pub length_adjust: Option<Cow<'static, str>>,
    pub text_length: Option<Cow<'static, str>>,
}

pub trait HasTextAttrs: Sized {
    fn text_attrs_mut(&mut self) -> &mut TextAttrs;

    /// The x coordinate of the starting point of the text baseline, or the x
    /// coordinate of each individual glyph if a list of values is provided.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/x)
    fn x(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_attrs_mut().x = Some(value.into());
        self
    }

    /// The y coordinate of the starting point of the text baseline, or the y
    /// coordinate of each individual glyph if a list of values is provided.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/y)
    fn y(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_attrs_mut().y = Some(value.into());
        self
    }

    /// Shifts the text position horizontally from a previous text element, or
    /// shifts the position of each individual glyph if a list of values is
    /// provided.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/dx)
    fn dx(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_attrs_mut().dx = Some(value.into());
        self
    }

    /// Shifts the text position vertically from a previous text element, or
    /// shifts the position of each individual glyph if a list of values is
    /// provided.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/dy)
    fn dy(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_attrs_mut().dy = Some(value.into());
        self
    }

    /// Rotates orientation of each individual glyph.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/rotate)
    fn rotate(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_attrs_mut().rotate = Some(value.into());
        self
    }

    /// How the text is stretched or compressed to fit the width defined by the
    /// `textLength` attribute.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/lengthAdjust)
    fn length_adjust(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_attrs_mut().length_adjust = Some(value.into());
        self
    }

    /// A width that the text should be scaled to fit.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/textLength)
    fn text_length(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_attrs_mut().text_length = Some(value.into());
        self
    }
}

impl HasTextAttrs for TextAttrs {
    fn text_attrs_mut(&mut self) -> &mut TextAttrs {
        self
    }
}

impl HasTextAttrs for &mut TextAttrs {
    fn text_attrs_mut(&mut self) -> &mut TextAttrs {
        self
    }
}

impl<R: TextRecipe> HasTextAttrs for SvgText<R> {
    fn text_attrs_mut(&mut self) -> &mut TextAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `SvgText`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let text = text!().id("text");
///
/// assert_eq!(text.bake(), r#"<text id="text"></text>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let text = text!("Hello, world!").x("20").y("20");
///
/// assert_eq!(text.bake(), r#"<text x="20" y="20">Hello, world!</text>"#);
/// ```
#[macro_export]
macro_rules! text {
    () => {
        $crate::svg::SvgText::new()
    };
    ($content:expr $(,)?) => {
        $crate::svg::SvgText::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::svg::SvgText::new().content($crate::bake_ws![$first $(, $rest)*])
    };

}
