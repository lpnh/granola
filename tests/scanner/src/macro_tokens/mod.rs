#[allow(unused_imports)]
use granola::{daisyui::btn, macros::*};

macro_rules! view {
    ($($tokens:tt)*) => {
        ()
    };
}

pub fn render() {
    let _ = view! { @if true { btn!("Discard") } };
}

#[cfg(test)]
mod tests {
    #[test]
    fn component_uses_inside_non_rust_macro_tokens() {
        crate::assert_safelist("macro_tokens", &["btn"]);
    }
}
