use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait ProgressTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl ProgressTag for () {}

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
/// assert_eq!(progress.bake(),
/// r#"<progress id="progress_indicator"></progress>"#);
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
/// assert_eq!(progress.bake(),
/// r#"<progress id="experience" max="300" value="10">10/300</progress>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <progress
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2, 70) }}</progress>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlProgress<M: ProgressTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ProgressTag> HtmlProgress<M> {
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

    /// If there is no value attribute, the progress bar is indeterminate; this indicates that an
    /// activity is ongoing with no indication of how long it is expected to take.
    pub fn indeterminate() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    /// Upper bound of range.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/max)
    pub fn max(mut self, value: f64) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("max", value.to_string());
        self
    }

    /// Current value of the element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#value)
    pub fn value(mut self, value: f64) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("value", value.to_string());
        self
    }
}
