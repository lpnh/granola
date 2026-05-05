use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<canvas>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let canvas: HtmlCanvas = HtmlCanvas::empty().id("graphics_canvas");
///
/// assert_eq!(canvas.bake(),
/// r#"<canvas id="graphics_canvas"></canvas>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let content = r#"
/// const ctx = document.getElementById("canvas").getContext("2d");
/// ctx.font = "64px sans";
/// ctx.fillText(":-)", 10, 62);"#;
///
/// let script: HtmlScript = HtmlScript::new(content);
///
/// let canvas: HtmlCanvas = HtmlCanvas::new("ASCII smiley")
///     .id("canvas")
///     .width(160)
///     .height(80);
///
/// let smiley = bake_block![script, canvas];
///
/// assert_eq!(smiley,
/// r#"<script>
///   const ctx = document.getElementById("canvas").getContext("2d");
///   ctx.font = "64px sans";
///   ctx.fillText(":-)", 10, 62);
/// </script>
/// <canvas id="canvas" width="160" height="80">ASCII smiley</canvas>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <canvas
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</canvas>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = CanvasTag, content = Cow<'static, str>, specific = CanvasAttrs)]
pub struct HtmlCanvas<M: CanvasTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub attrs: Attrs,
    pub specific_attrs: CanvasAttrs,
}

/// The HTML `<canvas>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- height | bake_attr("height") -}}
/// {{- width | bake_attr("width") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CanvasAttrs {
    pub height: Option<u32>,
    pub width: Option<u32>,
}

pub trait HasCanvasAttrs: Sized {
    fn canvas_attrs_mut(&mut self) -> &mut CanvasAttrs;

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas#height)
    fn height(mut self, value: u32) -> Self {
        self.canvas_attrs_mut().height = Some(value);
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas#width)
    fn width(mut self, value: u32) -> Self {
        self.canvas_attrs_mut().width = Some(value);
        self
    }
}

impl HasCanvasAttrs for CanvasAttrs {
    fn canvas_attrs_mut(&mut self) -> &mut CanvasAttrs {
        self
    }
}

impl HasCanvasAttrs for &mut CanvasAttrs {
    fn canvas_attrs_mut(&mut self) -> &mut CanvasAttrs {
        self
    }
}

impl<M: CanvasTag> HasCanvasAttrs for HtmlCanvas<M> {
    fn canvas_attrs_mut(&mut self) -> &mut CanvasAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlCanvas`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let canvas = canvas!().id("graphics_canvas");
///
/// assert_eq!(canvas.bake(),
/// r#"<canvas id="graphics_canvas"></canvas>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let content = r#"
/// const ctx = document.getElementById("canvas").getContext("2d");
/// ctx.font = "64px sans";
/// ctx.fillText(":-)", 10, 62);"#;
///
/// let script = script!(content);
///
/// let canvas = canvas!("ASCII smiley").id("canvas").width(160).height(80);
///
/// let smiley = bake_block![script, canvas];
///
/// assert_eq!(smiley,
/// r#"<script>
///   const ctx = document.getElementById("canvas").getContext("2d");
///   ctx.font = "64px sans";
///   ctx.fillText(":-)", 10, 62);
/// </script>
/// <canvas id="canvas" width="160" height="80">ASCII smiley</canvas>"#);
/// ```
#[macro_export]
macro_rules! canvas {
    () => {
        $crate::html::HtmlCanvas::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlCanvas::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlCanvas::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlCanvas::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlCanvas::<()>::new($crate::bake_inline![$($content),+])
    };
}
