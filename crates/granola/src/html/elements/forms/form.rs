use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait FormTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    const METHOD: Option<FormMethod> = None;
    /// Permitted ARIA roles: search, none or presentation
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl FormTag for () {}

/// The HTML `<form>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let form: HtmlForm = HtmlForm::empty().id("form");
///
/// assert_eq!(form.bake(),
/// r#"<form id="form"></form>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let input: HtmlInput = HtmlInput::new("cast-wish");
/// let label: HtmlLabel = HtmlLabel::new(bake_block!["Wish:", input]);
/// let button: HtmlButton = HtmlButton::new("Cast");
///
/// let form: HtmlForm = HtmlForm::new(bake_block![label, button]).method("get");
///
/// assert_eq!(form.bake(),
/// r#"<form method="get">
///   <label>
///     Wish:
///     <input name="cast-wish" />
///   </label>
///   <button>Cast</button>
/// </form>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <form
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</form>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlForm<M: FormTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: FormTag> HtmlForm<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(method) = M::METHOD {
            s = s.method(method);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(method) = M::METHOD {
            s = s.method(method);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    /// Character encodings to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#accept-charset)
    pub fn accept_charset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("accept-charset", value);
        self
    }

    /// URL to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#action)
    pub fn action(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("action", value);
        self
    }

    /// Default setting for autofill feature for controls in the form.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    pub fn autocomplete(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("autocomplete", value.into());
        self
    }

    /// Entry list encoding type to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#enctype)
    pub fn enctype(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("enctype", value.into());
        self
    }

    /// Entry list encoding type to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#enctype)
    pub fn method(mut self, value: impl Into<FormMethod>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("method", value.into());
        self
    }

    /// Name of form to use in the `document.forms` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value.into());
        self
    }

    /// Bypass form control validation for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#novalidate)
    pub fn novalidate(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("novalidate");
        }
        self
    }

    /// Controls the annotations and what kinds of links the form creates.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
    pub fn rel(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("rel", value.into());
        self
    }

    /// Navigable for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#target)
    pub fn target(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("target", value.into());
        self
    }
}

#[derive(strum::Display, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum FormMethod {
    Post,
    Get, // default
    Dialog,
}

impl<T: AsRef<str>> From<T> for FormMethod {
    fn from(s: T) -> Self {
        let form_method = s.as_ref().trim().to_lowercase();
        match form_method.as_str() {
            "post" => Self::Post,
            "get" => Self::Get,
            "dialog" => Self::Dialog,
            _ => Self::Get,
        }
    }
}

impl From<FormMethod> for Cow<'static, str> {
    fn from(m: FormMethod) -> Self {
        <&'static str>::from(m).into()
    }
}
