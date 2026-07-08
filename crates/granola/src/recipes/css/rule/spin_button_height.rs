use crate::{prelude::*, recipes::*};

/// The `::-webkit-inner-spin-button, ::-webkit-outer-spin-button { height: auto
/// }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SpinButtonHeight);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "::-webkit-inner-spin-button, ::-webkit-outer-spin-button {
///   height: auto;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SpinButtonHeight;

impl RuleRecipe for SpinButtonHeight {
    recipe_boilerplate!(StylesheetRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma!["::-webkit-inner-spin-button", "::-webkit-outer-spin-button"]
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from(Height).content("auto").into()
    }
}
