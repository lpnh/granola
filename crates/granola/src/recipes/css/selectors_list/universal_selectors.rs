use crate::{recipes::*, prelude::*};

/// The `*, ::after, ::before` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{recipes::*, prelude::*};
///
/// let list: CssSelectorsList<UniversalSelectors> = CssSelectorsList::from_cookbook();
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
            CssSimpleSelector::<Universal>::from_cookbook().into(),
            CssSimpleSelector::<UniversalAfter>::from_cookbook().into(),
            CssSimpleSelector::<UniversalBefore>::from_cookbook().into(),
        ]);
    }
}
