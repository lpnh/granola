use askama::Template;
use std::{borrow::Cow, fmt::Debug, marker::PhantomData};

use crate::{filters, prelude::*};

/// The HTML `<ol>` element.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let ol: HtmlOl = HtmlOl::empty().id("ordered_list");
///
/// assert_eq!(ol.bake(), r#"<ol id="ordered_list"></ol>"#);
/// ```
///
/// ```rust
/// use granola::prelude::*;
///
/// let items = [
///     HtmlLi::new("Add the sugar"),
///     HtmlLi::new("Coat with spice"),
///     HtmlLi::new("Fold in everything nice"),
/// ];
///
/// let instructions: HtmlOl = HtmlOl::new(items);
///
/// assert_eq!(
///     instructions.bake(),
///     r#"<ol>
///   <li>Add the sugar</li>
///   <li>Coat with spice</li>
///   <li>Fold in everything nice</li>
/// </ol>"#
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// <ol
///   {{- global_attrs -}}
///   {{- specific_attrs -}}
///   {{- global_aria_attrs -}}
///   {{- custom_data_attrs -}}
///   {{- event_handlers -}}
/// >{{ content | kirei(2) }}</ol>
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
#[recipe(name = OlTag, content = ListItems)]
pub struct HtmlOl<R: OlTag = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
    /// # Permitted ARIA roles
    ///
    /// directory, group, listbox, menu, menubar, none, presentation,
    /// radiogroup, tablist, toolbar, tree
    pub global_attrs: GlobalAttrs,
    pub specific_attrs: OlAttrs,
    pub global_aria_attrs: GlobalAriaAttrs,
    pub custom_data_attrs: CustomDataAttrs,
    pub event_handlers: EventHandlers,
}

/// The HTML `<ol>` element specific attributes.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol#attributes)
///
/// # Askama template
///
/// ```askama
/// {{- start | bake_attr("start") -}}
/// {{- list_type | bake_attr("type") -}}
/// {{- reversed | bake_bool_attr("reversed") -}}
/// ```
#[derive(Debug, Clone, Default, Template)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct OlAttrs {
    pub start: Option<i32>,
    pub list_type: Option<Cow<'static, str>>,
    pub reversed: bool,
}

pub trait HasOlAttrs: Sized {
    fn ol_attrs_mut(&mut self) -> &mut OlAttrs;

    /// Number the list backwards.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol#reversed)
    fn reversed(mut self, value: bool) -> Self {
        self.ol_attrs_mut().reversed = value;
        self
    }

    /// Starting value of the list.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol#start)
    fn start(mut self, value: i32) -> Self {
        self.ol_attrs_mut().start = Some(value);
        self
    }

    /// Kind of list marker.
    ///
    /// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Reference/Elements/ol#type)
    fn list_type(mut self, value: impl Into<Cow<'static, str>>) -> Self {
        self.ol_attrs_mut().list_type = Some(value.into());
        self
    }
}

impl HasOlAttrs for OlAttrs {
    fn ol_attrs_mut(&mut self) -> &mut OlAttrs {
        self
    }
}

impl HasOlAttrs for &mut OlAttrs {
    fn ol_attrs_mut(&mut self) -> &mut OlAttrs {
        self
    }
}

impl<R: OlTag> HasOlAttrs for HtmlOl<R> {
    fn ol_attrs_mut(&mut self) -> &mut OlAttrs {
        &mut self.specific_attrs
    }
}

/// Shorthand for `HtmlOl`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let ol = ol!().id("ordered_list");
///
/// assert_eq!(ol.bake(), r#"<ol id="ordered_list"></ol>"#);
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let items = [
///     li!("Add the sugar"),
///     li!("Coat with spice"),
///     li!("Fold in everything nice"),
/// ];
///
/// let instructions = ol!(items);
///
/// assert_eq!(
///     instructions.bake(),
///     r#"<ol>
///   <li>Add the sugar</li>
///   <li>Coat with spice</li>
///   <li>Fold in everything nice</li>
/// </ol>"#
/// );
/// ```
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let sugar = li!("Add the sugar");
/// let spice = li!("Coat with spice");
///
/// let instructions = ol!(sugar, spice);
///
/// assert_eq!(
///     instructions.bake(),
///     r#"<ol>
///   <li>Add the sugar</li>
///   <li>Coat with spice</li>
/// </ol>"#
/// );
/// ```
#[macro_export]
macro_rules! ol {
    () => {
        $crate::html::HtmlOl::<()>::empty()
    };
    ($content: expr $(,)?) => {
        $crate::html::HtmlOl::<()>::new($content)
    };
    ($first: expr $(, $rest: expr)+ $(,)?) => {
        $crate::html::HtmlOl::<()>::new([$first $(, $rest)*])
    };
}
