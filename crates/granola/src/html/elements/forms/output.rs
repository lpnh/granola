use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// any
pub trait OutputTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe(element: HtmlOutput<Self>) -> HtmlOutput<Self> {
        element
    }
}

impl OutputTag for () {}

/// The HTML `<output>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/output)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let output: HtmlOutput = HtmlOutput::empty().id("output");
///
/// assert_eq!(output.bake(),
/// r#"<output id="output"></output>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let output: HtmlOutput = HtmlOutput::new("42").name("answer").for_id("ultimate-question");
///
/// assert_eq!(output.bake(),
/// r#"<output name="answer" for="ultimate-question">42</output>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <output
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</output>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlOutput<M: OutputTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: OutputTag> HtmlOutput<M> {
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

    /// Specifies controls from which the output was calculated.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/for)
    pub fn for_id(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("for", value);
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    pub fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("form", value);
        self
    }

    /// Name of the element to use for form submission and in the `form.elements` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/output#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }
}

/// Shorthand for `HtmlOutput<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let output = output!().id("output");
///
/// assert_eq!(output.bake(),
/// r#"<output id="output"></output>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let output = output!("42").name("answer").for_id("ultimate-question");
///
/// assert_eq!(output.bake(),
/// r#"<output name="answer" for="ultimate-question">42</output>"#);
/// ```
#[macro_export]
macro_rules! output {
    () => {
        $crate::html::HtmlOutput::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlOutput::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlOutput::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlOutput::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlOutput::<()>::new($crate::bake_inline![$($content),+])
    };
}
