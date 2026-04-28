use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait LabelTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl LabelTag for () {}

/// The HTML `<label>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/label)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let label: HtmlLabel = HtmlLabel::empty().id("label");
///
/// assert_eq!(label.bake(),
/// r#"<label id="label"></label>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let input: HtmlInput = HtmlInput::from_type("checkbox")
///     .name("reality-check")
///     .disabled(true);
///
/// let label: HtmlLabel = HtmlLabel::new(bake_block!["We're so back", input]);
///
/// assert_eq!(label.bake(),
/// r#"<label>
///   We're so back
///   <input type="checkbox" name="reality-check" disabled />
/// </label>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <label
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</label>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlLabel<M: LabelTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: LabelTag> HtmlLabel<M> {
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

    pub fn validate(self) -> Self {
        self.class("validator")
    }

    /// Associate the label with form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/for)
    pub fn for_id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("for", value);
        self
    }
}

/// Shorthand for `HtmlLabel<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let label = label!().id("label");
///
/// assert_eq!(label.bake(),
/// r#"<label id="label"></label>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let input = input!(@from_type "checkbox").name("reality-check").disabled(true);
///
/// let label = label!["We're so back", input];
///
/// assert_eq!(label.bake(),
/// r#"<label>
///   We're so back
///   <input type="checkbox" name="reality-check" disabled />
/// </label>"#);
/// ```
#[macro_export]
macro_rules! label {
    () => {
        $crate::html::HtmlLabel::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlLabel::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlLabel::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlLabel::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlLabel::<()>::new($crate::bake_inline![$($content),+])
    };
}
