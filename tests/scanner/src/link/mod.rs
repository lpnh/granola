use granola::{daisyui::link, macros::*, prelude::*};

pub fn render() -> String {
    link!("Read more")
        .href("https://example.com")
        .color(link::Color::Secondary)
        .modifier(link::Modifier::Hover)
        .bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn link_usage() {
        crate::assert_safelist("link", &["link", "link-hover", "link-secondary"]);
    }
}
