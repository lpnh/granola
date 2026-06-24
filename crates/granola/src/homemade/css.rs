use crate::{prelude::*, recipes::*};

/// The homemade recipe for the [`CssStylesheet`].
///
/// # Example
///
/// ```rust
/// use granola::{homemade::*, prelude::*};
///
/// let stylesheet = CssStylesheet::from(Garnish);
///
/// assert_eq!(
///     stylesheet.bake_pretty(),
///     r#"a:not([class]) {
///   text-decoration-skip-ink: auto;
///   color: currentcolor;
/// }
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Garnish;

impl StylesheetRecipe for Garnish {
    fn statements_recipe(statements: &mut Vec<CssStatement>) {
        statements.extend([CssRule::from(AnchorDefaults).into()]);
    }
}
