use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait TextareaTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl TextareaTag for () {}

/// The HTML `<textarea>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let textarea: HtmlTextarea = HtmlTextarea::empty().id("textarea");
///
/// assert_eq!(textarea.bake(),
/// r#"<textarea id="textarea"></textarea>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let textarea: HtmlTextarea = HtmlTextarea::new("Carpe diem")
///     .name("reminder")
///     .readonly(true);
///
/// assert_eq!(textarea.bake(),
/// r#"<textarea name="reminder" readonly>Carpe diem</textarea>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <textarea
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</textarea>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlTextarea<M: TextareaTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: TextareaTag> HtmlTextarea<M> {
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

    /// Hint for form autofill feature.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    pub fn autocomplete(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("autocomplete", value);
        self
    }

    /// Maximum number of characters per line.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#cols)
    pub fn cols(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("cols", value.to_string());
        self
    }

    /// Name of form control to use for sending the element's directionality in form
    /// submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/dirname)
    pub fn dirname(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("dirname", value);
        self
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

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    pub fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("form", value);
        self
    }

    /// Maximum length of value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/maxlength)
    pub fn maxlength(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("maxlength", value.to_string());
        self
    }

    /// Minimum length of value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/minlength)
    pub fn minlength(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("minlength", value.to_string());
        self
    }

    /// Name of the element to use for form submission and in the `form.elements` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }

    /// User-visible label to be placed within the form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/placeholder)
    pub fn placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("placeholder", value);
        self
    }

    /// Whether to allow the value to be edited by the user.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/readonly)
    pub fn readonly(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("readonly");
        }
        self
    }

    /// Whether the control is required for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/required)
    pub fn required(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("required");
        }
        self
    }

    /// Number of lines to show.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#rows)
    pub fn rows(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("rows", value.to_string());
        self
    }

    /// How the value of the form control is to be wrapped for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/textarea#wrap)
    pub fn wrap(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("wrap", value);
        self
    }
}

/// Shorthand for `HtmlTextarea<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let textarea = textarea!().id("textarea");
///
/// assert_eq!(textarea.bake(),
/// r#"<textarea id="textarea"></textarea>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let textarea = textarea!("Carpe diem").name("reminder").readonly(true);
///
/// assert_eq!(textarea.bake(),
/// r#"<textarea name="reminder" readonly>Carpe diem</textarea>"#);
/// ```
#[macro_export]
macro_rules! textarea {
    () => {
        $crate::html::HtmlTextarea::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlTextarea::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlTextarea::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlTextarea::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlTextarea::<()>::new($crate::bake_inline![$($content),+])
    };
}
