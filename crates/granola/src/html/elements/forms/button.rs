use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// assert_eq!(button.bake(), r#"<button id="button"></button>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let button: HtmlButton = HtmlButton::new(bake_newline!("Add to favorites"))
///     .button_type("button")
///     .name("favorite");
///
/// assert_eq!(
///     button.bake(),
///     r#"<button type="button" name="favorite">
///   Add to favorites
/// </button>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <button
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</button>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = ButtonTag, content = Cow<'static, str>)]
pub struct HtmlButton<R: ButtonTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// checkbox, combobox, link, menuitem, menuitemcheckbox, menuitemradio,
    /// option, radio, switch, tab
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: ButtonAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<button>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- button_type | bake_attr("type") -}}
/// {{- name | bake_attr("name") -}}
/// {{- value | bake_attr("value") -}}
/// {{- commandfor | bake_attr("commandfor") -}}
/// {{- command | bake_attr("command") -}}
/// {{- form | bake_attr("form") -}}
/// {{- formaction | bake_attr("formaction") -}}
/// {{- formenctype | bake_attr("formenctype") -}}
/// {{- formmethod | bake_attr("formmethod") -}}
/// {{- formtarget | bake_attr("formtarget") -}}
/// {{- popovertarget | bake_attr("popovertarget") -}}
/// {{- popovertargetaction | bake_attr("popovertargetaction") -}}
/// {{- formnovalidate | bake_bool_attr("formnovalidate") -}}
/// {{- disabled | bake_bool_attr("disabled") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct ButtonAttrs {
    pub button_type: Option<Cow<'static, str>>,
    pub name: Option<Cow<'static, str>>,
    pub value: Option<Cow<'static, str>>,
    pub commandfor: Option<Cow<'static, str>>,
    pub command: Option<Cow<'static, str>>,
    pub form: Option<Cow<'static, str>>,
    pub formaction: Option<Cow<'static, str>>,
    pub formenctype: Option<Cow<'static, str>>,
    pub formmethod: Option<Cow<'static, str>>,
    pub formtarget: Option<Cow<'static, str>>,
    pub popovertarget: Option<Cow<'static, str>>,
    pub popovertargetaction: Option<Cow<'static, str>>,
    pub formnovalidate: bool,
    pub disabled: bool,
}

pub trait HasButtonAttrs: Sized {
    fn button_attrs_mut(&mut self) -> &mut ButtonAttrs;

    /// Indicates to the targeted element which action to take.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#command)
    fn command(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().command = Some(value.into());
        self
    }

    /// Targets another element to be invoked.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#commandfor)
    fn commandfor(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().commandfor = Some(value.into());
        self
    }

    /// Whether the form control is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    fn disabled(mut self, value: bool) -> Self {
        self.button_attrs_mut().disabled = value;
        self
    }

    /// Associates the element with a form element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/form)
    fn form(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().form = Some(value.into());
        self
    }

    /// URL to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formaction)
    fn formaction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().formaction = Some(value.into());
        self
    }

    /// Entry list encoding type to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formenctype)
    fn formenctype(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().formenctype = Some(value.into());
        self
    }

    /// Variant to use for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formmethod)
    fn formmethod(mut self, value: impl Into<FormMethod>) -> Self {
        self.button_attrs_mut().formmethod = Some(value.into().into());
        self
    }

    /// Bypass form control validation for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formnovalidate)
    fn formnovalidate(mut self, value: bool) -> Self {
        self.button_attrs_mut().formnovalidate = value;
        self
    }

    /// Navigable for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#formtarget)
    fn formtarget(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().formtarget = Some(value.into());
        self
    }

    // NOTE: Include `interestfor` in the future.
    // [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#interestfor)

    /// Name of the element to use for form submission and in the
    /// `form.elements` API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#name)
    fn name(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().name = Some(value.into());
        self
    }

    /// Targets a popover element to toggle, show, or hide.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#popovertarget)
    fn popovertarget(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().popovertarget = Some(value.into());
        self
    }

    /// Indicates whether a targeted popover element is to be toggled, shown, or
    /// hidden.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#popovertargetaction)
    fn popovertargetaction(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().popovertargetaction = Some(value.into());
        self
    }

    /// Type of button.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#type)
    fn button_type(mut self, value: impl Into<ButtonType>) -> Self {
        self.button_attrs_mut().button_type = Some(value.into().into());
        self
    }

    /// Value to be used for form submission.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/button#value)
    fn value(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.button_attrs_mut().value = Some(value.into());
        self
    }
}

impl HasButtonAttrs for ButtonAttrs {
    fn button_attrs_mut(&mut self) -> &mut ButtonAttrs {
        self
    }
}

impl HasButtonAttrs for &mut ButtonAttrs {
    fn button_attrs_mut(&mut self) -> &mut ButtonAttrs {
        self
    }
}

impl<R: ButtonTag> HasButtonAttrs for HtmlButton<R> {
    fn button_attrs_mut(&mut self) -> &mut ButtonAttrs {
        &mut self.specific_attrs
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

/// Shorthand for `HtmlButton`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let button = button!().id("button");
///
/// assert_eq!(button.bake(), r#"<button id="button"></button>"#);
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
        $crate::html::HtmlButton::<$crate::cookbook!($($r),+)>::from_recipe()
    };
    (@recipe $($r:ty),+ ; $content:expr $(,)?) => {
        $crate::html::HtmlButton::<$crate::cookbook!($($r),+)>::new($content)
    };
    (@recipe $($r:ty),+ ; $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlButton::<$crate::cookbook!($($r),+)>::new($crate::bake_block![$first $(, $rest)*])
    };
    (@recipe $($r:ty),+ ; @newline $content:expr $(,)?) => {
        $crate::html::HtmlButton::<$crate::cookbook!($($r),+)>::new($crate::bake_newline!($content))
    };
    (@recipe $($r:ty),+ ; @inline $($content:expr),+ $(,)?) => {
        $crate::html::HtmlButton::<$crate::cookbook!($($r),+)>::new($crate::bake_inline![$($content),+])
    };
}
