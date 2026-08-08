// use granola::daisyui::Btn;

const COMPONENT_PATH: &str = "granola::daisyui::Btn";

pub fn render() -> &'static str {
    COMPONENT_PATH
}

#[cfg(test)]
mod tests {
    #[test]
    fn comments_and_string_literals() {
        crate::assert_safelist("comments_and_strings", &[]);
    }
}
