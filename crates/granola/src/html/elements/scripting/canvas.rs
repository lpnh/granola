use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let canvas = HtmlCanvas::new().id("graphics_canvas");
///
/// assert_eq!(canvas.bake(), r#"<canvas id="graphics_canvas"></canvas>"#);
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
/// let script = HtmlScript::new().content(content);
///
/// let canvas = HtmlCanvas::new()
///     .content("ASCII smiley")
///     .id("canvas")
///     .width(160)
///     .height(80);
///
/// let smiley = bake![script, canvas];
///
/// assert_eq!(
///     smiley,
///     r#"<script>
/// const ctx = document.getElementById("canvas").getContext("2d");
/// ctx.font = "64px sans";
/// ctx.fillText(":-)", 10, 62);</script><canvas id="canvas" width="160" height="80">ASCII smiley</canvas>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <canvas
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</canvas>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = CanvasRecipe, content = Bake)]
pub struct HtmlCanvas<R: CanvasRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: CanvasAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<canvas>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- width | bake_attr("width") -}}
/// {{- height | bake_attr("height") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CanvasAttrs {
    pub width: Option<u32>,
    pub height: Option<u32>,
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

impl<R: CanvasRecipe> HasCanvasAttrs for HtmlCanvas<R> {
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
/// assert_eq!(canvas.bake(), r#"<canvas id="graphics_canvas"></canvas>"#);
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
/// let smiley = bake![script, canvas];
///
/// assert_eq!(
///     smiley,
///     r#"<script>
/// const ctx = document.getElementById("canvas").getContext("2d");
/// ctx.font = "64px sans";
/// ctx.fillText(":-)", 10, 62);</script><canvas id="canvas" width="160" height="80">ASCII smiley</canvas>"#
/// );
/// ```
#[macro_export]
macro_rules! canvas {
    () => {
        $crate::html::HtmlCanvas::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlCanvas::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlCanvas::new().content($crate::bake![$first $(, $rest)*])
    };

}
