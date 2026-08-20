# Baking Strings

The `Granola` derive adds a `bake()` method to any type that already derives
`askama::Template`. `bake()` returns the rendered template as a `String`. The
derive also implements `From<T>` and `From<&T>` for `Bake`. A `Bake` can then
be converted into `Cow<'static, str>` or `String`.
