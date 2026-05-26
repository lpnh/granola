use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<progress>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/progress)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let progress: HtmlProgress = HtmlProgress::empty().id("progress_indicator");
///
/// assert_eq!(
///     progress.bake(),
///     r#"<progress id="progress_indicator"></progress>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let progress: HtmlProgress = HtmlProgress::new("10/300")
///     .id("experience")
///     .max(300.)
///     .value(10.);
///
/// assert_eq!(
///     progress.bake(),
///     r#"<progress id="experience" max="300" value="10">10/300</progress>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <progress
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</progress>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ProgressTag, content = Cow<'static, str>)]
pub struct HtmlProgress<R: ProgressTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: ProgressAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: ProgressTag> HtmlProgress<R> {
    /// If there is no value attribute, the progress bar is indeterminate; this
    /// indicates that an activity is ongoing with no indication of how long
    /// it is expected to take.
    pub fn indeterminate() -> Self {
        Self::empty()
    }
}

/// The HTML `<progress>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/progress#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- max | bake_attr("max") -}}
/// {{- value | bake_attr("value") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ProgressAttrs {
    pub max: Option<Cow<'static, str>>,
    pub value: Option<Cow<'static, str>>,
}

pub trait HasProgressAttrs: Sized {
    fn progress_attrs_mut(&mut self) -> &mut ProgressAttrs;

    /// Upper bound of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/max)
    fn max(mut self, value: f64) -> Self {
        self.progress_attrs_mut().max = Some(value.to_string().into());
        self
    }

    /// Current value of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/progress#value)
    fn value(mut self, value: f64) -> Self {
        self.progress_attrs_mut().value = Some(value.to_string().into());
        self
    }
}

impl HasProgressAttrs for ProgressAttrs {
    fn progress_attrs_mut(&mut self) -> &mut ProgressAttrs {
        self
    }
}

impl HasProgressAttrs for &mut ProgressAttrs {
    fn progress_attrs_mut(&mut self) -> &mut ProgressAttrs {
        self
    }
}

impl<R: ProgressTag> HasProgressAttrs for HtmlProgress<R> {
    fn progress_attrs_mut(&mut self) -> &mut ProgressAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlProgress`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let progress = progress!().id("progress_indicator");
///
/// assert_eq!(
///     progress.bake(),
///     r#"<progress id="progress_indicator"></progress>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let progress = progress!("10/300").id("experience").max(300.).value(10.);
///
/// assert_eq!(
///     progress.bake(),
///     r#"<progress id="experience" max="300" value="10">10/300</progress>"#
/// );
/// ```
#[macro_export]
macro_rules! progress {
    () => {
        $crate::html::HtmlProgress::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlProgress::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlProgress::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlProgress::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlProgress::<()>::new($crate::bake_inline![$($content),+])
    };
}
