use crate::{prelude::*, recipes::*};

/// The `*, ::after, ::before` selectors list recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let list = CssSelectorsList::from(UniversalSelectors);
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
    fn selectors_recipe(selectors: &mut Vec<CssComplexSelector>) {
        selectors.extend([
            CssTypeSelector::<Universal>::from_cookbook().into(),
            CssSimpleSelector::<UniversalAfter>::from_cookbook().into(),
            CssSimpleSelector::<UniversalBefore>::from_cookbook().into(),
        ]);
    }
}
