use granola::btn as button;

pub fn render() -> String {
    button!("Continue").bake()
}

#[cfg(test)]
mod tests {
    #[test]
    fn renamed_component_macros() {
        crate::assert_safelist("macro_alias", &["btn"]);
    }
}
