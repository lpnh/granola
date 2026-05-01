use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// radiogroup, presentation, none
pub trait FieldsetTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlFieldset<Self>) -> HtmlFieldset<Self> {
        element
    }
}

impl FieldsetTag for () {}

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
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</fieldset>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlFieldset<M: FieldsetTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: FieldsetTag> HtmlFieldset<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let element = Self {
            content: content.into(),
            ..Default::default()
        };

        M::recipe(element)
    }

    pub fn empty() -> Self {
        let element = Self::default();

        M::recipe(element)
    }

    /// Whether the descendant form controls, except any inside legend, are disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("disabled");
        }
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    pub fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("form", value);
        self
    }

    /// Name of the element to use for form submission and in the form.elements API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/fieldset#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }
}

/// Shorthand for `HtmlFieldset<()>`.
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
