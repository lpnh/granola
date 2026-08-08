use granola::html::HtmlButton;

pub struct Btn;

pub fn render() {
    let _ = Btn;
    let _ = HtmlButton::new();
}

#[cfg(test)]
mod tests {
    #[test]
    fn unrelated_component_named_symbols() {
        crate::assert_safelist("unrelated_symbols", &[]);
    }
}
