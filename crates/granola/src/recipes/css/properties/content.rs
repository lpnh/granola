use crate::prelude::*;

/// The recipe for the CSS `value` property.
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/CSS/Reference/Properties/value)
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let css_value = CssDeclaration::from(Content).value(r#""this is new""#);
///
/// assert_eq!(css_value.bake(), r#"value: "this is new";"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct Content;

impl DeclarationRecipe for Content {
    fn property_recipe() -> Bake {
        "value".into()
    }
}
