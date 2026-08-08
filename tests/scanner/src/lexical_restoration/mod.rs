use granola::{
    daisyui::btn::{Btn, Color},
    prelude::*,
};

pub mod other {
    pub struct Btn;

    pub enum Color {
        Secondary,
    }
}

pub fn render() -> String {
    let before = HtmlButton::from(Btn)
        .color(Color::Primary)
        .content("Before")
        .bake();

    #[allow(clippy::let_unit_value)]
    let _shadowed = {
        use other::{Btn, Color};
        let _ = Btn;
        let _ = Color::Secondary;
    };

    let after = HtmlButton::from(Btn).content("After").bake();

    format!("{before}{after}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn lexical_scope_restoration() {
        crate::assert_safelist("lexical_restoration", &["btn", "btn-primary"]);
    }
}
