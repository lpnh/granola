# Baking Strings

The `Granola` derive adds a `bake()` method to any type that already derives
`askama::Template`. `bake()` returns the rendered template as a `String`. The
derive also implements `From<T> for Cow<'static, str>` (routed through
`bake()`), so a templated struct flows directly into `Cow`-typed slots.

For composing several items into one string, the `bake_block!`, `bake_inline!`,
and `bake_newline!` macros accept any mix of `askama::Template` types and
`AsRef<str>` values (`&str`, `String`, `Cow`, ...) in a single call. Dispatch
is resolved at compile time via autoref-based specialization (`oven::Bake` /
`oven::Roast`): template items go through `render_into`, string items use
`push_str`. Capacity is reserved per item, reading `Template::SIZE_HINT` for
template items and `str::len` for string items.

The `filters` module exposes three custom Askama filters used by the HTML
element templates:

- `kirei(indent_width)`: decides inline vs. block rendering for an element's
  content in a single streaming pass; see [`kirei`] for the exact rules.
- `bake_attr("name")`: renders ` name="value"` when `Some`, nothing when
  `None`, for any `Option<impl FastWritable>`.
- `bake_bool_attr("name")`: renders ` name` when `true`, nothing when `false`.

All three filters constrain their input to `askama::FastWritable` and return a
lightweight wrapper (`Kirei`, `OptAttr`, `BoolAttr`) that implements both
`FastWritable` and `Display`.
