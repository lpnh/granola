use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait OptgroupTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Options;
}

impl OptgroupTag for () {}

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
/// assert_eq!(optgroup.bake(),
/// r#"<optgroup id="option_group"></optgroup>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let options: [HtmlOption; 3] = [
///     HtmlOption::new("Grape"),
///     HtmlOption::new("Mango"),
///     HtmlOption::new("Strawberry"),
/// ];
///
/// let optgroup: HtmlOptgroup = HtmlOptgroup::new(options).label("Fruits");
///
/// assert_eq!(optgroup.bake(),
/// r#"<optgroup label="Fruits">
///   <option>Grape</option>
///   <option>Mango</option>
///   <option>Strawberry</option>
/// </optgroup>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <optgroup
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</optgroup>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlOptgroup<M: OptgroupTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: OptgroupTag> HtmlOptgroup<M> {
    pub fn new(content: impl Into<M::Content>) -> Self {
        let mut s = Self {
            content: content.into(),
            ..Default::default()
        };
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
    }

    pub fn empty() -> Self {
        let mut s = Self::default();
        if let Some(class) = M::CLASS {
            s = s.class(class);
        }
        s
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

    /// User-visible label.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/optgroup#label)
    pub fn label(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("label", value.into());
        self
    }
}
