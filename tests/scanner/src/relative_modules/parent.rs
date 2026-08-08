use granola::daisyui::btn;

pub mod nested {
    pub fn render() {
        let _ = super::btn::Btn;
        let _ = super::btn::Color::Secondary;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn parent_module_imports() {
        crate::assert_safelist_file("relative_modules", "parent.rs", &["btn", "btn-secondary"]);
    }
}
