use askama::{FastWritable, Template};
use std::{fmt::Debug, marker::PhantomData};

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
/// let dialog = HtmlDialog::new().id("dialog");
///
/// assert_eq!(dialog.bake(), r#"<dialog id="dialog"></dialog>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let close_button = HtmlButton::new()
///     .content("Close")
///     .popovertarget("modal_popover")
///     .popovertargetaction("hide");
///
/// let dialog = HtmlDialog::new()
///     .fold_in("Hello, there!")
///     .fold_in(close_button)
///     .id("modal_popover")
///     .popover("auto");
///
/// assert_eq!(
///     dialog.bake_pretty(),
///     r#"<dialog id="modal_popover" popover="auto">
///   Hello, there!<button popovertarget="modal_popover" popovertargetaction="hide">
///     Close
///   </button>
/// </dialog>
/// "#
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
/// >{{ content | kirei }}</dialog>
/// ```
#[derive(Debug, Clone, Default, PartialEq, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = DialogRecipe, content = Bake)]
pub struct HtmlDialog<R: DialogRecipe = ()> {
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

impl<R: DialogRecipe<Content = Bake>> HtmlDialog<R> {
    pub fn fold_in(mut self, content: impl FastWritable) -> Self {
        self.content.fold_in(content);
        self
    }
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
#[derive(Debug, Clone, Default, PartialEq, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct DialogAttrs {
    pub closedby: Option<Bake>,
    pub open: bool,
}

pub trait HasDialogAttrs: Sized {
    fn dialog_attrs_mut(&mut self) -> &mut DialogAttrs;

    /// Which user actions will close the dialog.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/dialog#closedby)
    fn closedby(mut self, value: impl Into<Bake>) -> Self {
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

impl<R: DialogRecipe> HasDialogAttrs for HtmlDialog<R> {
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
/// let close_button = button!("Close")
///     .popovertarget("modal_popover")
///     .popovertargetaction("hide");
///
/// let dialog = dialog!("Hello, there!", close_button)
///     .id("modal_popover")
///     .popover("auto");
///
/// assert_eq!(
///     dialog.bake_pretty(),
///     r#"<dialog id="modal_popover" popover="auto">
///   Hello, there!<button popovertarget="modal_popover" popovertargetaction="hide">
///     Close
///   </button>
/// </dialog>
/// "#
/// );
/// ```
#[macro_export]
macro_rules! dialog {
    () => {
        $crate::html::HtmlDialog::new()
    };
    ($content:expr $(,)?) => {
        $crate::html::HtmlDialog::new().content($content)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::html::HtmlDialog::new().content($crate::bake![$first $(, $rest)*])
    };

}
