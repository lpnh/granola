#[allow(unused_imports)]
use granola::daisyui::btn::{Btn, Color};

pub mod other {
    pub struct Btn;

    pub enum Color {
        Primary,
    }
}

pub fn render() {
    use other::{Btn, Color};

    let _ = Btn;
    let _ = Color::Primary;
}

#[cfg(test)]
mod tests {
    #[test]
    fn shadowed_unrelated_symbols() {
        crate::assert_safelist("shadowing_unrelated", &[]);
    }
}
