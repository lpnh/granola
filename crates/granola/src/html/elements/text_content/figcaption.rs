use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let figcaption = HtmlFigcaption::new().id("figure_caption");
///
/// assert_eq!(
///     figcaption.bake(),
///     r#"<figcaption id="figure_caption"></figcaption>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let code = HtmlCode::new().content(r#"function greet() print("hi!") end"#);
///
/// let figcaption = HtmlFigcaption::new().content("Defining a function in Lua");
///
/// let content = bake![code, figcaption];
///
/// assert_eq!(
///     content,
///     r#"<code>function greet() print("hi!") end</code><figcaption>Defining a function in Lua</figcaption>"#
/// );
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
/// >{{ content | kirei }}</figcaption>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = FigcaptionRecipe, content = Bake)]
pub struct HtmlFigcaption<R: FigcaptionRecipe = ()> {
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
/// assert_eq!(
///     figcaption.bake(),
///     r#"<figcaption id="figure_caption"></figcaption>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let code = code!(r#"function greet() print("hi!") end"#);
///
/// let figcaption = figcaption!("Defining a function in Lua");
///
/// let content = bake![code, figcaption];
///
/// assert_eq!(
///     content,
///     r#"<code>function greet() print("hi!") end</code><figcaption>Defining a function in Lua</figcaption>"#
/// );
/// ```
#[macro_export]
macro_rules! figcaption {
    () => {
        $crate::html::HtmlFigcaption::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlFigcaption::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlFigcaption::new().content($crate::bake![$first $(, $rest)*])
    };
}
