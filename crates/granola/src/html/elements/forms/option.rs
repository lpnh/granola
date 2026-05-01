use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait OptionTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlOption<Self>) -> HtmlOption<Self> {
        element
    }
}

impl OptionTag for () {}

/// The HTML `<option>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let option: HtmlOption = HtmlOption::empty().id("html_option");
///
/// assert_eq!(option.bake(),
/// r#"<option id="html_option"></option>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let option: HtmlOption = HtmlOption::new("Chocolate").value("chocolate");
///
/// assert_eq!(option.bake(),
/// r#"<option value="chocolate">Chocolate</option>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <option
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</option>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlOption<M: OptionTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: OptionTag> HtmlOption<M> {
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

    pub fn from_value(value: impl Into<Cow<'static, str>>) -> Self {
        let element = Self::default().value(value);

        M::recipe(element)
    }

    /// Whether the form control is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    pub fn disabled(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("disabled");
        }
        self
    }

    /// User-visible label.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#label)
    pub fn label(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("label", value);
        self
    }

    /// Whether the option is selected by default.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#selected)
    pub fn selected(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("selected");
        }
        self
    }

    /// Value to be used for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#value)
    pub fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("value", value);
        self
    }
}

/// A collection of HTML `<option>` items.
///
/// The content of [`HtmlOptgroup`].
///
/// ```askama
/// {%- for option in items -%}
/// {{- option -}}
/// {%- if !loop.last %}
/// {% endif -%}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, PartialEq, Template, Granola)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct Options {
    items: Vec<HtmlOption>,
}

impl<I: IntoIterator<Item = HtmlOption>> From<I> for Options {
    fn from(items: I) -> Self {
        Self {
            items: items.into_iter().collect(),
        }
    }
}

/// Shorthand for `HtmlOption<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let option = option!().id("html_option");
///
/// assert_eq!(option.bake(),
/// r#"<option id="html_option"></option>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let option = option!("Chocolate").value("chocolate");
///
/// assert_eq!(option.bake(),
/// r#"<option value="chocolate">Chocolate</option>"#);
/// ```
#[macro_export]
macro_rules! option {
    () => {
        $crate::html::HtmlOption::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlOption::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlOption::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlOption::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlOption::<()>::new($crate::bake_inline![$($content),+])
    };
    (@from_value $value: expr $(,)?) => {
        $crate::html::HtmlOption::<()>::from_value($value)
    };
}
