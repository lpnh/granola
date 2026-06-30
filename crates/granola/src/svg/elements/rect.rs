use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The SVG `<rect />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/rect)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let rect = SvgRect::new().id("rectangle");
///
/// assert_eq!(rect.bake(), r#"<rect id="rectangle" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let rect = SvgRect::new().x("10").y("10").width("100").height("100");
///
/// assert_eq!(
///     rect.bake(),
///     r#"<rect x="10" y="10" width="100" height="100" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <rect
///   {{- global_svg_attrs -}}
///   {{- specific_attrs -}}
///   {{- paint_attrs -}}
///   {{- shape_attrs }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = RectRecipe)]
pub struct SvgRect<R: RectRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_svg_attrs: GlobalSvgAttrs,
    pub specific_attrs: RectAttrs,
    pub paint_attrs: PaintAttrs,
    pub shape_attrs: ShapeAttrs,
}

/// The SVG `<rect>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/rect#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- x | bake_attr("x") -}}
/// {{- y | bake_attr("y") -}}
/// {{- width | bake_attr("width") -}}
/// {{- height | bake_attr("height") -}}
/// {{- rx | bake_attr("rx") -}}
/// {{- ry | bake_attr("ry") -}}
/// {{- path_length | bake_attr("pathLength") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct RectAttrs {
    pub x: Option<Cow<'static, str>>,
    pub y: Option<Cow<'static, str>>,
    pub width: Option<Cow<'static, str>>,
    pub height: Option<Cow<'static, str>>,
    pub rx: Option<Cow<'static, str>>,
    pub ry: Option<Cow<'static, str>>,
    pub path_length: Option<Cow<'static, str>>,
}

pub trait HasRectAttrs: Sized {
    fn rect_attrs_mut(&mut self) -> &mut RectAttrs;

    /// The x coordinate of the rect.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/x)
    fn x(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.rect_attrs_mut().x = Some(value.into());
        self
    }

    /// The y coordinate of the rect.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/y)
    fn y(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.rect_attrs_mut().y = Some(value.into());
        self
    }

    /// The width of the rect.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/width)
    fn width(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.rect_attrs_mut().width = Some(value.into());
        self
    }

    /// The height of the rect.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/height)
    fn height(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.rect_attrs_mut().height = Some(value.into());
        self
    }

    /// The horizontal corner radius of the rect.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/rx)
    fn rx(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.rect_attrs_mut().rx = Some(value.into());
        self
    }

    /// The vertical corner radius of the rect.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/ry)
    fn ry(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.rect_attrs_mut().ry = Some(value.into());
        self
    }

    /// The total length of the rectangle's perimeter, in user units.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/pathLength)
    fn path_length(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.rect_attrs_mut().path_length = Some(value.into());
        self
    }
}

impl HasRectAttrs for RectAttrs {
    fn rect_attrs_mut(&mut self) -> &mut RectAttrs {
        self
    }
}

impl HasRectAttrs for &mut RectAttrs {
    fn rect_attrs_mut(&mut self) -> &mut RectAttrs {
        self
    }
}

impl<R: RectRecipe> HasRectAttrs for SvgRect<R> {
    fn rect_attrs_mut(&mut self) -> &mut RectAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `SvgRect`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let rect = rect!().id("rectangle");
///
/// assert_eq!(rect.bake(), r#"<rect id="rectangle" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let rect = rect!().x("10").y("10").width("100").height("100");
///
/// assert_eq!(
///     rect.bake(),
///     r#"<rect x="10" y="10" width="100" height="100" />"#
/// );
/// ```
#[macro_export]
macro_rules! rect {
    () => {
        $crate::svg::SvgRect::new()
    };

    (@cookbook $r:ty $(,)?) => {
        $crate::svg::SvgRect::<$r>::from_cookbook()
    };
}
