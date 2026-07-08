use granola::{prelude::*, recipes::*};

/// The `sub { bottom: -0.25em }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SubVerticalPos);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "sub {
///   bottom: -0.25em;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SubVerticalPos;

impl RuleRecipe for SubVerticalPos {
    recipe_boilerplate!(StylesheetRecipe);

    fn selectors_list_recipe() -> Bake {
        "sub".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from(Bottom).content("-0.25em").into()
    }
}
