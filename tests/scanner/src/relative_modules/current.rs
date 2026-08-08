pub mod nested {
    use granola::daisyui::link;

    #[allow(unused_qualifications)]
    pub fn render() {
        let _ = self::link::Link;
        let _ = self::link::Color::Primary;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn current_module_imports() {
        crate::assert_safelist_file("relative_modules", "current.rs", &["link", "link-primary"]);
    }
}
