use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<legend>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/legend)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let legend = HtmlLegend::new().id("field_set_legend");
///
/// assert_eq!(legend.bake(), r#"<legend id="field_set_legend"></legend>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let legend = HtmlLegend::new().content("Choose your favorite spoon");
///
/// assert_eq!(
///     legend.bake(),
///     r#"<legend>Choose your favorite spoon</legend>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <legend
///   {{- global_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</legend>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = LegendRecipe, content = Bake)]
pub struct HtmlLegend<R: LegendRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// Shorthand for `HtmlLegend`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let legend = legend!().id("field_set_legend");
///
/// assert_eq!(legend.bake(), r#"<legend id="field_set_legend"></legend>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let legend = legend!("Choose your favorite spoon");
///
/// assert_eq!(
///     legend.bake(),
///     r#"<legend>Choose your favorite spoon</legend>"#
/// );
/// ```
#[macro_export]
macro_rules! legend {
    () => {
        $crate::html::HtmlLegend::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlLegend::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlLegend::new().content($crate::bake![$first $(, $rest)*])
    };

}
