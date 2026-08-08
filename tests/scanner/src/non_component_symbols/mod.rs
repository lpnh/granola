use granola::daisyui::{
    DaisyUIColor, DaisyUIModifier, DaisyUIStyle,
    btn::{Color, Modifier, Size, Style},
};

pub fn color_bound<T: DaisyUIColor>(_value: T) {}

pub fn modifier_types(_: Color, _: Modifier, _: Size, _: Style) {}

pub fn capability_bounds<T: DaisyUIModifier + DaisyUIStyle>(_value: T) {}

#[cfg(test)]
mod tests {
    #[test]
    fn non_component_daisyui_symbols() {
        crate::assert_safelist("non_component_symbols", &[]);
    }
}
