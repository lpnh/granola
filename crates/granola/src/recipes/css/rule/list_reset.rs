use crate::{prelude::*, recipes::*};

/// The `ul[role="list"], ol[role="list"] { list-style: none }` rule recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let rule = CssRule::from(ListReset);
///
/// assert_eq!(
///     rule.bake_pretty(),
///     r#"ul[role="list"], ol[role="list"] {
///   list-style: none;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct ListReset;

impl RuleRecipe for ListReset {
    recipe_boilerplate!(RuleRecipe);

    fn selectors_list_recipe() -> Bake {
        bake_comma![r#"ul[role="list"]"#, r#"ol[role="list"]"#]
    }

    fn content_recipe() -> Self::Content {
        CssDeclaration::from(ListStyle).content("none").into()
    }
}
