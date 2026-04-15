# Baking Strings

The `Granola` derive adds a `bake()` method to any type that already derives
`askama::Template`. `bake()` returns the rendered template as a `String`, using
`Self::SIZE_HINT` to pre-size the buffer and `render_into` to fill it in one
pass. The derive also implements `From<T> for Cow<'static, str>` (routed
through `bake()`), so a templated struct flows directly into `Cow`-typed slots.

For composing several items into one string, the `bake_block!` and
`bake_inline!` macros accept any mix of `askama::Template` types and
`AsRef<str>` values (`&str`, `String`, `Cow`, ...) in a single call. Dispatch
is resolved at compile time via autoref-based specialization (`oven::Bake` /
`oven::Roast`): template items go through `render_into`, string items use
`push_str`. `bake_block!` joins items with a newline; `bake_inline!`
concatenates them with no separator.

The `filters` module exposes three custom Askama filters used by the HTML
element templates:

- `kirei(indent_width, threshold)`: decides inline vs. block rendering for an
  element's content in a single streaming pass; see [`Kirei`] for the exact
  rules.
- `bake_attr("name")`: renders ` name="value"` when `Some`, nothing when
  `None`, for any `Option<impl Display>`.
- `bake_bool_attr("name")`: renders ` name` when `true`, nothing when `false`.

Each filter returns a lightweight wrapper (`Kirei`, `OptAttr`, `BoolAttr`)
that implements both `Display` and `askama::FastWritable`, so Askama picks the
streaming path automatically and avoids the intermediate `String` allocation a
`format!()`-based filter would add.
