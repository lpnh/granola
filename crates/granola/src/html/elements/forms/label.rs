use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</label>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = LabelTag, content = Cow<'static, str>, attrs = LabelAttrs)]
pub struct HtmlLabel<M: LabelTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: LabelAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<label>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/label#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- for_id | bake_attr("for") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct LabelAttrs {
    pub for_id: Option<Cow<'static, str>>,
}

impl HasLabelAttrs for LabelAttrs {
    fn label_attrs_mut(&mut self) -> &mut LabelAttrs {
        self
    }
}

impl HasLabelAttrs for &mut LabelAttrs {
    fn label_attrs_mut(&mut self) -> &mut LabelAttrs {
        self
    }
}

impl<M: LabelTag> HasLabelAttrs for HtmlLabel<M> {
    fn label_attrs_mut(&mut self) -> &mut LabelAttrs {
        &mut self.specific_attrs
    }
}

pub trait HasLabelAttrs: Sized {
    fn label_attrs_mut(&mut self) -> &mut LabelAttrs;

    /// Associate the label with form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/for)
    fn for_id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.label_attrs_mut().for_id = Some(value.into());
        self
    }
}

/// Shorthand for `HtmlLabel`.
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
