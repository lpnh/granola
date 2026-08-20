use crate::{prelude::*, recipes::*};

/// The universal reset rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(UniversalReset);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     "*, ::after, ::before, ::backdrop, ::file-selector-button {
///   box-sizing: border-box;
///   margin: 0;
///   padding: 0;
///   border: 0 solid;
/// }
/// "
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct UniversalReset;

impl RuleRecipe for UniversalReset {
    fn selectors_list_recipe() -> Bake {
        UniversalSelectorsExt::selectors_list_recipe()
    }

    fn content_recipe() -> Bake {
        bake_ws![
            CssDeclaration::from(BoxSizing).value("border-box"),
            CssDeclaration::from(Margin).value("0"),
            CssDeclaration::from(Padding).value("0"),
            CssDeclaration::from(Border).value(bake_ws!["0", "solid"]),
        ]
    }
}
