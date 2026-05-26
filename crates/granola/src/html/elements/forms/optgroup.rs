use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<optgroup>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/optgroup)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let optgroup: HtmlOptgroup = HtmlOptgroup::empty().id("option_group");
///
/// assert_eq!(
///     optgroup.bake(),
///     r#"<optgroup id="option_group"></optgroup>"#
/// );
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let options = [
///     HtmlOption::new("Grape"),
///     HtmlOption::new("Mango"),
///     HtmlOption::new("Strawberry"),
/// ];
///
/// let optgroup: HtmlOptgroup = HtmlOptgroup::new(options).label("Fruits");
///
/// assert_eq!(
///     optgroup.bake(),
///     r#"<optgroup label="Fruits">
///   <option>Grape</option>
///   <option>Mango</option>
///   <option>Strawberry</option>
/// </optgroup>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <optgroup
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</optgroup>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = OptgroupTag, content = Options)]
pub struct HtmlOptgroup<R: OptgroupTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: OptgroupAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<optgroup>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/optgroup#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- label | bake_attr("label") -}}
/// {{- disabled | bake_bool_attr("disabled") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct OptgroupAttrs {
    pub label: Option<Cow<'static, str>>,
    pub disabled: bool,
}

pub trait HasOptgroupAttrs: Sized {
    fn optgroup_attrs_mut(&mut self) -> &mut OptgroupAttrs;

    /// Whether the form control is disabled.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Attributes/disabled)
    fn disabled(mut self, value: bool) -> Self {
        self.optgroup_attrs_mut().disabled = value;
        self
    }

    /// User-visible label.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/optgroup#label)
    fn label(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.optgroup_attrs_mut().label = Some(value.into());
        self
    }
}

impl HasOptgroupAttrs for OptgroupAttrs {
    fn optgroup_attrs_mut(&mut self) -> &mut OptgroupAttrs {
        self
    }
}

impl HasOptgroupAttrs for &mut OptgroupAttrs {
    fn optgroup_attrs_mut(&mut self) -> &mut OptgroupAttrs {
        self
    }
}

impl<R: OptgroupTag> HasOptgroupAttrs for HtmlOptgroup<R> {
    fn optgroup_attrs_mut(&mut self) -> &mut OptgroupAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlOptgroup`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let optgroup = optgroup!().id("option_group");
///
/// assert_eq!(
///     optgroup.bake(),
///     r#"<optgroup id="option_group"></optgroup>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let yes = option!("Yes");
/// let no = option!("No");
///
/// let optgroup = optgroup![yes, no].label("Binary");
///
/// assert_eq!(
///     optgroup.bake(),
///     r#"<optgroup label="Binary">
///   <option>Yes</option>
///   <option>No</option>
/// </optgroup>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let options = [option!("Grape"), option!("Mango"), option!("Strawberry")];
///
/// let optgroup = optgroup!(options).label("Fruits");
///
/// assert_eq!(
///     optgroup.bake(),
///     r#"<optgroup label="Fruits">
///   <option>Grape</option>
///   <option>Mango</option>
///   <option>Strawberry</option>
/// </optgroup>"#
/// );
/// ```
#[macro_export]
macro_rules! optgroup {
    () => {
        $crate::html::HtmlOptgroup::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlOptgroup::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlOptgroup::<()>::new([$first $(, $rest)*])
    };
}
