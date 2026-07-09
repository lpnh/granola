use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

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
/// let label = HtmlLabel::new().id("label");
///
/// assert_eq!(label.bake(), r#"<label id="label"></label>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let input = HtmlInput::new()
///     .input_type(InputType::Checkbox)
///     .name("reality-check")
///     .disabled(true);
///
/// let label = HtmlLabel::new().fold_in("We're so back").fold_in(input);
///
/// assert_eq!(
///     label.bake(),
///     r#"<label>We're so back<input type="checkbox" name="reality-check" disabled /></label>"#
/// );
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
/// >{{ content | kirei }}</label>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = LabelRecipe, content = Bake)]
pub struct HtmlLabel<R: LabelRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: LabelAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl<R: LabelRecipe<Content = Bake>> HtmlLabel<R> {
    pub fn fold_in(mut self, content: impl FastWritable) -> Self {
        self.content.fold_in(content);
        self
    }
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
#[derive(Debug, Clone, Default, PartialEq, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct LabelAttrs {
    pub for_id: Option<Bake>,
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

impl<R: LabelRecipe> HasLabelAttrs for HtmlLabel<R> {
    fn label_attrs_mut(&mut self) -> &mut LabelAttrs {
        &mut self.specific_attrs
    }
}

pub trait HasLabelAttrs: Sized {
    fn label_attrs_mut(&mut self) -> &mut LabelAttrs;

    /// Associate the label with form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/for)
    fn for_id(mut self, value: impl Into<Bake>) -> Self {
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
/// assert_eq!(label.bake(), r#"<label id="label"></label>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let input = input!()
///     .input_type("checkbox")
///     .name("reality-check")
///     .disabled(true);
///
/// let label = label!["We're so back", input];
///
/// assert_eq!(
///     label.bake(),
///     r#"<label>We're so back<input type="checkbox" name="reality-check" disabled /></label>"#
/// );
/// ```
#[macro_export]
macro_rules! label {
    () => {
        $crate::html::HtmlLabel::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlLabel::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlLabel::new().content($crate::bake![$first $(, $rest)*])
    };

}
