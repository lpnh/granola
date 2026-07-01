use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The SVG `<path />` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/path)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let path = SvgPath::new().id("shape");
///
/// assert_eq!(path.bake(), r#"<path id="shape" />"#);
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
/// assert_eq!(
///     path.bake(),
///     r#"<path d="M 10,30
///   A 20,20 0,0,1 50,30
///   A 20,20 0,0,1 90,30
///   Q 90,60 50,90
///   Q 10,60 10,30 z" />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <path
///   {{- global_svg_attrs -}}
///   {{- specific_attrs -}}
///   {{- paint_attrs -}}
///   {{- shape_attrs }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = PathRecipe)]
pub struct SvgPath<R: PathRecipe = ()> {
    _recipe: PhantomData<R>,
    pub global_svg_attrs: GlobalSvgAttrs,
    pub specific_attrs: PathAttrs,
    pub paint_attrs: PaintAttrs,
    pub shape_attrs: ShapeAttrs,
}

/// The SVG `<path>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Element/path#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- d | bake_attr("d") -}}
/// {{- path_length | bake_attr("pathLength") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct PathAttrs {
    pub d: Option<Cow<'static, str>>,
    pub path_length: Option<Cow<'static, str>>,
}

pub trait HasPathAttrs: Sized {
    fn path_attrs_mut(&mut self) -> &mut PathAttrs;

    /// Defines the shape of the path.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/d)
    fn d(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.path_attrs_mut().d = Some(value.into());
        self
    }

    /// The total length for the path, in user units.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/SVG/Reference/Attribute/pathLength)
    fn path_length(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.path_attrs_mut().path_length = Some(value.into());
        self
    }
}

impl HasPathAttrs for PathAttrs {
    fn path_attrs_mut(&mut self) -> &mut PathAttrs {
        self
    }
}

impl HasPathAttrs for &mut PathAttrs {
    fn path_attrs_mut(&mut self) -> &mut PathAttrs {
        self
    }
}

impl<R: PathRecipe> HasPathAttrs for SvgPath<R> {
    fn path_attrs_mut(&mut self) -> &mut PathAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `SvgPath`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let path = path!().id("shape");
///
/// assert_eq!(path.bake(), r#"<path id="shape" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let shape = r#"M 10,30
///     A 20,20 0,0,1 50,30
///     A 20,20 0,0,1 90,30
///     Q 90,60 50,90
///     Q 10,60 10,30 z"#;
///
/// let path = path!().d(shape);
///
/// assert_eq!(
///     path.bake(),
///     r#"<path d="M 10,30
///     A 20,20 0,0,1 50,30
///     A 20,20 0,0,1 90,30
///     Q 90,60 50,90
///     Q 10,60 10,30 z" />"#
/// );
/// ```
#[macro_export]
macro_rules! path {
    () => {
        $crate::svg::SvgPath::new()
    };
}
