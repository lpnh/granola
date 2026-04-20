use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait MeterTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl MeterTag for () {}

/// The HTML `<meter>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let meter: HtmlMeter = HtmlMeter::empty().id("html_meter");
///
/// assert_eq!(meter.bake(),
/// r#"<meter id="html_meter"></meter>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let meter: HtmlMeter = HtmlMeter::new("12%")
///     .value(12.)
///     .min(0.)
///     .max(100.)
///     .low(20.)
///     .high(60.)
///     .optimum(80.);
///
/// assert_eq!(meter.bake(),
/// r#"<meter value="12" min="0" max="100" low="20" high="60" optimum="80">12%</meter>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <meter
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</meter>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlMeter<M: MeterTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: MeterTag> HtmlMeter<M> {
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

    /// Low limit of high range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#high)
    pub fn high(mut self, value: f64) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("high", value.to_string());
        self
    }

    /// High limit of low range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#low)
    pub fn low(mut self, value: f64) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("low", value.to_string());
        self
    }

    /// Upper bound of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/max)
    pub fn max(mut self, value: f64) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("max", value.to_string());
        self
    }

    /// Lower bound of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/min)
    pub fn min(mut self, value: f64) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("min", value.to_string());
        self
    }

    /// Optimum value in gauge.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#optimum)
    pub fn optimum(mut self, value: f64) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("optimum", value.to_string());
        self
    }

    /// Current value of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#value)
    pub fn value(mut self, value: f64) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("value", value.to_string());
        self
    }
}
