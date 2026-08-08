use granola::daisyui::link;

pub mod nested {
    use self::link::{Link, Modifier};
    use super::link;

    pub fn render() {
        let _ = Link;
        let _ = Modifier::Hover;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn dependent_import_order() {
        crate::assert_safelist_file("relative_modules", "order.rs", &["link", "link-hover"]);
    }
}
