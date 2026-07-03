use crate::prelude::*;

/// The `fill="currentColor"` recipe.
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let svg = Svg::from(FillCurrentColor);
///
/// assert_eq!(svg.bake(), r#"<svg fill="currentColor"></svg>"#);
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct FillCurrentColor;

impl SvgRecipe for FillCurrentColor {
    recipe_boilerplate!(SvgRecipe);

    fn paint_attrs_recipe() -> PaintAttrs {
        PaintAttrs::default().fill("currentColor")
    }
}
