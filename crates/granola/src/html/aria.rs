use askama::Template;
use indexmap::IndexMap;
use std::borrow::Cow;

use crate::filters;

/// The global ARIA attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes#global_aria_attributes)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let button: HtmlButton = HtmlButton::new("🔍").aria_label("Search");
///
/// assert_eq!(button.bake(), r#"<button aria-label="Search">🔍</button>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// {{- aria_atomic | bake_bool_attr("aria-atomic") -}}
/// {{- aria_busy | bake_bool_attr("aria-busy") -}}
/// {{- aria_controls | bake_attr("aria-controls") -}}
/// {{- aria_current | bake_attr("aria-current") -}}
/// {{- aria_describedby | bake_attr("aria-describedby") -}}
/// {{- aria_description | bake_attr("aria-description") -}}
/// {{- aria_details | bake_attr("aria-details") -}}
/// {{- aria_disabled | bake_bool_attr("aria-disabled") -}}
/// {{- aria_errormessage | bake_attr("aria-errormessage") -}}
/// {{- aria_flowto | bake_attr("aria-flowto") -}}
/// {{- aria_haspopup | bake_attr("aria-haspopup") -}}
/// {{- aria_hidden | bake_bool_attr("aria-hidden") -}}
/// {{- aria_invalid | bake_attr("aria-invalid") -}}
/// {{- aria_keyshortcuts | bake_attr("aria-keyshortcuts") -}}
/// {{- aria_label | bake_attr("aria-label") -}}
/// {{- aria_labelledby | bake_attr("aria-labelledby") -}}
/// {{- aria_live | bake_attr("aria-live") -}}
/// {{- aria_owns | bake_attr("aria-owns") -}}
/// {{- aria_relevant | bake_attr("aria-relevant") -}}
/// {{- aria_roledescription | bake_attr("aria-roledescription") -}}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct GlobalAriaAttrs {
    pub aria_atomic: bool,
    pub aria_busy: bool,
    pub aria_controls: Option<Cow<'static, str>>,
    pub aria_current: Option<Cow<'static, str>>,
    pub aria_describedby: Option<Cow<'static, str>>,
    pub aria_description: Option<Cow<'static, str>>,
    pub aria_details: Option<Cow<'static, str>>,
    pub aria_disabled: bool,
    pub aria_errormessage: Option<Cow<'static, str>>,
    pub aria_flowto: Option<Cow<'static, str>>,
    pub aria_haspopup: Option<Cow<'static, str>>,
    pub aria_hidden: bool,
    pub aria_invalid: Option<Cow<'static, str>>,
    pub aria_keyshortcuts: Option<Cow<'static, str>>,
    pub aria_label: Option<Cow<'static, str>>,
    pub aria_labelledby: Option<Cow<'static, str>>,
    pub aria_live: Option<Cow<'static, str>>,
    pub aria_owns: Option<Cow<'static, str>>,
    pub aria_relevant: Option<Cow<'static, str>>,
    pub aria_roledescription: Option<Cow<'static, str>>,
}

pub trait HasGlobalAriaAttrs: Sized {
    fn global_aria_attrs_mut(&mut self) -> &mut GlobalAriaAttrs;

    /// Indicates whether assistive technologies such as a screen reader will present all, or only
    /// parts of, the changed region based on the change notifications defined by the aria-relevant
    /// attribute.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-atomic)
    fn aria_atomic(mut self, value: bool) -> Self {
        self.global_aria_attrs_mut().aria_atomic = value;
        self
    }

    /// Indicates whether an element is currently being modified. It helps assistive technologies
    /// understand that changes to the content are not yet complete, and that they may want to wait
    /// before informing users of the update.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-busy)
    fn aria_busy(mut self, value: bool) -> Self {
        self.global_aria_attrs_mut().aria_busy = value;
        self
    }

    /// Identifies the element (or elements) whose contents or presence are controlled by the
    /// element on which this attribute is set.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-controls)
    fn aria_controls(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_controls = Some(value.into());
        self
    }

    /// Indicates that this element represents the current item within a container or set of related
    /// elements.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-current)
    fn aria_current(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_current = Some(value.into());
        self
    }

    /// Identifies the element (or elements) that describes the element on which the attribute is
    /// set.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-describedby)
    fn aria_describedby(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_describedby = Some(value.into());
        self
    }

    /// Defines a string value that describes or annotates the current element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-description)
    fn aria_description(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_description = Some(value.into());
        self
    }

    /// Identifies the element (or elements) that provide additional information related to the
    /// object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-details)
    fn aria_details(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_details = Some(value.into());
        self
    }

    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise
    /// operable.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-disabled)
    fn aria_disabled(mut self, value: bool) -> Self {
        self.global_aria_attrs_mut().aria_disabled = value;
        self
    }

    /// Identifies the element (or elements) that provide an error message for the object.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-errormessage)
    fn aria_errormessage(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_errormessage = Some(value.into());
        self
    }

    /// Identifies the next element (or elements) in an alternate reading order of content. This
    /// allows assistive technology to override the general default of reading in document source
    /// order at the user's discretion.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-flowto)
    fn aria_flowto(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_flowto = Some(value.into());
        self
    }

    /// Indicates the availability and type of interactive popup element that can be triggered by
    /// the element on which the attribute is set.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-haspopup)
    fn aria_haspopup(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_haspopup = Some(value.into());
        self
    }

    /// Indicates whether the element is exposed to an accessibility API.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-hidden)
    fn aria_hidden(mut self, value: bool) -> Self {
        self.global_aria_attrs_mut().aria_hidden = value;
        self
    }

    /// Indicates the entered value does not conform to the format expected by the application.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-invalid)
    fn aria_invalid(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_invalid = Some(value.into());
        self
    }

    /// Indicates keyboard shortcuts that an author has implemented to activate or give focus to an
    /// element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-keyshortcuts)
    fn aria_keyshortcuts(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_keyshortcuts = Some(value.into());
        self
    }

    /// Defines a string value that can be used to name an element, as long as the element's role
    /// does not prohibit naming.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-label)
    fn aria_label(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_label = Some(value.into());
        self
    }

    /// Identifies the element (or elements) that labels the element it is applied to.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-labelledby)
    fn aria_labelledby(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_labelledby = Some(value.into());
        self
    }

    /// Indicates that an element will be updated, and describes the types of updates the user
    /// agents, assistive technologies, and user can expect from the live region.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-live)
    fn aria_live(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_live = Some(value.into());
        self
    }

    /// Identifies an element (or elements) in order to define a visual, functional, or contextual
    /// relationship between a parent and its child elements when the DOM hierarchy cannot be used
    /// to represent the relationship.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-owns)
    fn aria_owns(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_owns = Some(value.into());
        self
    }

    /// Indicates what notifications the user agent will trigger when the accessibility tree within
    /// a live region is modified.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-relevant)
    fn aria_relevant(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_relevant = Some(value.into());
        self
    }

    /// Defines a human-readable, author-localized description for the role of an element.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes/aria-roledescription)
    fn aria_roledescription(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.global_aria_attrs_mut().aria_roledescription = Some(value.into());
        self
    }
}

/// HTML specific aria attributes
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Attributes#aria_attribute_types)
///
/// ```askama
/// {%- for (attr, value) in map.iter() %} aria-{{ attr }}="{{ value }}"{% endfor -%}
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct SpecificAriaAttrs {
    map: IndexMap<Cow<'static, str>, Cow<'static, str>>,
}

impl SpecificAriaAttrs {
    pub fn add_aria(
        mut self,
        key: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        self.map.insert(key.into(), value.into());
        self
    }
}
