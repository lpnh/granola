use askama::Template;
use std::{fmt::Debug, marker::PhantomData};

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
/// let input = HtmlInput::new().id("html_input");
///
/// assert_eq!(input.bake(), r#"<input id="html_input" />"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let input = HtmlInput::new()
///     .name("nickname")
///     .minlength(4)
///     .required(true);
///
/// assert_eq!(
///     input.bake(),
///     r#"<input name="nickname" minlength="4" required />"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <input
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers }} />
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = InputRecipe)]
pub struct HtmlInput<R: InputRecipe = ()> {
    _recipe: PhantomData<R>,
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
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: InputAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

impl HtmlInput {
    pub fn from_name(name: impl Into<Bake>) -> Self {
        Self::new().name(name)
    }

    pub fn from_value(value: impl Into<Bake>) -> Self {
        Self::new().value(value)
    }

    pub fn from_type(input_type: impl Into<Bake>) -> Self {
        Self::new().input_type(input_type)
    }
}

/// The HTML `<input />` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- input_type | bake_attr("type") -}}
/// {{- name | bake_attr("name") -}}
/// {{- form | bake_attr("form") -}}
/// {{- value | bake_attr("value") -}}
/// {{- autocomplete | bake_attr("autocomplete") -}}
/// {{- list | bake_attr("list") -}}
/// {{- placeholder | bake_attr("placeholder") -}}
/// {{- min | bake_attr("min") -}}
/// {{- max | bake_attr("max") -}}
/// {{- step | bake_attr("step") -}}
/// {{- minlength | bake_attr("minlength") -}}
/// {{- maxlength | bake_attr("maxlength") -}}
/// {{- pattern | bake_attr("pattern") -}}
/// {{- size | bake_attr("size") -}}
/// {{- dirname | bake_attr("dirname") -}}
/// {{- formaction | bake_attr("formaction") -}}
/// {{- formenctype | bake_attr("formenctype") -}}
/// {{- formmethod | bake_attr("formmethod") -}}
/// {{- formtarget | bake_attr("formtarget") -}}
/// {{- popovertarget | bake_attr("popovertarget") -}}
/// {{- popovertargetaction | bake_attr("popovertargetaction") -}}
/// {{- src | bake_attr("src") -}}
/// {{- alt | bake_attr("alt") -}}
/// {{- width | bake_attr("name") -}}
/// {{- height | bake_attr("height") -}}
/// {{- accept | bake_attr("accept") -}}
/// {{- alpha | bake_bool_attr("alpha") -}}
/// {{- multiple | bake_bool_attr("multiple") -}}
/// {{- formnovalidate | bake_bool_attr("formnovalidate") -}}
/// {{- checked | bake_bool_attr("checked") -}}
/// {{- readonly | bake_bool_attr("readonly") -}}
/// {{- required | bake_bool_attr("required") -}}
/// {{- disabled | bake_bool_attr("disabled") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct InputAttrs {
    /// Available for all input types.
    pub input_type: Option<Bake>,
    /// Available for all input types.
    pub name: Option<Bake>,
    /// Available for all input types.
    pub form: Option<Bake>,
    /// Available for all input types except image.
    pub value: Option<Bake>,
    /// Available for all input types except checkbox, radio, and button.
    pub autocomplete: Option<Bake>,
    /// Available for all input types except hidden, password, checkbox, radio,
    /// and button.
    pub list: Option<Bake>,
    /// Available for text, search, url, tel, email, password, and number input
    /// types.
    pub placeholder: Option<Bake>,
    /// Available for date, month, week, time, datetime-local, number, and range
    /// input types.
    pub min: Option<Bake>,
    /// Available for date, month, week, time, datetime-local, number, and range
    /// input types.
    pub max: Option<Bake>,
    /// Available for date, month, week, time, datetime-local, number, and range
    /// input types.
    pub step: Option<Bake>,
    /// Available for text, search, url, tel, email, and password input types.
    pub minlength: Option<u32>,
    /// Available for text, search, url, tel, email, and password input types.
    pub maxlength: Option<u32>,
    /// Available for text, search, url, tel, email, and password input types.
    pub pattern: Option<Bake>,
    /// Available for text, search, url, tel, email, and password input types.
    pub size: Option<u32>,
    /// Available for hidden, text, search, url, tel, and email input types.
    pub dirname: Option<Bake>,
    /// Available for the image and submit input types.
    pub formaction: Option<Bake>,
    /// Available for the image and submit input types.
    pub formenctype: Option<Bake>,
    /// Available for the image and submit input types.
    pub formmethod: Option<Bake>,
    /// Available for the image and submit input types.
    pub formtarget: Option<Bake>,
    /// Available for the button input type.
    pub popovertarget: Option<Bake>,
    /// Available for the button input type.
    pub popovertargetaction: Option<Bake>,
    /// Available for the image input type.
    pub src: Option<Bake>,
    /// Available for the image input type.
    pub alt: Option<Bake>,
    /// Available for the image input type.
    pub width: Option<u32>,
    /// Available for the image input type.
    pub height: Option<u32>,
    /// Available for the file input type.
    pub accept: Option<Bake>,
    /// Available for the color input type.
    pub alpha: bool,
    /// Available for the email and file input types.
    pub multiple: bool,
    /// Available for the image and submit input types.
    pub formnovalidate: bool,
    /// Available for the checkbox and radio input types.
    pub checked: bool,
    /// Available for all input types except hidden, range, color, checkbox,
    /// radio, and button.
    pub readonly: bool,
    /// Available for all input types except hidden, range, color, and button.
    pub required: bool,
    /// Available for all input types.
    pub disabled: bool,
}

pub trait HasInputAttrs: Sized {
    fn input_attrs_mut(&mut self) -> &mut InputAttrs;

    /// Hint for expected file type in file upload controls.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/accept)
    fn accept(mut self, value: impl Into<Bake>) -> Self {
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
    fn alt(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().alt = Some(value.into());
        self
    }

    /// Hint for form autofill feature.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/autocomplete)
    fn autocomplete(mut self, value: impl Into<Bake>) -> Self {
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

    // NOTE: Include `colorspace` in the future.
    //
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#colorspace)

    /// Name of form control to use for sending the element's directionality in
    /// form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/dirname)
    fn dirname(mut self, value: impl Into<Bake>) -> Self {
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
    fn form(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().form = Some(value.into());
        self
    }

    /// URL to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formaction)
    fn formaction(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().formaction = Some(value.into());
        self
    }

    /// Entry list encoding type to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#formenctype)
    fn formenctype(mut self, value: impl Into<Bake>) -> Self {
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
    fn formtarget(mut self, value: impl Into<Bake>) -> Self {
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
    fn list(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().list = Some(value.into());
        self
    }

    /// Defines the greatest value in the range of permitted values.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/max)
    fn max(mut self, value: impl Into<Bake>) -> Self {
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
    fn min(mut self, value: impl Into<Bake>) -> Self {
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

    /// Name of the element to use for form submission and in the form.elements
    /// API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#name)
    fn name(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().name = Some(value.into());
        self
    }

    /// Pattern to be matched by the form control's value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/pattern)
    fn pattern(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().pattern = Some(value.into());
        self
    }

    /// User-visible label to be placed within the form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/placeholder)
    fn placeholder(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().placeholder = Some(value.into());
        self
    }

    /// Targets a popover element to toggle, show, or hide.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#popovertarget)
    fn popovertarget(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().popovertarget = Some(value.into());
        self
    }

    /// Indicates whether a targeted popover element is to be toggled, shown, or
    /// hidden.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#popovertargetaction)
    fn popovertargetaction(mut self, value: impl Into<Bake>) -> Self {
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
    fn src(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().src = Some(value.into());
        self
    }

    /// Granularity to be matched by the form control's value.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/step)
    fn step(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().step = Some(value.into());
        self
    }

    /// Type of form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#input_types)
    ///
    /// See [`InputType`]
    fn input_type(mut self, value: impl Into<Bake>) -> Self {
        self.input_attrs_mut().input_type = Some(value.into());
        self
    }

    /// Value of the form control.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/input#value)
    fn value(mut self, value: impl Into<Bake>) -> Self {
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

impl<R: InputRecipe> HasInputAttrs for HtmlInput<R> {
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
    Text,
    Time,
    Url,
    Week,
}

impl From<InputType> for Bake {
    fn from(input_type: InputType) -> Self {
        <&'static str>::from(input_type).into()
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
/// assert_eq!(input.bake(), r#"<input id="html_input" />"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let input = input!().name("nickname").minlength(4).required(true);
///
/// assert_eq!(
///     input.bake(),
///     r#"<input name="nickname" minlength="4" required />"#
/// );
/// ```
#[macro_export]
macro_rules! input {
    () => {
        $crate::html::HtmlInput::new()
    };
}
