// mod transition_behavior;
// mod transition_delay;
// mod transition_duration;
// mod transition_property;
// mod transition_timing_function;

use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// The CSS `transition` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/transition)
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let css_transition: CssTransition = CssTransition::new("background-color 150ms ease");
///
/// assert_eq!(css_transition.bake(),
/// "transition: background-color 150ms ease;");
/// ```
///
/// # Askama template
///
/// ```askama
/// transition: {{ value }};
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = TransitionTag)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssTransition<R: TransitionTag = ()> {
    _recipe: PhantomData<R>,
    pub value: Cow<'static, str>,
}

impl<R: TransitionTag> CssTransition<R> {
    pub fn new(value: impl Into<Cow<'static, str>>) -> Self {
        Self {
            value: value.into(),
            ..Default::default()
        }
    }
}

impl<R: TransitionTag> From<CssTransition<R>> for CssDeclaration {
    fn from(css_transition: CssTransition<R>) -> Self {
        Self::new("transition", css_transition.value)
    }
}

impl<R: TransitionTag> From<CssTransition<R>> for CssPropertiesList {
    fn from(css_transition: CssTransition<R>) -> Self {
        Self {
            declarations: vec![css_transition.into()],
        }
    }
}
