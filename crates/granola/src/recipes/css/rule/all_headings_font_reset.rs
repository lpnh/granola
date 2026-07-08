use crate::{prelude::*, recipes::*};

/// The `h1, h2, h3, h4, h5, h6` font reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(AllHeadingsFontReset);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "h1, h2, h3, h4, h5, h6 {
///   font-size: inherit;
///   font-weight: inherit;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AllHeadingsFontReset;

impl RuleRecipe for AllHeadingsFontReset {
    recipe_boilerplate!(StylesheetRecipe);

    fn selectors_list_recipe() -> Bake {
        AllHeadings::selectors_list_recipe()
    }

    fn content_recipe() -> Self::Content {
        bake_ws![
            CssDeclaration::from(FontSize).inherit(),
            CssDeclaration::from(FontWeight).inherit(),
        ]
    }
}
