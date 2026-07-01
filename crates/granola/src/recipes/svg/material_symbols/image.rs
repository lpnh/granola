use crate::{prelude::*, recipes::*};

/// The Material Symbols `image` icon recipe.
///
/// # Example
///
/// ```rust
/// use granola::{prelude::*, recipes::*};
///
/// let image_icon = Svg::from(MaterialSymbolsImage);
///
/// assert_eq!(
///     image_icon.bake_pretty(),
///     r#"<svg height="24px" viewBox="0 -960 960 960" width="24px" fill="currentColor">
///   <path d="M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h560q33 0 56.5 23.5T840-760v560q0 33-23.5 56.5T760-120H200Zm0-80h560v-560H200v560Zm0 0v-560 560Zm80-80h400q12 0 18-11t-2-21L586-459q-6-8-16-8t-16 8L450-320l-74-99q-6-8-16-8t-16 8l-80 107q-8 10-2 21t18 11Z" />
/// </svg>
/// "#
/// );
/// ```
#[derive(Default, Debug, Clone, PartialEq)]
pub struct MaterialSymbolsImage;

impl SvgRecipe for MaterialSymbolsImage {
    recipe_boilerplate!(SvgRecipe, SvgPath<MaterialSymbolsImage>);

    fn content_recipe(content: &mut Self::Content) {
        *content = SvgPath::from(MaterialSymbolsImage);
    }

    fn specific_attrs_recipe(specific_attrs: &mut SvgAttrs) {
        MaterialSymbols::specific_attrs_recipe(specific_attrs);
    }

    fn paint_attrs_recipe(paint_attrs: &mut PaintAttrs) {
        MaterialSymbols::paint_attrs_recipe(paint_attrs);
    }
}

impl PathRecipe for MaterialSymbolsImage {
    fn specific_attrs_recipe(specific_attrs: &mut PathAttrs) {
        specific_attrs.d("M200-120q-33 0-56.5-23.5T120-200v-560q0-33 23.5-56.5T200-840h560q33 0 56.5 23.5T840-760v560q0 33-23.5 56.5T760-120H200Zm0-80h560v-560H200v560Zm0 0v-560 560Zm80-80h400q12 0 18-11t-2-21L586-459q-6-8-16-8t-16 8L450-320l-74-99q-6-8-16-8t-16 8l-80 107q-8 10-2 21t18 11Z");
    }
}
