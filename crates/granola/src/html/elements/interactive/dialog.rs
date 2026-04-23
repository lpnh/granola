use askama::Template;
use std::{
    borrow::Cow,
    fmt::{Debug, Display},
    marker::PhantomData,
};

use crate::{filters, prelude::*};

pub trait DialogTag: Default + Clone + Debug + 'static {
    const CLASS: Option<&'static str> = None;
    /// Permitted ARIA roles: alertdialog
    const ROLE: Option<&'static str> = None;
    type Content: Display + Default + Clone + Debug = Cow<'static, str>;
}

impl DialogTag for () {}

/// The HTML `<dialog>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let dialog: HtmlDialog = HtmlDialog::empty().id("dialog");
///
/// assert_eq!(dialog.bake(),
/// r#"<dialog id="dialog"></dialog>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let open_button: HtmlButton = HtmlButton::new("open dialog").popovertarget("modal_popover");
/// let close_button: HtmlButton = HtmlButton::new("Close")
///     .popovertarget("modal_popover")
///     .popovertargetaction("hide");
///
/// let dialog: HtmlDialog = HtmlDialog::new(bake_block!["Hello, there!", close_button])
///     .id("modal_popover")
///     .popover("auto");
///
/// let modal = bake_block![open_button, dialog];
///
/// assert_eq!(modal,
/// r#"<button popovertarget="modal_popover">open dialog</button>
/// <dialog id="modal_popover" popover="auto">
///   Hello, there!
///   <button popovertarget="modal_popover" popovertargetaction="hide">Close</button>
/// </dialog>"#);
/// ```
///
/// # Askama template
///
/// ```askama
/// <dialog
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- data_attrs -}}
///   {{- event_handlers -}}
///   {{- global_aria_attrs -}}
/// >{{ content | kirei(2) }}</dialog>
/// ```
#[derive(Debug, Clone, PartialEq, Default, Template, Granola, MutAttrs)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct HtmlDialog<M: DialogTag = ()> {
    _marker: PhantomData<M>,
    pub content: M::Content,
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: SpecificAttrs,
    pub data_attrs: DataAttrs,
    pub event_handlers: EventHandlers,
    pub global_aria_attrs: GlobalAriaAttrs,
}

impl<M: DialogTag> HtmlDialog<M> {
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

    /// Which user actions will close the dialog.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog#closedby)
    pub fn closedby(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.specific_attrs = self.specific_attrs.add_attr("closedby", value.into());
        self
    }

    /// Whether the dialog box is showing.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog#open)
    pub fn open(mut self, value: bool) -> Self {
        if value {
            self.specific_attrs = self.specific_attrs.add_bool_attr("open");
        }
        self
    }
}
