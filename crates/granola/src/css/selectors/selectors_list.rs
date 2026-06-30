use askama::Template;
use std::{borrow::Cow, marker::PhantomData};

use crate::prelude::*;

/// A collection of [`CssComplexSelector`].
///
/// The selectors-list of [`CssRule`].
///
/// # Example
///
/// ```rust
/// use granola::prelude::*;
///
/// let selector = CssSimpleSelector::new().selector("p");
///
/// let selector_list = CssSelectorsList::new().push(selector);
///
/// assert_eq!(selector_list.bake(), "p");
/// ```
///
/// # Askama template
///
/// ```askama
/// {%- for s in selectors -%}
///     {{ s }}{% if !loop.last %}, {% endif %}
/// {%- endfor -%}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = SelectorsListRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssSelectorsList<R: SelectorsListRecipe = ()> {
    _recipe: PhantomData<R>,
    pub selectors: Vec<CssComplexSelector>,
}

impl<R: SelectorsListRecipe> CssSelectorsList<R> {
    pub fn push(mut self, selector: impl Into<CssComplexSelector>) -> Self {
        self.selectors.push(selector.into());
        self
    }

    pub fn push_mut(&mut self, selector: impl Into<CssComplexSelector>) -> &mut Self {
        self.selectors.push(selector.into());
        self
    }

    pub fn extend_mut(
        &mut self,
        selectors: impl IntoIterator<Item = CssComplexSelector>,
    ) -> &mut Self {
        self.selectors.extend(selectors);
        self
    }
}

impl<S: Into<CssComplexSelector>, const N: usize> From<[S; N]> for CssSelectorsList {
    fn from(items: [S; N]) -> Self {
        Self {
            selectors: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl<S: Into<CssComplexSelector>> From<Vec<S>> for CssSelectorsList {
    fn from(items: Vec<S>) -> Self {
        Self {
            selectors: items.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl<R: ComplexSelectorRecipe> From<CssComplexSelector<R>> for CssSelectorsList {
    fn from(complex_selector: CssComplexSelector<R>) -> Self {
        Self::new().push(complex_selector.bake_recipe())
    }
}

impl<R: CompoundSelectorRecipe> From<CssCompoundSelector<R>> for CssSelectorsList {
    fn from(compound_selector: CssCompoundSelector<R>) -> Self {
        Self::new().push(compound_selector)
    }
}

impl<R: SimpleSelectorRecipe> From<CssSimpleSelector<R>> for CssSelectorsList {
    fn from(simple_selector: CssSimpleSelector<R>) -> Self {
        Self::new().push(simple_selector)
    }
}

impl From<Cow<'static, str>> for CssSelectorsList {
    fn from(s: Cow<'static, str>) -> Self {
        Self::new().push(s)
    }
}

impl From<&'static str> for CssSelectorsList {
    fn from(s: &'static str) -> Self {
        Self::new().push(s)
    }
}

impl From<String> for CssSelectorsList {
    fn from(s: String) -> Self {
        Self::new().push(s)
    }
}

/// Shorthand for `CssSelectorsList`.
///
/// # Example
///
/// ```rust
/// use granola::{macros::*, prelude::*};
///
/// let selector_list = selectors_list!("p");
///
/// assert_eq!(selector_list.bake(), "p");
/// ```
#[macro_export]
macro_rules! selectors_list {
    () => {
        $crate::css::CssSelectorsList::new()
    };
    ($sel:expr $(,)?) => {
        $crate::css::CssSelectorsList::from($sel)
    };
    ($first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::css::CssSelectorsList::new().push($first)$(.push($rest))*
    };
    (@cookbook $r:ty $(,)?) => {
        $crate::css::CssSelectorsList::<$r>::from_cookbook()
    };
    (@cookbook $r:ty ; @push $sel:expr $(,)?) => {
        $crate::css::CssSelectorsList::<$r>::from_cookbook().push($sel)
    };
    (@cookbook $r:ty ; @push $first:expr $(, $rest:expr)+ $(,)?) => {
        $crate::css::CssSelectorsList::<$r>::from_cookbook().push($first)$(.push($rest))*
    };
}
