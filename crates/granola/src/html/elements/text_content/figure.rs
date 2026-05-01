use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// With figcaption descendant: no permitted roles
/// Otherwise: any
pub trait FigureTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlFigure<Self>) -> HtmlFigure<Self> {
        element
    }
}

impl FigureTag for () {}

/// The HTML `<figure>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figure)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let figure: HtmlFigure = HtmlFigure::empty().id("figure_with_optional_caption");
///
/// assert_eq!(figure.bake(),
/// r#"<figure id="figure_with_optional_caption"></figure>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let code: HtmlCode = HtmlCode::new(r#"function greet() print("hi!") end"#);
///
/// let figcaption: HtmlFigcaption = HtmlFigcaption::new("Defining a function in Lua");
///
/// let content = bake_block![code, figcaption];
///
/// let figure: HtmlFigure = HtmlFigure::new(content);
///
/// assert_eq!(figure.bake(),
/// r#"<figure>
///   <code>function greet() print("hi!") end</code>
///   <figcaption>Defining a function in Lua</figcaption>
/// </figure>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <figure
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</figure>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlFigure<M: FigureTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: FigureTag> HtmlFigure<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }
}

/// Shorthand for `HtmlFigure<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let figure = figure!().id("figure_with_optional_caption");
///
/// assert_eq!(figure.bake(),
/// r#"<figure id="figure_with_optional_caption"></figure>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let code = code!(r#"function greet() print("hi!") end"#);
///
/// let figcaption = figcaption!("Defining a function in Lua");
///
/// let figure = figure!(code, figcaption);
///
/// assert_eq!(figure.bake(),
/// r#"<figure>
///   <code>function greet() print("hi!") end</code>
///   <figcaption>Defining a function in Lua</figcaption>
/// </figure>"#);
/// ```
#[macro_export]
macro_rules! figure {
    () => {
        $crate::html::HtmlFigure::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlFigure::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlFigure::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlFigure::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlFigure::<()>::new($crate::bake_inline![$($content),+])
    };
}
