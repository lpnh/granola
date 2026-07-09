use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let time = HtmlTime::new().id("time_date");
///
/// assert_eq!(time.bake(), r#"<time id="time_date"></time>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let time = HtmlTime::new()
///     .content("Unix epoch")
///     .datetime("1970-01-01T00:00:00Z");
///
/// assert_eq!(
///     time.bake(),
///     r#"<time datetime="1970-01-01T00:00:00Z">Unix epoch</time>"#
/// );
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
/// >{{ content | kirei }}</time>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TimeRecipe, content = Bake)]
pub struct HtmlTime<R: TimeRecipe = ()> {
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
#[derive(Debug, Clone, Default, PartialEq, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TimeAttrs {
    pub datetime: Option<Bake>,
}

pub trait HasTimeAttrs: Sized {
    fn time_attrs_mut(&mut self) -> &mut TimeAttrs;

    /// Machine-readable datetime representation.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time#datetime)
    fn datetime(mut self, value: impl Into<Bake>) -> Self {
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

impl<R: TimeRecipe> HasTimeAttrs for HtmlTime<R> {
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
/// assert_eq!(time.bake(), r#"<time id="time_date"></time>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let time = time!("Unix epoch").datetime("1970-01-01T00:00:00Z");
///
/// assert_eq!(
///     time.bake(),
///     r#"<time datetime="1970-01-01T00:00:00Z">Unix epoch</time>"#
/// );
/// ```
#[macro_export]
macro_rules! time {
    () => {
        $crate::html::HtmlTime::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTime::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTime::new().content($crate::bake![$first $(, $rest)*])
    };

}
