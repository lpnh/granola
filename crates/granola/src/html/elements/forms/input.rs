use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::prelude::*;

pub trait InputTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles:
    ///     type=button: checkbox, combobox, link, menuitem, menuitemcheckbox, menuitemradio,
    ///     option, radio, switch, tab
    ///     type=checkbox: button when used with aria-pressed, menuitemcheckbox, option, switch
    ///     type=image: link, menuitem, menuitemcheckbox, menuitemradio, radio, switch
    ///     type=radio: menuitemradio
    ///     type=text with no list attribute: combobox, searchbox, spinbutton
    ///     type=color|date|datetime-local|email|file|hidden|month|number|password|range|reset|search|submit|tel|url|week
    ///     or text with list attribute: no role permitted
    const ROLE: Option<&'static str> = None;
    const TYPE: Option<InputType> = None;
}

impl InputTag for () {}

/// The HTML `<input>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let input: HtmlInput = HtmlInput::empty().id("html_input");
///
/// assert_eq!(input.bake(),
/// r#"<input id="html_input" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let input: HtmlInput = HtmlInput::new("nickname").minlength(4).required(true);
///
/// assert_eq!(input.bake(),
/// r#"<input name="nickname" minlength="4" required />"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <input
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs }} />
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlInput<M: InputTag = ()> {
    _marker: PhantomData<M>,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: InputTag> HtmlInput<M> {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        if let Some(t) = M::TYPE {
            s = s.input_type(t);
        }
        s.name(name)
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        if let Some(role) = M::ROLE {
            s = s.role(role);
        }
        if let Some(t) = M::TYPE {
            s = s.input_type(t);
        }
        s
    }

    pub fn from_type(input_type: impl Into<InputType>) -> Self {
        Self::empty().input_type(input_type.into())
    }

    pub fn validate(self) -> Self {
        self.class("validator")
    }

    /// Hint for expected file type in file upload controls.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/accept)
    pub fn accept(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("accept", value);
        self
    }

    /// Allow the color's alpha component to be set.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#alpha)
    pub fn alpha(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("alpha");
        }
        self
    }

    /// Replacement text for use when images are not available.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#alt)
    pub fn alt(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("alt", value);
        self
    }

    /// Hint for form autofill feature.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    pub fn autocomplete(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("autocomplete", value);
        self
    }

    /// Whether the control is checked.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#checked)
    pub fn checked(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("checked");
        }
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

    /// URL to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formaction)
    pub fn formaction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("formaction", value);
        self
    }

    /// Entry list encoding type to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formenctype)
    pub fn formenctype(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("formenctype", value);
        self
    }

    /// Variant to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formmethod)
    pub fn formmethod(mut self, value: impl Into<FormMethod>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("formmethod", value.into());
        self
    }

    /// Bypass form control validation for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formnovalidate)
    pub fn formnovalidate(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("formnovalidate");
        }
        self
    }

    /// Navigable for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formtarget)
    pub fn formtarget(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("formtarget", value);
        self
    }

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#height)
    pub fn height(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("height", value.to_string());
        self
    }

    /// List of autocomplete options.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#list)
    pub fn list(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("list", value);
        self
    }

    /// Defines the greatest value in the range of permitted values.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/max)
    pub fn max(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("max", value);
        self
    }

    /// Maximum length of value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/maxlength)
    pub fn maxlength(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("maxlength", value.to_string());
        self
    }

    /// Defines the most negative value in the range of permitted value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/min)
    pub fn min(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("min", value);
        self
    }

    /// Minimum length of value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/minlength)
    pub fn minlength(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("minlength", value.to_string());
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

    /// Name of the element to use for form submission and in the form.elements API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#name)
    pub fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("name", value);
        self
    }

    /// Pattern to be matched by the form control's value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/pattern)
    pub fn pattern(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("pattern", value);
        self
    }

    /// User-visible label to be placed within the form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/placeholder)
    pub fn placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("placeholder", value);
        self
    }

    /// Targets a popover element to toggle, show, or hide.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#popovertarget)
    pub fn popovertarget(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("popovertarget", value);
        self
    }

    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#popovertargetaction)
    pub fn popovertargetaction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("popovertargetaction", value);
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

    /// Size of the control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/size)
    pub fn size(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("size", value.to_string());
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#src)
    pub fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("src", value);
        self
    }

    /// Granularity to be matched by the form control's value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/step)
    pub fn step(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("step", value);
        self
    }

    /// Type of form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#input_types)
    pub fn input_type(mut self, value: impl Into<InputType>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("type", value.into());
        self
    }

    /// Value of the form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#value)
    pub fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("value", value);
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#width)
    pub fn width(mut self, value: u32) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("width", value.to_string());
        self
    }
}

#[derive(strum::Display, strum::IntoStaticStr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum InputType {
    Button,
    Checkbox,
    Color,
    Date,
    #[strum(serialize = "datetime-local")]
    Datetime,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Radio,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text, // default
    Time,
    Url,
    Week,
}

impl<T: AsRef<str>> From<T> for InputType {
    fn from(s: T) -> Self {
        let input_type = s.as_ref().trim().to_lowercase();
        match input_type.as_str() {
            "button" => Self::Button,
            "checkbox" => Self::Checkbox,
            "color" => Self::Color,
            "date" => Self::Date,
            "datetime" => Self::Datetime,
            "email" => Self::Email,
            "file" => Self::File,
            "hidden" => Self::Hidden,
            "image" => Self::Image,
            "month" => Self::Month,
            "number" => Self::Number,
            "password" => Self::Password,
            "radio" => Self::Radio,
            "range" => Self::Range,
            "reset" => Self::Reset,
            "search" => Self::Search,
            "submit" => Self::Submit,
            "tel" => Self::Tel,
            "text" => Self::Text,
            "time" => Self::Time,
            "url" => Self::Url,
            "week" => Self::Week,
            _ => Self::Text,
        }
    }
}

impl From<InputType> for Cow<'static, str> {
    fn from(s: InputType) -> Self {
        <&'static str>::from(s).into()
    }
}

/// Shorthand for `HtmlInput<()>`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let input = input!().id("html_input");
///
/// assert_eq!(input.bake(),
/// r#"<input id="html_input" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let input = input!("nickname").minlength(4).required(true);
///
/// assert_eq!(input.bake(),
/// r#"<input name="nickname" minlength="4" required />"#);
/// ```
#[macro_export]
macro_rules! input {
    () => {
        $crate::html::HtmlInput::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlInput::<()>::new($content)
    };
    (@from_type $type: expr $(,)?) => {
        $crate::html::HtmlInput::<()>::from_type($type)
    };
}
