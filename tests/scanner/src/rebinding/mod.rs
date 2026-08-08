#[allow(unused_imports)]
use granola::{daisyui::btn::Btn, prelude::*};

pub fn render() -> String {
    use granola::daisyui::{Link as Btn, link::Color as LinkColor};

    HtmlA::from(Btn)
        .color(LinkColor::Accent)
        .content("Rebound to Link")
        .bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn rebound_component_symbols() {
        crate::assert_safelist("rebinding", &["link", "link-accent"]);
    }
}
