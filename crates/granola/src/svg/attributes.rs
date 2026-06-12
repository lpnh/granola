use askama::Template;
use std::borrow::Cow;

use crate::filters;

/// The SVG core attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute#core_attributes)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let rect = SvgRect::new()
///     .id("small_rect")
///     .x("10")
///     .y("10")
///     .width("100")
///     .height("100");
///
/// assert_eq!(
///     rect.bake(),
///     r#"<rect id="small_rect" x="10" y="10" width="100" height="100" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{- id | bake_attr("id") -}}
/// {{- class | bake_attr("class") -}}
/// {{- lang | bake_attr("lang") -}}
/// {{- style | bake_attr("style") -}}
/// {{- tabindex | bake_attr("tabindex") -}}
/// {{- autofocus | bake_bool_attr("autofocus") -}}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct GlobalSvgAttrs {
    pub id: Option<Cow<'static, str>>,
    pub class: Option<Cow<'static, str>>,
    pub lang: Option<Cow<'static, str>>,
    pub style: Option<Cow<'static, str>>,
    pub tabindex: Option<i64>,
    pub autofocus: bool,
}

impl HasGlobalSvgAttrs for &mut GlobalSvgAttrs {
    fn global_svg_attrs_mut(&mut self) -> &mut GlobalSvgAttrs {
        self
    }
}

pub trait HasGlobalSvgAttrs: Sized {
    fn global_svg_attrs_mut(&mut self) -> &mut GlobalSvgAttrs;

    /// Automatically focus the element when the page is loaded.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/autofocus)
    fn autofocus(mut self) -> Self {
        self.global_svg_attrs_mut().autofocus = true;
        self
    }

    /// Classes to which the element belongs.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/class)
    fn class(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        let new = value.into();
        let ga = self.global_svg_attrs_mut();
        ga.class = Some(match ga.class.take() {
            None => new,
            Some(existing) => format!("{existing} {new}").into(),
        });
        self
    }

    /// The element's ID.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/id)
    fn id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_svg_attrs_mut().id = Some(value.into());
        self
    }

    /// Language of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/lang)
    fn lang(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_svg_attrs_mut().lang = Some(value.into());
        self
    }

    /// Presentational and formatting instructions.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/style)
    fn style(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        let new = value.into();
        let ga = self.global_svg_attrs_mut();
        ga.style = Some(match ga.style.take() {
            None => new,
            Some(existing) => format!("{existing} {new}").into(),
        });
        self
    }

    /// Whether the element is focusable and sequentially focusable, and the
    /// relative order of the element for the purposes of sequential focus
    /// navigation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/tabindex)
    fn tabindex(mut self, value: i64) -> Self {
        self.global_svg_attrs_mut().tabindex = Some(value);
        self
    }
}

/// The presentation attributes applied to shapes and text content elements.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute#presentation_attributes)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let rect = SvgRect::new().fill("coral").stroke("maroon");
///
/// assert_eq!(rect.bake(), r#"<rect fill="coral" stroke="maroon" />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// {{- fill | bake_attr("fill") -}}
/// {{- fill_opacity | bake_attr("fill-opacity") -}}
/// {{- fill_rule | bake_attr("fill-rule") -}}
/// {{- paint_order | bake_attr("paint-order") -}}
/// {{- stroke | bake_attr("stroke") -}}
/// {{- stroke_dasharray | bake_attr("stroke-dasharray") -}}
/// {{- stroke_dashoffset | bake_attr("stroke-dashoffset") -}}
/// {{- stroke_linecap | bake_attr("stroke-linecap") -}}
/// {{- stroke_linejoin | bake_attr("stroke-linejoin") -}}
/// {{- stroke_miterlimit | bake_attr("stroke-miterlimit") -}}
/// {{- stroke_opacity | bake_attr("stroke-opacity") -}}
/// {{- stroke_width | bake_attr("stroke-width") -}}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct PaintAttrs {
    pub fill: Option<Cow<'static, str>>,
    pub fill_opacity: Option<Cow<'static, str>>,
    pub fill_rule: Option<Cow<'static, str>>,
    pub paint_order: Option<Cow<'static, str>>,
    pub stroke: Option<Cow<'static, str>>,
    pub stroke_dasharray: Option<Cow<'static, str>>,
    pub stroke_dashoffset: Option<Cow<'static, str>>,
    pub stroke_linecap: Option<Cow<'static, str>>,
    pub stroke_linejoin: Option<Cow<'static, str>>,
    pub stroke_miterlimit: Option<Cow<'static, str>>,
    pub stroke_opacity: Option<Cow<'static, str>>,
    pub stroke_width: Option<Cow<'static, str>>,
}

impl HasPaintAttrs for &mut PaintAttrs {
    fn paint_attrs_mut(&mut self) -> &mut PaintAttrs {
        self
    }
}

pub trait HasPaintAttrs: Sized {
    fn paint_attrs_mut(&mut self) -> &mut PaintAttrs;

    /// The color used to paint the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/fill)
    fn fill(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().fill = Some(value.into());
        self
    }

    /// The opacity of the paint applied to the fill of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/fill-opacity)
    fn fill_opacity(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().fill_opacity = Some(value.into());
        self
    }

    /// The algorithm determining which parts of the element are inside its
    /// fill.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/fill-rule)
    fn fill_rule(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().fill_rule = Some(value.into());
        self
    }

    /// The order in which the fill, stroke, and markers of the element are
    /// painted.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/paint-order)
    fn paint_order(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().paint_order = Some(value.into());
        self
    }

    /// The color used to paint the outline of the shape.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke)
    fn stroke(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().stroke = Some(value.into());
        self
    }

    /// The pattern of dashes and gaps used to paint the outline of the shape.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke-dasharray)
    fn stroke_dasharray(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().stroke_dasharray = Some(value.into());
        self
    }

    /// The offset on the rendering of the associated dash array.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke-dashoffset)
    fn stroke_dashoffset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().stroke_dashoffset = Some(value.into());
        self
    }

    /// The shape to be used at the end of open subpaths when they are stroked.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke-linecap)
    fn stroke_linecap(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().stroke_linecap = Some(value.into());
        self
    }

    /// The shape to be used at the corners of paths when they are stroked.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke-linejoin)
    fn stroke_linejoin(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().stroke_linejoin = Some(value.into());
        self
    }

    /// The limit on the ratio of the miter length to the `stroke-width` used to
    /// draw a miter join.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke-miterlimit)
    fn stroke_miterlimit(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().stroke_miterlimit = Some(value.into());
        self
    }

    /// The opacity of the paint applied to the stroke of a shape.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke-opacity)
    fn stroke_opacity(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().stroke_opacity = Some(value.into());
        self
    }

    /// The width of the stroke applied to the shape.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/stroke-width)
    fn stroke_width(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.paint_attrs_mut().stroke_width = Some(value.into());
        self
    }
}

/// The presentation attributes applied to shapes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute#presentation_attributes)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let path = SvgPath::new().d("M 10,10 H 90").marker_end("url(#arrow)");
///
/// assert_eq!(
///     path.bake(),
///     r#"<path d="M 10,10 H 90" marker-end="url(#arrow)" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{- marker_start | bake_attr("marker-start") -}}
/// {{- marker_mid | bake_attr("marker-mid") -}}
/// {{- marker_end | bake_attr("marker-end") -}}
/// {{- shape_rendering | bake_attr("shape-rendering") -}}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ShapeAttrs {
    pub marker_start: Option<Cow<'static, str>>,
    pub marker_mid: Option<Cow<'static, str>>,
    pub marker_end: Option<Cow<'static, str>>,
    pub shape_rendering: Option<Cow<'static, str>>,
}

impl HasShapeAttrs for &mut ShapeAttrs {
    fn shape_attrs_mut(&mut self) -> &mut ShapeAttrs {
        self
    }
}

pub trait HasShapeAttrs: Sized {
    fn shape_attrs_mut(&mut self) -> &mut ShapeAttrs;

    /// The marker drawn at the first vertex of the shape.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/marker-start)
    fn marker_start(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.shape_attrs_mut().marker_start = Some(value.into());
        self
    }

    /// The marker drawn at the inner vertices of the shape.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/marker-mid)
    fn marker_mid(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.shape_attrs_mut().marker_mid = Some(value.into());
        self
    }

    /// The marker drawn at the final vertex of the shape.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/marker-end)
    fn marker_end(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.shape_attrs_mut().marker_end = Some(value.into());
        self
    }

    /// Hints about what tradeoffs to make as the shape is rendered.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/shape-rendering)
    fn shape_rendering(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.shape_attrs_mut().shape_rendering = Some(value.into());
        self
    }
}

/// The presentation attributes applied to text content elements.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute#presentation_attributes)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let text = SvgText::new()
///     .content("Hello, world!")
///     .text_anchor("middle");
///
/// assert_eq!(
///     text.bake(),
///     r#"<text text-anchor="middle">Hello, world!</text>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// {{- direction | bake_attr("direction") -}}
/// {{- dominant_baseline | bake_attr("dominant-baseline") -}}
/// {{- font_family | bake_attr("font-family") -}}
/// {{- font_size | bake_attr("font-size") -}}
/// {{- font_size_adjust | bake_attr("font-size-adjust") -}}
/// {{- font_stretch | bake_attr("font-stretch") -}}
/// {{- font_style | bake_attr("font-style") -}}
/// {{- font_variant | bake_attr("font-variant") -}}
/// {{- font_weight | bake_attr("font-weight") -}}
/// {{- text_anchor | bake_attr("text-anchor") -}}
/// {{- text_decoration | bake_attr("text-decoration") -}}
/// {{- text_overflow | bake_attr("text-overflow") -}}
/// {{- text_rendering | bake_attr("text-rendering") -}}
/// {{- white_space | bake_attr("white-space") -}}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TextContentAttrs {
    pub direction: Option<Cow<'static, str>>,
    pub dominant_baseline: Option<Cow<'static, str>>,
    pub font_family: Option<Cow<'static, str>>,
    pub font_size: Option<Cow<'static, str>>,
    pub font_size_adjust: Option<Cow<'static, str>>,
    pub font_stretch: Option<Cow<'static, str>>,
    pub font_style: Option<Cow<'static, str>>,
    pub font_variant: Option<Cow<'static, str>>,
    pub font_weight: Option<Cow<'static, str>>,
    pub text_anchor: Option<Cow<'static, str>>,
    pub text_decoration: Option<Cow<'static, str>>,
    pub text_overflow: Option<Cow<'static, str>>,
    pub text_rendering: Option<Cow<'static, str>>,
    pub white_space: Option<Cow<'static, str>>,
}

impl HasTextContentAttrs for &mut TextContentAttrs {
    fn text_content_attrs_mut(&mut self) -> &mut TextContentAttrs {
        self
    }
}

pub trait HasTextContentAttrs: Sized {
    fn text_content_attrs_mut(&mut self) -> &mut TextContentAttrs;

    /// The base direction of the text.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/direction)
    fn direction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().direction = Some(value.into());
        self
    }

    /// The baseline used to align the text.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/dominant-baseline)
    fn dominant_baseline(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().dominant_baseline = Some(value.into());
        self
    }

    /// Indicates which font family will be used to render the text.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/font-family)
    fn font_family(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().font_family = Some(value.into());
        self
    }

    /// The size of the font from baseline to baseline when multiple lines of
    /// text are set solid in a multiline layout environment.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/font-size)
    fn font_size(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().font_size = Some(value.into());
        self
    }

    /// Aspect value for an element that will preserve the x-height of the first
    /// choice font in a substitute font.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/font-size-adjust)
    fn font_size_adjust(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().font_size_adjust = Some(value.into());
        self
    }

    /// Indicates the desired amount of condensing or expansion in the glyphs
    /// used to render the text.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/font-stretch)
    fn font_stretch(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().font_stretch = Some(value.into());
        self
    }

    /// Whether the text is to be rendered using a normal, italic, or oblique
    /// face.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/font-style)
    fn font_style(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().font_style = Some(value.into());
        self
    }

    /// Indicates whether the text is to be rendered using variations of the
    /// font's glyphs.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/font-variant)
    fn font_variant(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().font_variant = Some(value.into());
        self
    }

    /// The boldness or lightness of the glyphs used to render the text,
    /// relative to other fonts in the same font family.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/font-weight)
    fn font_weight(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().font_weight = Some(value.into());
        self
    }

    /// Align a string of pre-formatted text or auto-wrapped text where the
    /// wrapping area is determined from the `inline-size` property relative
    /// to a given point.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/text-anchor)
    fn text_anchor(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().text_anchor = Some(value.into());
        self
    }

    /// Decorative lines added to the text.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/text-decoration)
    fn text_decoration(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().text_decoration = Some(value.into());
        self
    }

    /// How text content block elements render when text overflows line boxes.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/text-overflow)
    fn text_overflow(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().text_overflow = Some(value.into());
        self
    }

    /// Hints to the renderer about what tradeoffs to make when rendering text.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/text-rendering)
    fn text_rendering(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().text_rendering = Some(value.into());
        self
    }

    /// How white space inside the text is handled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/white-space)
    fn white_space(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.text_content_attrs_mut().white_space = Some(value.into());
        self
    }
}
