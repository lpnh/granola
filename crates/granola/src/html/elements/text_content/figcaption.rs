use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait FigcaptionTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: group, none, presentation
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl FigcaptionTag for () {}

/// The HTML `<figcaption>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figcaption)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let figcaption: HtmlFigcaption = HtmlFigcaption::empty().id("figure_caption");
///
/// assert_eq!(figcaption.bake(),
/// r#"<figcaption id="figure_caption"></figcaption>"#);
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
/// assert_eq!(content,
/// r#"<code>function greet() print("hi!") end</code>
/// <figcaption>Defining a function in Lua</figcaption>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <figcaption
///   {{- global_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</figcaption>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlFigcaption<M: FigcaptionTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: FigcaptionTag> HtmlFigcaption<M> {
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
}

/// Shorthand for `HtmlFigcaption<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let figcaption = figcaption!().id("figure_caption");
///
/// assert_eq!(figcaption.bake(),
/// r#"<figcaption id="figure_caption"></figcaption>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let code = code!(r#"function greet() print("hi!") end"#);
///
/// let figcaption = figcaption!("Defining a function in Lua");
///
/// let content = bake_block![code, figcaption];
///
/// assert_eq!(content,
/// r#"<code>function greet() print("hi!") end</code>
/// <figcaption>Defining a function in Lua</figcaption>"#);
/// ```
#[macro_export]
macro_rules! figcaption {
    () => {
        $crate::html::HtmlFigcaption::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlFigcaption::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlFigcaption::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlFigcaption::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlFigcaption::<()>::new($crate::bake_inline![$($content),+])
    };
}
