use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<option>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let option = HtmlOption::new().id("html_option");
///
/// assert_eq!(option.bake(), r#"<option id="html_option"></option>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let option = HtmlOption::new().content("Chocolate").value("chocolate");
///
/// assert_eq!(
///     option.bake(),
///     r#"<option value="chocolate">Chocolate</option>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <option
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei }}</option>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = OptionRecipe, content = Cow<'static, str>)]
pub struct HtmlOption<R: OptionRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: OptionAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlOption {
    pub fn from_value(value: impl Into<Cow<'static, str>>) -> Self {
        Self::new().value(value)
    }
}

/// The HTML `<option>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- label | bake_attr("label") -}}
/// {{- value | bake_attr("value") -}}
/// {{- disabled | bake_bool_attr("disabled") -}}
/// {{- selected | bake_bool_attr("selected") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct OptionAttrs {
    pub label: Option<Cow<'static, str>>,
    pub value: Option<Cow<'static, str>>,
    pub disabled: bool,
    pub selected: bool,
}

pub trait HasOptionAttrs: Sized {
    fn option_attrs_mut(&mut self) -> &mut OptionAttrs;

    /// Whether the form control is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    fn disabled(mut self, value: bool) -> Self {
        self.option_attrs_mut().disabled = value;
        self
    }

    /// User-visible label.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#label)
    fn label(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.option_attrs_mut().label = Some(value.into());
        self
    }

    /// Whether the option is selected by default.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#selected)
    fn selected(mut self, value: bool) -> Self {
        self.option_attrs_mut().selected = value;
        self
    }

    /// Value to be used for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#value)
    fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.option_attrs_mut().value = Some(value.into());
        self
    }
}

impl HasOptionAttrs for OptionAttrs {
    fn option_attrs_mut(&mut self) -> &mut OptionAttrs {
        self
    }
}

impl HasOptionAttrs for &mut OptionAttrs {
    fn option_attrs_mut(&mut self) -> &mut OptionAttrs {
        self
    }
}

impl<R: OptionRecipe> HasOptionAttrs for HtmlOption<R> {
    fn option_attrs_mut(&mut self) -> &mut OptionAttrs {
        &mut self.specific_attrs
    }
}

/// A collection of HTML `<option>` items.
///
/// The content of [`HtmlOptgroup`].
///
/// ```askama
/// {%- for option in items -%}
/// {{- option -}}
/// {%- endfor -%}
/// ```
#[derive(Default, Debug, Clone, Template, Granola)]
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

/// Shorthand for `HtmlOption`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let option = option!().id("html_option");
///
/// assert_eq!(option.bake(), r#"<option id="html_option"></option>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let option = option!("Chocolate").value("chocolate");
///
/// assert_eq!(
///     option.bake(),
///     r#"<option value="chocolate">Chocolate</option>"#
/// );
/// ```
#[macro_export]
macro_rules! option {
    () => {
        $crate::html::HtmlOption::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlOption::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlOption::new().content($crate::bake![$first $(, $rest)*])
    };

    (@cookbook $r:ty $(,)?) => {
        $crate::html::HtmlOption::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; $content:expr $(,)?) => {
        $crate::html::HtmlOption::<$r>::from_cookbook().content($content)
    };
    (@cookbook $r:ty ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlOption::<$r>::from_cookbook().content($crate::bake![$first $(, $rest)*])
    };
}
