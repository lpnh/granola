#[cfg(test)]
mod tests {
    use fixtures::standard::{page, style};

    macro_rules! compare_snapshot {
        ($name:ident, $build:expr) => {
            #[test]
            fn $name() {
                let value = $build;
                insta::with_settings!({
                    omit_expression => true,
                    prepend_module_to_snapshot => false,
                }, {
                    insta::assert_snapshot!(
                        concat!(stringify!($name), "_bake"),
                        value.bake()
                    );
                    insta::assert_snapshot!(
                        concat!(stringify!($name), "_pretty"),
                        value.bake_pretty()
                    );
                });
            }
        };
    }

    compare_snapshot!(full_page, page());
    compare_snapshot!(stylesheet, style());
}
