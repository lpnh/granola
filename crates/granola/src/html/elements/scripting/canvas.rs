use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait CanvasTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: any
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl CanvasTag for () {}

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
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</canvas>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlCanvas<M: CanvasTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: CanvasTag> HtmlCanvas<M> {
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

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas#height)
    pub fn height(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("height", value.to_string());
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/canvas#width)
    pub fn width(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("width", value.to_string());
        self
    }
}

/// Shorthand for `HtmlCanvas<()>`.
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
