use crate::{cookbook::*, prelude::*};

/// The `*, *::before, *::after` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{cookbook::*, prelude::*};
///
/// let list: CssSelectorsList<UniversalReset> = CssSelectorsList::from_recipe();
///
/// assert_eq!(list.bake(), "*, *::before, *::after");
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalReset;

impl SelectorsListRecipe for UniversalReset {
    fn selectors_recipe(selectors: &mut Vec<CssSelector>) {
        selectors.extend([
            CssSelector::<Universal>::from_recipe().bake_recipe(),
            CssSelector::<UniversalBefore>::from_recipe().bake_recipe(),
            CssSelector::<UniversalAfter>::from_recipe().bake_recipe(),
        ]);
    }
}
