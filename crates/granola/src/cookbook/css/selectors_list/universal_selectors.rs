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
            CssSelector::<Universal>::from_recipe().bake_recipe(),
            CssSelector::<UniversalAfter>::from_recipe().bake_recipe(),
            CssSelector::<UniversalBefore>::from_recipe().bake_recipe(),
        ]);
    }
}
