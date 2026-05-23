use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<time>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/time)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let time: HtmlTime = HtmlTime::empty().id("time_date");
///
/// assert_eq!(time.bake(),
/// r#"<time id="time_date"></time>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let time: HtmlTime = HtmlTime::new("Unix epoch").datetime("1970-01-01T00:00:00Z");
///
/// assert_eq!(time.bake(),
/// r#"<time datetime="1970-01-01T00:00:00Z">Unix epoch</time>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <time
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</time>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TimeTag, content = Cow<'static, str>)]
pub struct HtmlTime<R: TimeTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: TimeAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<time>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/time#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- datetime | bake_attr("datetime") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TimeAttrs {
    pub datetime: Option<Cow<'static, str>>,
}

pub trait HasTimeAttrs: Sized {
    fn time_attrs_mut(&mut self) -> &mut TimeAttrs;

    /// Machine-readable datetime representation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time#datetime)
    fn datetime(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.time_attrs_mut().datetime = Some(value.into());
        self
    }
}

impl HasTimeAttrs for TimeAttrs {
    fn time_attrs_mut(&mut self) -> &mut TimeAttrs {
        self
    }
}

impl HasTimeAttrs for &mut TimeAttrs {
    fn time_attrs_mut(&mut self) -> &mut TimeAttrs {
        self
    }
}

impl<R: TimeTag> HasTimeAttrs for HtmlTime<R> {
    fn time_attrs_mut(&mut self) -> &mut TimeAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlTime`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let time = time!().id("time_date");
///
/// assert_eq!(time.bake(),
/// r#"<time id="time_date"></time>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let time = time!("Unix epoch").datetime("1970-01-01T00:00:00Z");
///
/// assert_eq!(time.bake(),
/// r#"<time datetime="1970-01-01T00:00:00Z">Unix epoch</time>"#);
/// ```
#[macro_export]
macro_rules! time {
    () => {
        $crate::html::HtmlTime::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTime::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTime::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTime::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTime::<()>::new($crate::bake_inline![$($content),+])
    };
}
