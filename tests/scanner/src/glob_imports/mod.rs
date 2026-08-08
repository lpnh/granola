use granola::{daisyui::btn::*, prelude::*};

pub fn render() -> String {
    HtmlButton::from(Btn)
        .color(Color::Primary)
        .size(Size::Sm)
        .content("Save")
        .bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn component_module_glob_imports() {
        crate::assert_safelist("glob_imports", &["btn", "btn-primary", "btn-sm"]);
    }
}
