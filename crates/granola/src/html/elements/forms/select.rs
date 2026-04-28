use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

pub trait SelectTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: menu (with no multiple attribute and no size attribute greater than 1)
    const ROLE: Option<&'static str> = None;
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;
}

impl SelectTag for () {}

/// The HTML `<select>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/select)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let select: HtmlSelect = HtmlSelect::empty().id("html_select");
///
/// assert_eq!(select.bake(),
/// r#"<select id="html_select"></select>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let opt_1: HtmlOption = HtmlOption::new("Salmon").value("salmon");
/// let opt_2: HtmlOption = HtmlOption::new("Turbot").value("turbot");
///
/// let select: HtmlSelect = HtmlSelect::new(bake_block![opt_1, opt_2]).name("fishes");
///
/// assert_eq!(select.bake(),
/// r#"<select name="fishes">
///   <option value="salmon">Salmon</option>
///   <option value="turbot">Turbot</option>
/// </select>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <select
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</select>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlSelect<M: SelectTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: SelectTag> HtmlSelect<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
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
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        s
    }

    /// Hint for form autofill feature.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    pub fn autocomplete(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("autocomplete", value);
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

    /// Whether to allow multiple values.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/multiple)
    pub fn multiple(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("multiple");
        }
        self
    }

    /// Name of the element to use for form submission and in the `form.elements` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/select#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
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

    /// Size of the control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/size)
    pub fn size(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("size", value.to_string());
        self
    }
}

/// Shorthand for `HtmlSelect<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let select = select!().id("html_select");
///
/// assert_eq!(select.bake(),
/// r#"<select id="html_select"></select>"#);
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
/// assert_eq!(select.bake(),
/// r#"<select name="fishes">
///   <option value="salmon">Salmon</option>
///   <option value="turbot">Turbot</option>
/// </select>"#);
/// ```
#[macro_export]
macro_rules! select {
    () => {
        $crate::html::HtmlSelect::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlSelect::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlSelect::<()>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlSelect::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlSelect::<()>::new($crate::bake_inline![$($content),+])
    };
}
