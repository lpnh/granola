use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The SVG `<svg>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/svg)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let svg = Svg::new().id("scalable_vector_graphics");
///
/// assert_eq!(svg.bake(), r#"<svg id="scalable_vector_graphics"></svg>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let shape = r#"M 10,30
///   A 20,20 0,0,1 50,30
///   A 20,20 0,0,1 90,30
///   Q 90,60 50,90
///   Q 10,60 10,30 z"#;
///
/// let path = SvgPath::new().d(shape);
///
/// let svg = Svg::new().content(path);
///
/// assert_eq!(
///     svg.bake(),
///     r#"<svg><path d="M 10,30
///   A 20,20 0,0,1 50,30
///   A 20,20 0,0,1 90,30
///   Q 90,60 50,90
///   Q 10,60 10,30 z" /></svg>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <svg
///   {{- global_svg_attrs -}}
///   {{- specific_attrs -}}
///   {{- paint_attrs -}}
///   {{- shape_attrs -}}
///   {{- text_content_attrs -}}
/// >{{ content | kirei }}</svg>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SvgRecipe, content = Cow<'static, str>)]
pub struct Svg<R: SvgRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_svg_attrs: GlobalSvgAttrs,
    pub specific_attrs: SvgAttrs,
    pub paint_attrs: PaintAttrs,
    pub shape_attrs: ShapeAttrs,
    pub text_content_attrs: TextContentAttrs,
}

/// The SVG `<svg>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/svg#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- height | bake_attr("height") -}}
/// {{- preserve_aspect_ratio | bake_attr("preserveAspectRatio") -}}
/// {{- view_box | bake_attr("viewBox") -}}
/// {{- width | bake_attr("width") -}}
/// {{- x | bake_attr("x") -}}
/// {{- y | bake_attr("y") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct SvgAttrs {
    pub height: Option<Cow<'static, str>>,
    pub preserve_aspect_ratio: Option<Cow<'static, str>>,
    pub view_box: Option<Cow<'static, str>>,
    pub width: Option<Cow<'static, str>>,
    pub x: Option<Cow<'static, str>>,
    pub y: Option<Cow<'static, str>>,
}

pub trait HasSvgAttrs: Sized {
    fn svg_attrs_mut(&mut self) -> &mut SvgAttrs;

    /// The displayed height of the rectangular viewport.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/height)
    fn height(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.svg_attrs_mut().height = Some(value.into());
        self
    }

    /// How the `svg` fragment must be deformed if it is displayed with a
    /// different aspect ratio.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/preserveAspectRatio)
    fn preserve_aspect_ratio(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.svg_attrs_mut().preserve_aspect_ratio = Some(value.into());
        self
    }

    /// The SVG viewport coordinates for the current `svg` fragment.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/viewBox)
    fn view_box(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.svg_attrs_mut().view_box = Some(value.into());
        self
    }

    /// The displayed width of the rectangular viewport.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/width)
    fn width(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.svg_attrs_mut().width = Some(value.into());
        self
    }

    /// The displayed x coordinate of the svg container.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/x)
    fn x(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.svg_attrs_mut().x = Some(value.into());
        self
    }

    /// The displayed y coordinate of the svg container.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/y)
    fn y(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.svg_attrs_mut().y = Some(value.into());
        self
    }
}

impl HasSvgAttrs for SvgAttrs {
    fn svg_attrs_mut(&mut self) -> &mut SvgAttrs {
        self
    }
}

impl HasSvgAttrs for &mut SvgAttrs {
    fn svg_attrs_mut(&mut self) -> &mut SvgAttrs {
        self
    }
}

impl<R: SvgRecipe> HasSvgAttrs for Svg<R> {
    fn svg_attrs_mut(&mut self) -> &mut SvgAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `Svg`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let svg = svg!().id("scalable_vector_graphics");
///
/// assert_eq!(svg.bake(), r#"<svg id="scalable_vector_graphics"></svg>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let shape = r#"M 10,30
///   A 20,20 0,0,1 50,30
///   A 20,20 0,0,1 90,30
///   Q 90,60 50,90
///   Q 10,60 10,30 z"#;
///
/// let path = path!().d(shape);
///
/// let svg = svg!().content(path);
///
/// assert_eq!(
///     svg.bake(),
///     r#"<svg><path d="M 10,30
///   A 20,20 0,0,1 50,30
///   A 20,20 0,0,1 90,30
///   Q 90,60 50,90
///   Q 10,60 10,30 z" /></svg>"#
/// );
/// ```
#[macro_export]
macro_rules! svg {
    () => {
        $crate::svg::Svg::new()
    };
    ($content:expr $(,)?) => {
        $crate::svg::Svg::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::svg::Svg::new().content($crate::bake_block![$first $(, $rest)*])
    };

}
