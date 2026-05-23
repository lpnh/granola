use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// let meter: HtmlMeter = HtmlMeter::new(bake_newline!("12%"))
///     .value(12.)
///     .min(0.)
///     .max(100.)
///     .low(20.)
///     .high(60.)
///     .optimum(80.);
///
/// assert_eq!(meter.bake(),
/// r#"<meter value="12" min="0" max="100" low="20" high="60" optimum="80">
///   12%
/// </meter>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <meter
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</meter>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = MeterTag, content = Cow<'static, str>)]
pub struct HtmlMeter<R: MeterTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: MeterAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<meter>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- value | bake_attr("value") -}}
/// {{- min | bake_attr("min") -}}
/// {{- max | bake_attr("max") -}}
/// {{- low | bake_attr("low") -}}
/// {{- high | bake_attr("high") -}}
/// {{- optimum | bake_attr("optimum") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct MeterAttrs {
    pub value: Option<Cow<'static, str>>,
    pub min: Option<Cow<'static, str>>,
    pub max: Option<Cow<'static, str>>,
    pub low: Option<Cow<'static, str>>,
    pub high: Option<Cow<'static, str>>,
    pub optimum: Option<Cow<'static, str>>,
}

impl<R: MeterTag> HtmlMeter<R> {
    pub fn from_value(value: f64) -> Self {
        let mut global_attrs = GlobalAttrs::default();
        R::global_attrs_recipe(&mut global_attrs);

        let mut specific_attrs = MeterAttrs::default().value(value);
        R::specific_attrs_recipe(&mut specific_attrs);

        let mut global_aria_attrs = GlobalAriaAttrs::default();
        R::global_aria_attrs_recipe(&mut global_aria_attrs);

        let mut custom_data_attrs = CustomDataAttrs::default();
        R::custom_data_attrs_recipe(&mut custom_data_attrs);

        let mut event_handlers = EventHandlers::default();
        R::event_handlers_recipe(&mut event_handlers);

        Self {
            global_attrs,
            specific_attrs,
            global_aria_attrs,
            custom_data_attrs,
            event_handlers,
            ..Default::default()
        }
    }
}

pub trait HasMeterAttrs: Sized {
    fn meter_attrs_mut(&mut self) -> &mut MeterAttrs;

    /// Low limit of high range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#high)
    fn high(mut self, value: f64) -> Self {
        self.meter_attrs_mut().high = Some(value.to_string().into());
        self
    }

    /// High limit of low range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#low)
    fn low(mut self, value: f64) -> Self {
        self.meter_attrs_mut().low = Some(value.to_string().into());
        self
    }

    /// Upper bound of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/max)
    fn max(mut self, value: f64) -> Self {
        self.meter_attrs_mut().max = Some(value.to_string().into());
        self
    }

    /// Lower bound of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/min)
    fn min(mut self, value: f64) -> Self {
        self.meter_attrs_mut().min = Some(value.to_string().into());
        self
    }

    /// Optimum value in gauge.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#optimum)
    fn optimum(mut self, value: f64) -> Self {
        self.meter_attrs_mut().optimum = Some(value.to_string().into());
        self
    }

    /// Current value of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/meter#value.to_string().into())
    fn value(mut self, value: f64) -> Self {
        self.meter_attrs_mut().value = Some(value.to_string().into());
        self
    }
}

impl HasMeterAttrs for MeterAttrs {
    fn meter_attrs_mut(&mut self) -> &mut MeterAttrs {
        self
    }
}

impl HasMeterAttrs for &mut MeterAttrs {
    fn meter_attrs_mut(&mut self) -> &mut MeterAttrs {
        self
    }
}

impl<R: MeterTag> HasMeterAttrs for HtmlMeter<R> {
    fn meter_attrs_mut(&mut self) -> &mut MeterAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlMeter`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let meter = meter!().id("html_meter");
///
/// assert_eq!(meter.bake(),
/// r#"<meter id="html_meter"></meter>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let meter = meter!(@newline "12%")
///     .value(12.)
///     .min(0.)
///     .max(100.)
///     .low(20.)
///     .high(60.)
///     .optimum(80.);
///
/// assert_eq!(meter.bake(),
/// r#"<meter value="12" min="0" max="100" low="20" high="60" optimum="80">
///   12%
/// </meter>"#);
/// ```
#[macro_export]
macro_rules! meter {
    () => {
        $crate::html::HtmlMeter::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlMeter::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlMeter::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlMeter::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlMeter::<()>::new($crate::bake_inline![$($content),+])
    };
}
