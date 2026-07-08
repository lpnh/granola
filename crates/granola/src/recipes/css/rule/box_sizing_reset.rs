use crate::{prelude::*, recipes::*};

/// The universal reset `box-sizing: border-box` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(BoxSizingReset);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "*, ::after, ::before {
///   box-sizing: border-box;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct BoxSizingReset;

impl RuleRecipe for BoxSizingReset {
    recipe_boilerplate!(StylesheetRecipe);

    fn selectors_list_recipe() -> Bake {
        UniversalSelectors::selectors_list_recipe()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from(BoxSizing).content("border-box").into()
    }
}
