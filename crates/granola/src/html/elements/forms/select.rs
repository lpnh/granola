use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<select>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/select)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let select = HtmlSelect::new().id("html_select");
///
/// assert_eq!(select.bake(), r#"<select id="html_select"></select>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let opt_1 = HtmlOption::new().content("Salmon").value("salmon");
/// let opt_2 = HtmlOption::new().content("Turbot").value("turbot");
///
/// let select = HtmlSelect::new()
///     .content(bake_block![opt_1, opt_2])
///     .name("fishes");
///
/// assert_eq!(
///     select.bake(),
///     r#"<select name="fishes">
///   <option value="salmon">Salmon</option>
///   <option value="turbot">Turbot</option>
/// </select>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <select
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</select>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = SelectRecipe, content = Cow<'static, str>)]
pub struct HtmlSelect<R: SelectRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// menu (with no multiple attribute and no size attribute greater than 1)
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SelectAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<select>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/select#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- autocomplete | bake_attr("autocomplete") -}}
/// {{- form | bake_attr("form") -}}
/// {{- name | bake_attr("name") -}}
/// {{- size | bake_attr("size") -}}
/// {{- disabled | bake_bool_attr("disabled") -}}
/// {{- multiple | bake_bool_attr("multiple") -}}
/// {{- required | bake_bool_attr("required") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct SelectAttrs {
    autocomplete: Option<Cow<'static, str>>,
    form: Option<Cow<'static, str>>,
    name: Option<Cow<'static, str>>,
    size: Option<u32>,
    disabled: bool,
    multiple: bool,
    required: bool,
}

pub trait HasSelectAttrs: Sized {
    fn select_attrs_mut(&mut self) -> &mut SelectAttrs;

    /// Hint for form autofill feature.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    fn autocomplete(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.select_attrs_mut().autocomplete = Some(value.into());
        self
    }

    /// Whether the form control is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    fn disabled(mut self, value: bool) -> Self {
        self.select_attrs_mut().disabled = value;
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.select_attrs_mut().form = Some(value.into());
        self
    }

    /// Whether to allow multiple values.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/multiple)
    fn multiple(mut self, value: bool) -> Self {
        self.select_attrs_mut().multiple = value;
        self
    }

    /// Name of the element to use for form submission and in the
    /// `form.elements` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/select#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.select_attrs_mut().name = Some(value.into());
        self
    }

    /// Whether the control is required for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/required)
    fn required(mut self, value: bool) -> Self {
        self.select_attrs_mut().required = value;
        self
    }

    /// Size of the control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/size)
    fn size(mut self, value: u32) -> Self {
        self.select_attrs_mut().size = Some(value);
        self
    }
}

impl HasSelectAttrs for SelectAttrs {
    fn select_attrs_mut(&mut self) -> &mut SelectAttrs {
        self
    }
}

impl HasSelectAttrs for &mut SelectAttrs {
    fn select_attrs_mut(&mut self) -> &mut SelectAttrs {
        self
    }
}

impl<R: SelectRecipe> HasSelectAttrs for HtmlSelect<R> {
    fn select_attrs_mut(&mut self) -> &mut SelectAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlSelect`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let select = select!().id("html_select");
///
/// assert_eq!(select.bake(), r#"<select id="html_select"></select>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let opt_1 = option!("Salmon").value("salmon");
/// let opt_2 = option!("Turbot").value("turbot");
///
/// let select: HtmlSelect = select![opt_1, opt_2].name("fishes");
///
/// assert_eq!(
///     select.bake(),
///     r#"<select name="fishes">
///   <option value="salmon">Salmon</option>
///   <option value="turbot">Turbot</option>
/// </select>"#
/// );
/// ```
#[macro_export]
macro_rules! select {
    () => {
        $crate::html::HtmlSelect::new()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSelect::new().content($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSelect::new().content($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSelect::new().content($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSelect::new().content($crate::bake_inline![$($content),+])
    };
}
