use crate::prelude::*;

/// The `::-webkit-search-decoration { -webkit-appearance: none }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SearchDecorationAppearance);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "::-webkit-search-decoration {
///   -webkit-appearance: none;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SearchDecorationAppearance;

impl RuleRecipe for SearchDecorationAppearance {
    recipe_boilerplate!(StylesheetRecipe);

    fn selectors_list_recipe() -> Bake {
        "::-webkit-search-decoration".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from(("-webkit-appearance", "none")).into()
    }
}
