use askama::{FastWritable, Template};
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// # Permitted ARIA roles
///
/// checkbox, combobox, link, menuitem, menuitemcheckbox, menuitemradio, option,
/// radio, switch, tab
pub trait ButtonTag: Default + Clone + Debug + 'static {
    type Content: FastWritable + Default + Clone + Debug = Cow<'static, str>;

    fn recipe<R: ButtonTag>(element: HtmlButton<R>) -> HtmlButton<R> {
        element
    }
}

impl ButtonTag for () {}

impl<A: ButtonTag, B: ButtonTag> ButtonTag for (A, B) {
    fn recipe<R: ButtonTag>(element: HtmlButton<R>) -> HtmlButton<R> {
        B::recipe(A::recipe(element))
    }
}

/// The HTML `<button>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let button: HtmlButton = HtmlButton::empty().id("button");
///
/// assert_eq!(button.bake(),
/// r#"<button id="button"></button>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let button: HtmlButton = HtmlButton::new(bake_newline!("Add to favorites"))
///     .button_type("button")
///     .name("favorite");
///
/// assert_eq!(button.bake(),
/// r#"<button type="button" name="favorite">
///   Add to favorites
/// </button>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <button
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</button>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlButton<M: ButtonTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: ButtonTag> HtmlButton<M> {
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

    /// Indicates to the targeted element which action to take.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#command)
    pub fn command(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("command", value);
        self
    }

    /// Targets another element to be invoked.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#commandfor)
    pub fn commandfor(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("commandfor", value);
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

    /// URL to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formaction)
    pub fn formaction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("formaction", value);
        self
    }

    /// Entry list encoding type to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formenctype)
    pub fn formenctype(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("formenctype", value);
        self
    }

    /// Variant to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formmethod)
    pub fn formmethod(mut self, value: impl Into<FormMethod>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("formmethod", value.into());
        self
    }

    /// Bypass form control validation for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formnovalidate)
    pub fn formnovalidate(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("formnovalidate");
        }
        self
    }

    /// Navigable for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formtarget)
    pub fn formtarget(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("formtarget", value);
        self
    }

    // NOTE: Include `interestfor` in the future.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#interestfor)

    /// Name of the element to use for form submission and in the `form.elements` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }

    /// Targets a popover element to toggle, show, or hide.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#popovertarget)
    pub fn popovertarget(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("popovertarget", value);
        self
    }

    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#popovertargetaction)
    pub fn popovertargetaction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("popovertargetaction", value);
        self
    }

    /// Type of button.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#type)
    pub fn button_type(mut self, value: impl Into<ButtonType>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value.into());
        self
    }

    /// Value to be used for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#value)
    pub fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("value", value);
        self
    }
}

#[derive(strum::Display, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum ButtonType {
    Submit, // default
    Reset,
    Button,
}

impl<T: AsRef<str>> From<T> for ButtonType {
    fn from(s: T) -> Self {
        let button_type = s.as_ref().trim().to_lowercase();
        match button_type.as_str() {
            "submit" => Self::Submit,
            "reset" => Self::Reset,
            "button" => Self::Button,
            _ => Self::Submit,
        }
    }
}

impl From<ButtonType> for Cow<'static, str> {
    fn from(s: ButtonType) -> Self {
        <&'static str>::from(s).into()
    }
}

/// Shorthand for `HtmlButton<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let button = button!().id("button");
///
/// assert_eq!(button.bake(),
/// r#"<button id="button"></button>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let button = button!(@newline "Add to favorites")
///     .button_type("button")
///     .name("favorite");
///
/// assert_eq!(button.bake(),
/// r#"<button type="button" name="favorite">
///   Add to favorites
/// </button>"#);
/// ```
#[macro_export]
macro_rules! button {
    () => {
        $crate::html::HtmlButton::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlButton::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlButton::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlButton::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlButton::<()>::new($crate::bake_inline![$($content),+])
    };

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlButton::<$crate::rec!($($r),+)>::empty()
    };
    (@recipe $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlButton::<$crate::rec!($($r),+)>::new($content)
    };
    (@recipe $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlButton::<$crate::rec!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@recipe $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlButton::<$crate::rec!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@recipe $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlButton::<$crate::rec!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
