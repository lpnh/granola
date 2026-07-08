use crate::{prelude::*, recipes::*};

/// The `b, strong { font-weight: bolder }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(BStrongFontWeight);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "b, strong {
///   font-weight: bolder;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BStrongFontWeight;

impl RuleRecipe for BStrongFontWeight {
    recipe_boilerplate!(StylesheetRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma!["b", "strong"]
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from(FontWeight).content("bolder").into()
    }
}
