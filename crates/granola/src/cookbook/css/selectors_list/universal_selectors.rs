use crate::{cookbook::*, prelude::*};

/// The `*, ::after, ::before` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let list: CssSelectorsList<UniversalSelectors> = CssSelectorsList::from_recipe();
///
/// assert_eq!(
///     list.bake(),
///     "*,
/// ::after,
/// ::before"
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalSelectors;

impl SelectorsListRecipe for UniversalSelectors {
    fn selectors_recipe(selectors: &mut Vec<CssSelector>) {
        selectors.extend([
            CssSimpleSelector::<Universal>::from_recipe().into(),
            CssSimpleSelector::<UniversalAfter>::from_recipe().into(),
            CssSimpleSelector::<UniversalBefore>::from_recipe().into(),
        ]);
    }
}
