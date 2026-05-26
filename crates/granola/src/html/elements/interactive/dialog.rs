use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

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
/// assert_eq!(dialog.bake(), r#"<dialog id="dialog"></dialog>"#);
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
/// assert_eq!(
///     modal,
///     r#"<button popovertarget="modal_popover">open dialog</button>
/// <dialog id="modal_popover" popover="auto">
///   Hello, there!
///   <button popovertarget="modal_popover" popovertargetaction="hide">Close</button>
/// </dialog>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <dialog
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</dialog>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DialogTag, content = Cow<'static, str>)]
pub struct HtmlDialog<R: DialogTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// alertdialog
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: DialogAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<dialog>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- closedby | bake_attr("closedby") -}}
/// {{- open | bake_bool_attr("open") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct DialogAttrs {
    pub closedby: Option<Cow<'static, str>>,
    pub open: bool,
}

pub trait HasDialogAttrs: Sized {
    fn dialog_attrs_mut(&mut self) -> &mut DialogAttrs;

    /// Which user actions will close the dialog.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog#closedby)
    fn closedby(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.dialog_attrs_mut().closedby = Some(value.into());
        self
    }

    /// Whether the dialog box is showing.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog#open)
    fn open(mut self, value: bool) -> Self {
        self.dialog_attrs_mut().open = value;
        self
    }
}

impl HasDialogAttrs for DialogAttrs {
    fn dialog_attrs_mut(&mut self) -> &mut DialogAttrs {
        self
    }
}

impl HasDialogAttrs for &mut DialogAttrs {
    fn dialog_attrs_mut(&mut self) -> &mut DialogAttrs {
        self
    }
}

impl<R: DialogTag> HasDialogAttrs for HtmlDialog<R> {
    fn dialog_attrs_mut(&mut self) -> &mut DialogAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlDialog`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let dialog = dialog!().id("dialog");
///
/// assert_eq!(dialog.bake(), r#"<dialog id="dialog"></dialog>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let open_button = button!("open dialog").popovertarget("modal_popover");
/// let close_button = button!("Close")
///     .popovertarget("modal_popover")
///     .popovertargetaction("hide");
///
/// let dialog = dialog!["Hello, there!", close_button]
///     .id("modal_popover")
///     .popover("auto");
///
/// let modal = bake_block![open_button, dialog];
///
/// assert_eq!(
///     modal,
///     r#"<button popovertarget="modal_popover">open dialog</button>
/// <dialog id="modal_popover" popover="auto">
///   Hello, there!
///   <button popovertarget="modal_popover" popovertargetaction="hide">Close</button>
/// </dialog>"#
/// );
/// ```
#[macro_export]
macro_rules! dialog {
    () => {
        $crate::html::HtmlDialog::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlDialog::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlDialog::<()>::new($crate::bake_block![$first $(, $rest)*])
    };

    (@newline $content: expr $(,)?) => {
        $crate::html::HtmlDialog::<()>::new($crate::bake_newline!($content))
    };
    (@inline $($content: expr),+ $(,)?) => {
        $crate::html::HtmlDialog::<()>::new($crate::bake_inline![$($content),+])
    };
}
