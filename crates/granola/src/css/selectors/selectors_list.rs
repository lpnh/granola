use askama::{FastWritable, Template};
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
/// {{ selectors }}
/// ```
#[derive(Debug, Clone, Default, Template, Granola, Recipe)]
#[recipe(name = SelectorsListRecipe)]
#[template(ext = "html", in_doc = true, escape = "none")]
pub struct CssSelectorsList<R: SelectorsListRecipe = ()> {
    _recipe: PhantomData<R>,
    pub selectors: Bake,
}

impl<R: SelectorsListRecipe> CssSelectorsList<R> {
    pub fn push(mut self, selector: impl FastWritable) -> Self {
        self.selectors.fold_in_with(", ", selector);
        self
    }

    pub fn push_mut(&mut self, selector: impl FastWritable) -> &mut Self {
        self.selectors.fold_in_with(", ", selector);
        self
    }

    pub fn extend_mut<S: FastWritable>(
        &mut self,
        selectors: impl IntoIterator<Item = S>,
    ) -> &mut Self {
        for selector in selectors {
            self.selectors.fold_in_with(", ", selector);
        }
        self
    }
}

impl<S: FastWritable, const N: usize> From<[S; N]> for CssSelectorsList {
    fn from(items: [S; N]) -> Self {
        let mut list = Self::new();
        for item in items {
            list.push_mut(item);
        }
        list
    }
}

impl<S: FastWritable> From<Vec<S>> for CssSelectorsList {
    fn from(items: Vec<S>) -> Self {
        let mut list = Self::new();
        for item in items {
            list.push_mut(item);
        }
        list
    }
}

impl<R: ComplexSelectorRecipe> From<CssComplexSelector<R>> for CssSelectorsList {
    fn from(complex_selector: CssComplexSelector<R>) -> Self {
        Self::new().push(complex_selector)
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

impl From<Bake> for CssSelectorsList {
    fn from(selectors: Bake) -> Self {
        Self {
            selectors,
            ..Default::default()
        }
    }
}

impl From<Cow<'static, str>> for CssSelectorsList {
    fn from(s: Cow<'static, str>) -> Self {
        Bake::from(s).into()
    }
}

impl From<&'static str> for CssSelectorsList {
    fn from(s: &'static str) -> Self {
        Bake::from(s).into()
    }
}

impl From<String> for CssSelectorsList {
    fn from(s: String) -> Self {
        Bake::from(s).into()
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
}
