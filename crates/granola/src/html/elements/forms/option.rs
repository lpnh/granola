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
///   {{- attrs -}}
///   {{- specific_attrs -}}
/// >{{ content | kirei(2) }}</option>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = OptionTag, content = Cow<'static, str>, specific = OptionAttrs)]
pub struct HtmlOption<M: OptionTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub attrs: Attrs,
    pub specific_attrs: OptionAttrs,
}

impl<M: OptionTag> HtmlOption<M> {
    pub fn from_value(value: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = OptionAttrs::default().value(value);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }
}

/// The HTML `<option>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/option#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- disabled | bake_bool_attr("disabled") -}}
/// {{- label | bake_attr("label") -}}
/// {{- selected | bake_bool_attr("selected") -}}
/// {{- value | bake_attr("value") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct OptionAttrs {
    pub disabled: bool,
    pub label: Option<Cow<'static, str>>,
    pub selected: bool,
    pub value: Option<Cow<'static, str>>,
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

impl<M: OptionTag> HasOptionAttrs for HtmlOption<M> {
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
/// {%- if !loop.last %}
/// {% endif -%}
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
