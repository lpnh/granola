use granola::{prelude::*, recipes::*};

/// The `small { font-size: 80% }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(SmallFontSize);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "small {
///   font-size: 80%;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SmallFontSize;

impl RuleRecipe for SmallFontSize {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        "small".into()
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from(FontSize).content("80%").into()
    }
}
