use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<output>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/output)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let output = HtmlOutput::new().id("output");
///
/// assert_eq!(output.bake(), r#"<output id="output"></output>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let output = HtmlOutput::new()
///     .content("42")
///     .name("answer")
///     .for_id("ultimate-question");
///
/// assert_eq!(
///     output.bake(),
///     r#"<output name="answer" for="ultimate-question">42</output>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <output
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</output>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = OutputRecipe, content = Cow<'static, str>)]
pub struct HtmlOutput<R: OutputRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// any
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: OutputAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<output>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/output#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- name | bake_attr("name") -}}
/// {{- for_id | bake_attr("for") -}}
/// {{- form | bake_attr("form") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct OutputAttrs {
    pub name: Option<Cow<'static, str>>,
    pub for_id: Option<Cow<'static, str>>,
    pub form: Option<Cow<'static, str>>,
}

pub trait HasOutputAttrs: Sized {
    fn output_attrs_mut(&mut self) -> &mut OutputAttrs;

    /// Specifies controls from which the output was calculated.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/for)
    fn for_id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.output_attrs_mut().for_id = Some(value.into());
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.output_attrs_mut().form = Some(value.into());
        self
    }

    /// Name of the element to use for form submission and in the
    /// `form.elements` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/output#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.output_attrs_mut().name = Some(value.into());
        self
    }
}

impl HasOutputAttrs for OutputAttrs {
    fn output_attrs_mut(&mut self) -> &mut OutputAttrs {
        self
    }
}

impl HasOutputAttrs for &mut OutputAttrs {
    fn output_attrs_mut(&mut self) -> &mut OutputAttrs {
        self
    }
}

impl<R: OutputRecipe> HasOutputAttrs for HtmlOutput<R> {
    fn output_attrs_mut(&mut self) -> &mut OutputAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlOutput`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let output = output!().id("output");
///
/// assert_eq!(output.bake(), r#"<output id="output"></output>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let output = output!("42").name("answer").for_id("ultimate-question");
///
/// assert_eq!(
///     output.bake(),
///     r#"<output name="answer" for="ultimate-question">42</output>"#
/// );
/// ```
#[macro_export]
macro_rules! output {
    () => {
        $crate::html::HtmlOutput::new()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlOutput::new().content($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlOutput::new().content($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlOutput::new().content($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlOutput::new().content($crate::bake_inline![$($content),+])
    };
}
