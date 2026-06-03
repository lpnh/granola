use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<figure>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/figure)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let figure = HtmlFigure::new().id("figure_with_optional_caption");
///
/// assert_eq!(
///     figure.bake(),
///     r#"<figure id="figure_with_optional_caption"></figure>"#
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
/// let content = bake_block![code, figcaption];
///
/// let figure = HtmlFigure::new().content(content);
///
/// assert_eq!(
///     figure.bake(),
///     r#"<figure>
///   <code>function greet() print("hi!") end</code>
///   <figcaption>Defining a function in Lua</figcaption>
/// </figure>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <figure
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</figure>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = FigureRecipe, content = Cow<'static, str>)]
pub struct HtmlFigure<R: FigureRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// With figcaption descendant: no permitted roles
    /// Otherwise: any
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlFigure`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let figure = figure!().id("figure_with_optional_caption");
///
/// assert_eq!(
///     figure.bake(),
///     r#"<figure id="figure_with_optional_caption"></figure>"#
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
/// let figure = figure!(code, figcaption);
///
/// assert_eq!(
///     figure.bake(),
///     r#"<figure>
///   <code>function greet() print("hi!") end</code>
///   <figcaption>Defining a function in Lua</figcaption>
/// </figure>"#
/// );
/// ```
#[macro_export]
macro_rules! figure {
    () => {
        $crate::html::HtmlFigure::new()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlFigure::new().content($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlFigure::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlFigure::new().content($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlFigure::new().content($crate::bake_inline![$($content),+])
    };
}
