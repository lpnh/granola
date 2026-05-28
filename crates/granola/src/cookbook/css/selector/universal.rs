use std::borrow::Cow;

use crate::prelude::*;

/// The `*` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<Universal> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "*");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Universal;

impl SelectorRecipe for Universal {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = "*".into();
    }
}

/// The `*::before` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<UniversalBefore> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "*::before");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalBefore;

impl SelectorRecipe for UniversalBefore {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = CssSelector::<Universal>::from_recipe()
            .compound("::before")
            .selector;
    }
}

/// The `*::after` selector recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let selector: CssSelector<UniversalAfter> = CssSelector::from_recipe();
///
/// assert_eq!(selector.bake(), "*::after");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalAfter;

impl SelectorRecipe for UniversalAfter {
    fn selector_recipe(selector: &mut Cow<'static, str>) {
        *selector = CssSelector::<Universal>::from_recipe()
            .compound("::after")
            .selector;
    }
}
