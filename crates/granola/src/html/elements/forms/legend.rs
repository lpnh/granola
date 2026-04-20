use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait LegendTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
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
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }
}
