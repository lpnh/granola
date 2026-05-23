use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<fieldset>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fieldset)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let fieldset: HtmlFieldset = HtmlFieldset::empty().id("field_set");
///
/// assert_eq!(fieldset.bake(),
/// r#"<fieldset id="field_set"></fieldset>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let legend: HtmlLegend = HtmlLegend::new("To be, or not to be?");
/// let input: HtmlInput = HtmlInput::from_type("checkbox")
///     .id("chbx")
///     .name("to-be")
///     .value("dunno");
/// let label: HtmlLabel = HtmlLabel::new("That is the question").for_id("chbx");
///
/// let fieldset: HtmlFieldset = HtmlFieldset::new(bake_block![legend, input, label]);
///
/// assert_eq!(fieldset.bake(),
/// r#"<fieldset>
///   <legend>To be, or not to be?</legend>
///   <input id="chbx" type="checkbox" name="to-be" value="dunno" />
///   <label for="chbx">That is the question</label>
/// </fieldset>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <fieldset
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</fieldset>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = FieldsetTag, content = Cow<'static, str>)]
pub struct HtmlFieldset<R: FieldsetTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// radiogroup, presentation, none
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: FieldsetAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<fieldset>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fieldset#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- name | bake_attr("name") -}}
/// {{- form | bake_attr("form") -}}
/// {{- disabled | bake_bool_attr("disabled") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct FieldsetAttrs {
    pub name: Option<Cow<'static, str>>,
    pub form: Option<Cow<'static, str>>,
    pub disabled: bool,
}

pub trait HasFieldsetAttrs: Sized {
    fn fieldset_attrs_mut(&mut self) -> &mut FieldsetAttrs;

    /// Whether the descendant form controls, except any inside legend, are disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    fn disabled(mut self, value: bool) -> Self {
        self.fieldset_attrs_mut().disabled = value;
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.fieldset_attrs_mut().form = Some(value.into());
        self
    }

    /// Name of the element to use for form submission and in the form.elements API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fieldset#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.fieldset_attrs_mut().name = Some(value.into());
        self
    }
}

impl HasFieldsetAttrs for FieldsetAttrs {
    fn fieldset_attrs_mut(&mut self) -> &mut FieldsetAttrs {
        self
    }
}

impl HasFieldsetAttrs for &mut FieldsetAttrs {
    fn fieldset_attrs_mut(&mut self) -> &mut FieldsetAttrs {
        self
    }
}

impl<R: FieldsetTag> HasFieldsetAttrs for HtmlFieldset<R> {
    fn fieldset_attrs_mut(&mut self) -> &mut FieldsetAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlFieldset`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let fieldset = fieldset!().id("field_set");
///
/// assert_eq!(fieldset.bake(),
/// r#"<fieldset id="field_set"></fieldset>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let legend = legend!("To be, or not to be?");
/// let input = input!(@from_type "checkbox")
///     .id("chbx")
///     .name("to-be")
///     .value("dunno");
/// let label = label!("That is the question").for_id("chbx");
///
/// let fieldset = fieldset![legend, input, label];
///
/// assert_eq!(fieldset.bake(),
/// r#"<fieldset>
///   <legend>To be, or not to be?</legend>
///   <input id="chbx" type="checkbox" name="to-be" value="dunno" />
///   <label for="chbx">That is the question</label>
/// </fieldset>"#);
/// ```
#[macro_export]
macro_rules! fieldset {
    () => {
        $crate::html::HtmlFieldset::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlFieldset::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlFieldset::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlFieldset::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlFieldset::<()>::new($crate::bake_inline![$($content),+])
    };
}
