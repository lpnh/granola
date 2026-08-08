use granola::{daisyui::btn, macros::*, prelude::*};

pub fn render() -> String {
    btn!("Continue")
        .color(btn::Color::Primary)
        .size(btn::Size::Lg)
        .style(btn::Style::Outline)
        .modifier(btn::Modifier::Wide)
        .bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn typed_button_usage() {
        crate::assert_safelist(
            "typed_button",
            &["btn", "btn-lg", "btn-outline", "btn-primary", "btn-wide"],
        );
    }
}
