use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait LegendTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlLegend<Self>) -> HtmlLegend<Self> {
        element
    }
}

impl LegendTag for () {}

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
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</legend>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlLegend<M: LegendTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: LegendTag> HtmlLegend<M> {
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

/// Shorthand for `HtmlLegend<()>`.
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
