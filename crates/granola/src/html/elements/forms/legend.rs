use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

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
/// let legend: HtmlLegend = HtmlLegend::empty().id("field_set_legend");
///
/// assert_eq!(legend.bake(),
/// r#"<legend id="field_set_legend"></legend>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let legend: HtmlLegend = HtmlLegend::new("Choose your favorite spoon");
///
/// assert_eq!(legend.bake(),
/// r#"<legend>Choose your favorite spoon</legend>"#);
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
/// >{{ content | kirei(2) }}</legend>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = LegendTag, content = Cow<'static, str>)]
pub struct HtmlLegend<M: LegendTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
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
/// assert_eq!(legend.bake(),
/// r#"<legend id="field_set_legend"></legend>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let legend = legend!("Choose your favorite spoon");
///
/// assert_eq!(legend.bake(),
/// r#"<legend>Choose your favorite spoon</legend>"#);
/// ```
#[macro_export]
macro_rules! legend {
    () => {
        $crate::html::HtmlLegend::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlLegend::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlLegend::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlLegend::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlLegend::<()>::new($crate::bake_inline![$($content),+])
    };
}
