use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</figcaption>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = FigcaptionTag, content = Cow<'static, str>)]
pub struct HtmlFigcaption<R: FigcaptionTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// group, none, presentation
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlFigcaption`.
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
