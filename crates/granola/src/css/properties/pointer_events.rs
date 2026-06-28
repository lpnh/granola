use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `pointer-events` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/pointer-events)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_pointer_events = CssPointerEvents::new().content("none");
///
/// assert_eq!(css_pointer_events.bake(), "pointer-events: none;");
/// ```
///
/// # Askama template
///
/// ```askama
/// pointer-events: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = PointerEventsRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssPointerEvents<R: PointerEventsRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: PointerEventsRecipe> From<CssPointerEvents<R>> for CssDeclaration {
    fn from(css_pointer_events: CssPointerEvents<R>) -> Self {
        Self::new("pointer-events", css_pointer_events.bake_recipe().content)
    }
}

impl<R: PointerEventsRecipe> From<CssPointerEvents<R>> for CssDeclarationsBlock {
    fn from(css_pointer_events: CssPointerEvents<R>) -> Self {
        Self::new().push(css_pointer_events)
    }
}
