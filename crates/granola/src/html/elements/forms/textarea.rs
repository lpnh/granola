use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<textarea>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let textarea = HtmlTextarea::new().id("textarea");
///
/// assert_eq!(textarea.bake(), r#"<textarea id="textarea"></textarea>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let textarea = HtmlTextarea::new()
///     .content("Carpe diem")
///     .name("reminder")
///     .readonly(true);
///
/// assert_eq!(
///     textarea.bake(),
///     r#"<textarea name="reminder" readonly>Carpe diem</textarea>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <textarea
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</textarea>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = TextareaRecipe, content = Cow<'static, str>)]
pub struct HtmlTextarea<R: TextareaRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: TextareaAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<textarea>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- autocomplete | bake_attr("autocomplete") -}}
/// {{- cols | bake_attr("cols") -}}
/// {{- dirname | bake_attr("dirname") -}}
/// {{- form | bake_attr("form") -}}
/// {{- maxlength | bake_attr("maxlength") -}}
/// {{- minlength | bake_attr("minlength") -}}
/// {{- name | bake_attr("name") -}}
/// {{- placeholder | bake_attr("placeholder") -}}
/// {{- rows | bake_attr("rows") -}}
/// {{- wrap | bake_attr("wrap") -}}
/// {{- disabled | bake_bool_attr("disabled") -}}
/// {{- readonly | bake_bool_attr("readonly") -}}
/// {{- required | bake_bool_attr("required") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct TextareaAttrs {
    pub autocomplete: Option<Cow<'static, str>>,
    pub cols: Option<u32>,
    pub dirname: Option<Cow<'static, str>>,
    pub form: Option<Cow<'static, str>>,
    pub maxlength: Option<u32>,
    pub minlength: Option<u32>,
    pub name: Option<Cow<'static, str>>,
    pub placeholder: Option<Cow<'static, str>>,
    pub rows: Option<u32>,
    pub wrap: Option<Cow<'static, str>>,
    pub disabled: bool,
    pub readonly: bool,
    pub required: bool,
}

pub trait HasTextareaAttrs: Sized {
    fn textarea_attrs_mut(&mut self) -> &mut TextareaAttrs;

    /// Hint for form autofill feature.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    fn autocomplete(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.textarea_attrs_mut().autocomplete = Some(value.into());
        self
    }

    /// Maximum number of characters per line.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#cols)
    fn cols(mut self, value: u32) -> Self {
        self.textarea_attrs_mut().cols = Some(value);
        self
    }

    /// Name of form control to use for sending the element's directionality in
    /// form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/dirname)
    fn dirname(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.textarea_attrs_mut().dirname = Some(value.into());
        self
    }

    /// Whether the form control is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    fn disabled(mut self, value: bool) -> Self {
        self.textarea_attrs_mut().disabled = value;
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.textarea_attrs_mut().form = Some(value.into());
        self
    }

    /// Maximum length of value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/maxlength)
    fn maxlength(mut self, value: u32) -> Self {
        self.textarea_attrs_mut().maxlength = Some(value);
        self
    }

    /// Minimum length of value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/minlength)
    fn minlength(mut self, value: u32) -> Self {
        self.textarea_attrs_mut().minlength = Some(value);
        self
    }

    /// Name of the element to use for form submission and in the
    /// `form.elements` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.textarea_attrs_mut().name = Some(value.into());
        self
    }

    /// User-visible label to be placed within the form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/placeholder)
    fn placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.textarea_attrs_mut().placeholder = Some(value.into());
        self
    }

    /// Whether to allow the value to be edited by the user.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/readonly)
    fn readonly(mut self, value: bool) -> Self {
        self.textarea_attrs_mut().readonly = value;
        self
    }

    /// Whether the control is required for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/required)
    fn required(mut self, value: bool) -> Self {
        self.textarea_attrs_mut().required = value;
        self
    }

    /// Number of lines to show.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#rows)
    fn rows(mut self, value: u32) -> Self {
        self.textarea_attrs_mut().rows = Some(value);
        self
    }

    /// How the value of the form control is to be wrapped for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#wrap)
    fn wrap(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.textarea_attrs_mut().wrap = Some(value.into());
        self
    }
}

impl HasTextareaAttrs for TextareaAttrs {
    fn textarea_attrs_mut(&mut self) -> &mut TextareaAttrs {
        self
    }
}

impl HasTextareaAttrs for &mut TextareaAttrs {
    fn textarea_attrs_mut(&mut self) -> &mut TextareaAttrs {
        self
    }
}

impl<R: TextareaRecipe> HasTextareaAttrs for HtmlTextarea<R> {
    fn textarea_attrs_mut(&mut self) -> &mut TextareaAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlTextarea`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let textarea = textarea!().id("textarea");
///
/// assert_eq!(textarea.bake(), r#"<textarea id="textarea"></textarea>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let textarea = textarea!("Carpe diem").name("reminder").readonly(true);
///
/// assert_eq!(
///     textarea.bake(),
///     r#"<textarea name="reminder" readonly>Carpe diem</textarea>"#
/// );
/// ```
#[macro_export]
macro_rules! textarea {
    () => {
        $crate::html::HtmlTextarea::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlTextarea::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTextarea::new().content($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content:expr $(,)?) => {
        $crate::html::HtmlTextarea::new().content($crate::bake_newline!($content))
    };
    (@inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlTextarea::new().content($crate::bake_inline![$($content),+])
    };
    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlTextarea::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlTextarea::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlTextarea::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlTextarea::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlTextarea::<$crate::cookbook_type!($($r),+)>::from_cookbook().content($crate::bake_inline![$($content),+])
    };
}
