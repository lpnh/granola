use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// assert_eq!(form.bake(), r#"<form id="form"></form>"#);
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
/// assert_eq!(
///     form.bake(),
///     r#"<form method="get">
///   <label>
///     Wish:
///     <input name="cast-wish" />
///   </label>
///   <button>Cast</button>
/// </form>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <form
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</form>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = FormRecipe, content = Cow<'static, str>)]
pub struct HtmlForm<R: FormRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// search, none or presentation
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: FormAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<form>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- accept_charset | bake_attr("accept-charset") -}}
/// {{- action | bake_attr("action") -}}
/// {{- autocomplete | bake_attr("autocomplete") -}}
/// {{- enctype | bake_attr("enctype") -}}
/// {{- method | bake_attr("method") -}}
/// {{- name | bake_attr("name") -}}
/// {{- rel | bake_attr("rel") -}}
/// {{- target | bake_attr("target") -}}
/// {{- novalidate | bake_bool_attr("novalidate") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct FormAttrs {
    pub accept_charset: Option<Cow<'static, str>>,
    pub action: Option<Cow<'static, str>>,
    pub autocomplete: Option<Cow<'static, str>>,
    pub enctype: Option<Cow<'static, str>>,
    pub method: Option<Cow<'static, str>>,
    pub name: Option<Cow<'static, str>>,
    pub rel: Option<Cow<'static, str>>,
    pub target: Option<Cow<'static, str>>,
    pub novalidate: bool,
}

pub trait HasFormAttrs: Sized {
    fn form_attrs_mut(&mut self) -> &mut FormAttrs;

    /// Character encodings to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#accept-charset)
    fn accept_charset(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.form_attrs_mut().accept_charset = Some(value.into());
        self
    }

    /// URL to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#action)
    fn action(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.form_attrs_mut().action = Some(value.into());
        self
    }

    /// Default setting for autofill feature for controls in the form.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    fn autocomplete(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.form_attrs_mut().autocomplete = Some(value.into());
        self
    }

    /// Entry list encoding type to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#enctype)
    fn enctype(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.form_attrs_mut().enctype = Some(value.into());
        self
    }

    /// Variant to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#method)
    fn method(mut self, value: impl Into<FormMethod>) -> Self {
        self.form_attrs_mut().method = Some(value.into().into());
        self
    }

    /// Name of form to use in the `document.forms` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.form_attrs_mut().name = Some(value.into());
        self
    }

    /// Bypass form control validation for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#novalidate)
    fn novalidate(mut self, value: bool) -> Self {
        self.form_attrs_mut().novalidate = value;
        self
    }

    /// Controls the annotations and what kinds of links the form creates.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/rel)
    fn rel(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.form_attrs_mut().rel = Some(value.into());
        self
    }

    /// Navigable for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/form#target)
    fn target(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.form_attrs_mut().target = Some(value.into());
        self
    }
}

impl HasFormAttrs for FormAttrs {
    fn form_attrs_mut(&mut self) -> &mut FormAttrs {
        self
    }
}

impl HasFormAttrs for &mut FormAttrs {
    fn form_attrs_mut(&mut self) -> &mut FormAttrs {
        self
    }
}

impl<R: FormRecipe> HasFormAttrs for HtmlForm<R> {
    fn form_attrs_mut(&mut self) -> &mut FormAttrs {
        &mut self.specific_attrs
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

/// Shorthand for `HtmlForm`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let form = form!().id("form");
///
/// assert_eq!(form.bake(), r#"<form id="form"></form>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let input = input!("cast-wish");
/// let label = label!["Wish:", input];
/// let button = button!("Cast");
///
/// let form = form![label, button].method("get");
///
/// assert_eq!(
///     form.bake(),
///     r#"<form method="get">
///   <label>
///     Wish:
///     <input name="cast-wish" />
///   </label>
///   <button>Cast</button>
/// </form>"#
/// );
/// ```
#[macro_export]
macro_rules! form {
    () => {
        $crate::html::HtmlForm::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlForm::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlForm::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlForm::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlForm::<()>::new($crate::bake_inline![$($content),+])
    };


    (@cookbook $($r:ty),+) => {
        $crate::html::HtmlForm::<$crate::cookbook_type!($($r),+)>::from_cookbook()
    };
    (@cookbook $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlForm::<$crate::cookbook_type!($($r),+)>::new($content)
    };
    (@cookbook $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlForm::<$crate::cookbook_type!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@cookbook $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlForm::<$crate::cookbook_type!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@cookbook $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlForm::<$crate::cookbook_type!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
