macro_rules! btn {
    ($content:expr) => {
        ()
    };
}

pub fn render() {
    let _ = btn!("Continue");
}

#[cfg(test)]
mod tests {
    #[test]
    fn local_component_named_macros() {
        crate::assert_safelist("local_macro", &[]);
    }
}
