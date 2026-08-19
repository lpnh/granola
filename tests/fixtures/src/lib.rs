pub mod css;
pub mod macros;
pub mod recipes;
pub mod standard;

#[cfg(test)]
mod fixture_tests {
    use super::*;

    #[test]
    fn html_eq_tests() {
        let standard_page = standard::page();
        let macros_page = macros::page();
        let recipes_page = recipes::page();

        assert_eq!(standard_page, macros_page);
        assert_eq!(standard_page, recipes_page);

        assert_eq!(standard_page.bake(), macros_page.bake());
        assert_eq!(standard_page.bake(), recipes_page.bake());
    }
}
