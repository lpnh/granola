# Baking Strings

The `Granola` derive adds a `bake()` method to any type that already derives
`askama::Template`. `bake()` returns the rendered template as a `String`. The
derive also implements `From<T> for Cow<'static, str>` (routed through
`bake()`), so a templated struct flows directly into `Cow`-typed slots.
