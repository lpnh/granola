use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<input />` element.
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
///   {{- attrs -}}
///   {{- specific_attrs }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = InputTag, specific = InputAttrs)]
pub struct HtmlInput<M: InputTag = ()> {
    _marker: PhantomData<M>,
    /// # Permitted ARIA roles
    ///
    /// type=button:
    ///     checkbox, combobox, link, menuitem, menuitemcheckbox, menuitemradio,
    ///     option, radio, switch, tab
    /// type=checkbox:
    ///     button when used with aria-pressed, menuitemcheckbox, option, switch
    /// type=image:
    ///     link, menuitem, menuitemcheckbox, menuitemradio, radio, switch
    /// type=radio:
    ///     menuitemradio
    /// type=text (with no list attribute):
    ///     combobox, searchbox, spinbutton
    /// type=color|date|datetime-local|email|file|hidden|month|number|password|
    /// range|reset|search|submit|tel|url|week or text with list attribute:
    ///     no role permitted
    pub attrs: Attrs,
    pub specific_attrs: InputAttrs,
}

impl<M: InputTag> HtmlInput<M> {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = InputAttrs::default().name(name);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }

    pub fn from_value(value: impl Into<Cow<'static, str>>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = InputAttrs::default().value(value);

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }

    pub fn from_type(input_type: impl Into<InputType>) -> Self {
        let mut attrs = Attrs::default();

        M::decoration_recipe(&mut attrs);

        let mut specific_attrs = InputAttrs::default().input_type(input_type.into());

        M::specific_recipe(&mut specific_attrs);

        Self {
            attrs,
            specific_attrs,
            ..Default::default()
        }
    }
}

/// The HTML `<input />` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- accept | bake_attr("accept") -}}
/// {{- alpha | bake_bool_attr("alpha") -}}
/// {{- autocomplete | bake_attr("autocomplete") -}}
/// {{- checked | bake_bool_attr("checked") -}}
/// {{- dirname | bake_attr("dirname") -}}
/// {{- disabled | bake_bool_attr("disabled") -}}
/// {{- form | bake_attr("form") -}}
/// {{- formaction | bake_attr("formaction") -}}
/// {{- formenctype | bake_attr("formenctype") -}}
/// {{- formmethod | bake_attr("formmethod") -}}
/// {{- formnovalidate | bake_bool_attr("formnovalidate") -}}
/// {{- formtarget | bake_attr("formtarget") -}}
/// {{- height | bake_attr("height") -}}
/// {{- list | bake_attr("list") -}}
/// {{- max | bake_attr("max") -}}
/// {{- maxlength | bake_attr("maxlength") -}}
/// {{- min | bake_attr("min") -}}
/// {{- minlength | bake_attr("minlength") -}}
/// {{- multiple | bake_bool_attr("multiple") -}}
/// {{- name | bake_attr("name") -}}
/// {{- pattern | bake_attr("pattern") -}}
/// {{- placeholder | bake_attr("placeholder") -}}
/// {{- popovertarget | bake_attr("popovertarget") -}}
/// {{- popovertargetaction | bake_attr("popovertargetaction") -}}
/// {{- readonly | bake_bool_attr("readonly") -}}
/// {{- required | bake_bool_attr("required") -}}
/// {{- size | bake_attr("size") -}}
/// {{- src | bake_attr("src") -}}
/// {{- step | bake_attr("step") -}}
/// {{- input_type | bake_attr("type") -}}
/// {{- value | bake_attr("value") -}}
/// {{- width | bake_attr("name") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct InputAttrs {
    pub accept: Option<Cow<'static, str>>,
    pub alpha: bool,
    pub alt: Option<Cow<'static, str>>,
    pub autocomplete: Option<Cow<'static, str>>,
    pub checked: bool,
    pub dirname: Option<Cow<'static, str>>,
    pub disabled: bool,
    pub form: Option<Cow<'static, str>>,
    pub formaction: Option<Cow<'static, str>>,
    pub formenctype: Option<Cow<'static, str>>,
    pub formmethod: Option<Cow<'static, str>>,
    pub formnovalidate: bool,
    pub formtarget: Option<Cow<'static, str>>,
    pub height: Option<u32>,
    pub list: Option<Cow<'static, str>>,
    pub max: Option<Cow<'static, str>>,
    pub maxlength: Option<u32>,
    pub min: Option<Cow<'static, str>>,
    pub minlength: Option<u32>,
    pub multiple: bool,
    pub name: Option<Cow<'static, str>>,
    pub pattern: Option<Cow<'static, str>>,
    pub placeholder: Option<Cow<'static, str>>,
    pub popovertarget: Option<Cow<'static, str>>,
    pub popovertargetaction: Option<Cow<'static, str>>,
    pub readonly: bool,
    pub required: bool,
    pub size: Option<u32>,
    pub src: Option<Cow<'static, str>>,
    pub step: Option<Cow<'static, str>>,
    pub input_type: Option<Cow<'static, str>>,
    pub value: Option<Cow<'static, str>>,
    pub width: Option<u32>,
}

pub trait HasInputAttrs: Sized {
    fn input_attrs_mut(&mut self) -> &mut InputAttrs;

    /// Hint for expected file type in file upload controls.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/accept)
    fn accept(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().accept = Some(value.into());
        self
    }

    /// Allow the color's alpha component to be set.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#alpha)
    fn alpha(mut self, value: bool) -> Self {
        self.input_attrs_mut().alpha = value;
        self
    }

    /// Replacement text for use when images are not available.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#alt)
    fn alt(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().alt = Some(value.into());
        self
    }

    /// Hint for form autofill feature.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    fn autocomplete(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().autocomplete = Some(value.into());
        self
    }

    /// Whether the control is checked.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#checked)
    fn checked(mut self, value: bool) -> Self {
        self.input_attrs_mut().checked = value;
        self
    }

    /// Name of form control to use for sending the element's directionality in form
    /// submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/dirname)
    fn dirname(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().dirname = Some(value.into());
        self
    }

    /// Whether the form control is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    fn disabled(mut self, value: bool) -> Self {
        self.input_attrs_mut().disabled = value;
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().form = Some(value.into());
        self
    }

    /// URL to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formaction)
    fn formaction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().formaction = Some(value.into());
        self
    }

    /// Entry list encoding type to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formenctype)
    fn formenctype(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().formenctype = Some(value.into());
        self
    }

    /// Variant to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formmethod)
    fn formmethod(mut self, value: impl Into<FormMethod>) -> Self {
        self.input_attrs_mut().formmethod = Some(value.into().into());
        self
    }

    /// Bypass form control validation for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formnovalidate)
    fn formnovalidate(mut self, value: bool) -> Self {
        self.input_attrs_mut().formnovalidate = value;
        self
    }

    /// Navigable for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formtarget)
    fn formtarget(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().formtarget = Some(value.into());
        self
    }

    /// Vertical dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#height)
    fn height(mut self, value: u32) -> Self {
        self.input_attrs_mut().height = Some(value);
        self
    }

    /// List of autocomplete options.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#list)
    fn list(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().list = Some(value.into());
        self
    }

    /// Defines the greatest value in the range of permitted values.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/max)
    fn max(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().max = Some(value.into());
        self
    }

    /// Maximum length of value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/maxlength)
    fn maxlength(mut self, value: u32) -> Self {
        self.input_attrs_mut().maxlength = Some(value);
        self
    }

    /// Defines the most negative value in the range of permitted value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/min)
    fn min(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().min = Some(value.into());
        self
    }

    /// Minimum length of value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/minlength)
    fn minlength(mut self, value: u32) -> Self {
        self.input_attrs_mut().minlength = Some(value);
        self
    }

    /// Whether to allow multiple values.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/multiple)
    fn multiple(mut self, value: bool) -> Self {
        self.input_attrs_mut().multiple = value;
        self
    }

    /// Name of the element to use for form submission and in the form.elements API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().name = Some(value.into());
        self
    }

    /// Pattern to be matched by the form control's value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/pattern)
    fn pattern(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().pattern = Some(value.into());
        self
    }

    /// User-visible label to be placed within the form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/placeholder)
    fn placeholder(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().placeholder = Some(value.into());
        self
    }

    /// Targets a popover element to toggle, show, or hide.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#popovertarget)
    fn popovertarget(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().popovertarget = Some(value.into());
        self
    }

    /// Indicates whether a targeted popover element is to be toggled, shown, or hidden.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#popovertargetaction)
    fn popovertargetaction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().popovertargetaction = Some(value.into());
        self
    }

    /// Whether to allow the value to be edited by the user.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/readonly)
    fn readonly(mut self, value: bool) -> Self {
        self.input_attrs_mut().readonly = value;
        self
    }

    /// Whether the control is required for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/required)
    fn required(mut self, value: bool) -> Self {
        self.input_attrs_mut().required = value;
        self
    }

    /// Size of the control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/size)
    fn size(mut self, value: u32) -> Self {
        self.input_attrs_mut().size = Some(value);
        self
    }

    /// Address of the resource.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#src)
    fn src(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().src = Some(value.into());
        self
    }

    /// Granularity to be matched by the form control's value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/step)
    fn step(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().step = Some(value.into());
        self
    }

    /// Type of form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#input_types)
    fn input_type(mut self, value: impl Into<InputType>) -> Self {
        self.input_attrs_mut().input_type = Some(value.into().into());
        self
    }

    /// Value of the form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#value)
    fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.input_attrs_mut().value = Some(value.into());
        self
    }

    /// Horizontal dimension.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#width)
    fn width(mut self, value: u32) -> Self {
        self.input_attrs_mut().width = Some(value);
        self
    }
}

impl HasInputAttrs for InputAttrs {
    fn input_attrs_mut(&mut self) -> &mut InputAttrs {
        self
    }
}

impl HasInputAttrs for &mut InputAttrs {
    fn input_attrs_mut(&mut self) -> &mut InputAttrs {
        self
    }
}

impl<M: InputTag> HasInputAttrs for HtmlInput<M> {
    fn input_attrs_mut(&mut self) -> &mut InputAttrs {
        &mut self.specific_attrs
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

/// Shorthand for `HtmlInput`.
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

    (@from_value $value: expr $(,)?) => {
        $crate::html::HtmlInput::<()>::from_value($value)
    };
    (@from_type $type: expr $(,)?) => {
        $crate::html::HtmlInput::<()>::from_type($type)
    };

    (@recipe $($r:ty),+) => {
        $crate::html::HtmlInput::<$crate::rec!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $name:expr $(,)?) => {
        $crate::html::HtmlInput::<$crate::rec!($($r),+)>::new($name)
    };
    (@recipe $($r:ty),+ ; @from_value $value:expr $(,)?) => {
        $crate::html::HtmlInput::<$crate::rec!($($r),+)>::from_value($value)
    };
    (@recipe $($r:ty),+ ; @from_type $type:expr $(,)?) => {
        $crate::html::HtmlInput::<$crate::rec!($($r),+)>::from_type($type)
    };
}
