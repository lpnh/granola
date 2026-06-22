// mod transition_behavior;
// mod transition_delay;
// mod transition_duration;
// mod transition_property;
// mod transition_timing_function;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::{filters, prelude::*};

/// The CSS `transition` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/transition)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_transition = CssTransition::new().content("background-color 150ms ease");
///
/// assert_eq!(
///     css_transition.bake(),
///     "transition: background-color 150ms ease;"
/// );
/// ```
///
/// # Askama template
///
/// ```askama
/// transition: {{ content | kirei }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TransitionRecipe, content = Cow<'static, str>)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTransition<R: TransitionRecipe = ()> {
    _recipe: PhantomData<R>,
    pub content: R::Content,
}

impl<R: TransitionRecipe> From<CssTransition<R>> for CssDeclaration {
    fn from(css_transition: CssTransition<R>) -> Self {
        Self::new("transition", css_transition.bake_recipe().content)
    }
}

impl<R: TransitionRecipe> From<CssTransition<R>> for CssDeclarationsBlock {
    fn from(css_transition: CssTransition<R>) -> Self {
        Self::new().push(css_transition)
    }
}
