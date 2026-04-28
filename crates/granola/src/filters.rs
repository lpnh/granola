use askama::{FastWritable, NO_VALUES, Values};
use std::fmt::{self, Display, Formatter, Write};

/// Decides between inline and block rendering of element content in a single
/// streaming pass.
///
/// - Empty → nothing
/// - No newlines → inline
/// - Has newlines → block, each non-blank line indented by `indent_width` spaces
///   - Lines that are exactly `\n` or `\r\n` are blank and pass through without indentation
///   - `indent_width` is capped at 16
#[askama::filter_fn]
pub fn kirei<S: FastWritable>(
    source: S,
    _env: &dyn Values,
    indent_width: usize,
) -> askama::Result<Kirei<S>> {
    Ok(Kirei {
        source,
        indent_width,
    })
}

/// Renders an optional value as an HTML attribute. See [`OptAttr`].
#[askama::filter_fn]
pub fn bake_attr<'a, V: FastWritable>(
    value: &'a Option<V>,
    _env: &dyn Values,
    name: &'a str,
) -> askama::Result<OptAttr<'a, V>> {
    Ok(OptAttr { value, name })
}

/// Renders a boolean as an HTML boolean attribute. See [`BoolAttr`].
#[askama::filter_fn]
pub fn bake_bool_attr<'a>(
    value: &bool,
    _env: &dyn Values,
    name: &'a str,
) -> askama::Result<BoolAttr<'a>> {
    Ok(BoolAttr {
        value: *value,
        name,
    })
}

/// Return type of [`kirei`].
pub struct Kirei<S> {
    source: S,
    indent_width: usize,
}

// Use [`FastWritable::write_into`] instead
impl<S: FastWritable> Display for Kirei<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_into(f, NO_VALUES).map_err(|_| fmt::Error)
    }
}

impl<S: FastWritable> FastWritable for Kirei<S> {
    fn write_into(&self, dest: &mut dyn Write, values: &dyn Values) -> askama::Result<()> {
        let mut writer = KireiWriter::new(dest, self.indent_width);

        self.source.write_into(&mut writer, values)?;

        Ok(writer.finish()?)
    }
}

const MAX_INDENT: usize = 16;
const SPACES: &str = "                "; // MAX_INDENT spaces

fn indent_str(width: usize) -> &'static str {
    &SPACES[..width.min(MAX_INDENT)]
}

/// Buffers until a newline is seen, then switches to streaming block mode.
enum KireiState {
    Undecided(String),
    /// `is_newline` tracks whether the last byte written was `\n`, so the
    /// closing newline isn't duplicated when content already ends with one.
    Block {
        is_newline: bool,
    },
}

/// `Write` adapter implementing the [`kirei`] streaming state machine.
struct KireiWriter<'a, W: Write + ?Sized> {
    dest: &'a mut W,
    indent: &'static str,
    state: KireiState,
}

impl<'a, W: Write + ?Sized> KireiWriter<'a, W> {
    fn new(dest: &'a mut W, indent_width: usize) -> Self {
        KireiWriter {
            dest,
            indent: indent_str(indent_width),
            state: KireiState::Undecided(String::new()),
        }
    }

    /// Flushes buffered inline content, or appends the closing newline for
    /// block output.
    fn finish(self) -> fmt::Result {
        match self.state {
            KireiState::Undecided(buffer) => {
                if !buffer.is_empty() {
                    self.dest.write_str(&buffer)?;
                }
            }
            KireiState::Block { is_newline } => {
                if !is_newline {
                    self.dest.write_char('\n')?;
                }
            }
        }
        Ok(())
    }
}

impl<W: Write + ?Sized> Write for KireiWriter<'_, W> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let buffer = match &mut self.state {
            KireiState::Block { is_newline } => {
                return write_indented(self.dest, s, self.indent, is_newline);
            }
            KireiState::Undecided(buffer) => {
                buffer.push_str(s);
                if !s.contains('\n') {
                    return Ok(());
                }
                std::mem::take(buffer)
            }
        };

        // Buffer contains a newline: emit the opening '\n' unless the body
        // already starts with one, then stream it through the block writer.
        if !(buffer.starts_with('\n') || buffer.starts_with("\r\n")) {
            self.dest.write_char('\n')?;
        }

        let mut is_newline = true;

        let result = write_indented(self.dest, &buffer, self.indent, &mut is_newline);

        self.state = KireiState::Block { is_newline };

        result
    }
}

/// Writes `s` to `dest`, prefixing each line with `indent` when `is_newline`
/// is set — except lines whose only content is the terminator (`\n` or
/// `\r\n`), which pass through unindented. Updates `is_newline` to reflect
/// whether the last byte written was `\n`.
fn write_indented<W: Write + ?Sized>(
    dest: &mut W,
    s: &str,
    indent: &str,
    is_newline: &mut bool,
) -> fmt::Result {
    for chunk in s.split_inclusive('\n') {
        if *is_newline && !matches!(chunk, "\n" | "\r\n") {
            dest.write_str(indent)?;
        }

        dest.write_str(chunk)?;

        *is_newline = chunk.ends_with('\n');
    }
    Ok(())
}

/// Renders ` name="value"` when `Some`, nothing when `None`.
pub struct OptAttr<'a, V> {
    value: &'a Option<V>,
    name: &'a str,
}

// Use [`FastWritable::write_into`] instead
impl<V: FastWritable> Display for OptAttr<'_, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_into(f, NO_VALUES).map_err(|_| fmt::Error)
    }
}

impl<V: FastWritable> FastWritable for OptAttr<'_, V> {
    fn write_into(&self, dest: &mut dyn Write, values: &dyn Values) -> askama::Result<()> {
        if let Some(v) = self.value {
            dest.write_char(' ')?;
            dest.write_str(self.name)?;
            dest.write_str("=\"")?;
            v.write_into(dest, values)?;
            dest.write_char('"')?;
        }
        Ok(())
    }
}

/// Renders ` name` when `true`, nothing when `false`.
pub struct BoolAttr<'a> {
    value: bool,
    name: &'a str,
}

// Use [`FastWritable::write_into`] instead
impl Display for BoolAttr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_into(f, NO_VALUES).map_err(|_| fmt::Error)
    }
}

impl FastWritable for BoolAttr<'_> {
    fn write_into(&self, dest: &mut dyn Write, _values: &dyn Values) -> askama::Result<()> {
        if self.value {
            dest.write_char(' ')?;
            dest.write_str(self.name)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod opt_attr_tests {
    use askama::FastWritable;
    use std::borrow::Cow;

    use super::OptAttr;

    #[test]
    fn value_is_some() {
        let value = Some("bar");

        let attr = OptAttr {
            value: &value,
            name: "foo",
        };

        let mut buf = String::new();

        attr.write_into(&mut buf, &()).unwrap();

        assert_eq!(buf, r#" foo="bar""#);
    }

    #[test]
    fn value_is_none() {
        let value: Option<Cow<'static, str>> = None;

        let attr = OptAttr {
            value: &value,
            name: "foo",
        };

        let mut buf = String::new();

        attr.write_into(&mut buf, &()).unwrap();

        assert_eq!(buf, "");
    }
}

#[cfg(test)]
mod bool_attr_tests {
    use askama::FastWritable;

    use super::BoolAttr;

    #[test]
    fn value_is_true() {
        let attr = BoolAttr {
            value: true,
            name: "disabled",
        };

        let mut buf = String::new();

        attr.write_into(&mut buf, &()).unwrap();

        assert_eq!(buf, " disabled");
    }

    #[test]
    fn value_is_false() {
        let attr = BoolAttr {
            value: false,
            name: "disabled",
        };

        let mut buf = String::new();

        attr.write_into(&mut buf, &()).unwrap();

        assert_eq!(buf, "");
    }
}

#[cfg(test)]
mod test_util {
    use askama::FastWritable;

    use super::Kirei;

    pub(super) fn kirei(s: &str, indent: usize) -> String {
        let mut buf = String::new();

        Kirei {
            source: s,
            indent_width: indent,
        }
        .write_into(&mut buf, &())
        .unwrap();

        buf
    }
}

#[cfg(test)]
mod kirei_ws_only_tests {
    use super::test_util::kirei;

    #[test]
    fn empty() {
        assert_eq!(kirei("", 4), "");
    }

    #[test]
    fn only_whitespace() {
        assert_eq!(kirei("   ", 4), "   ");
    }

    #[test]
    fn single_tab() {
        assert_eq!(kirei("\t", 4), "\t");
    }

    #[test]
    fn multiple_tabs() {
        assert_eq!(kirei("\t\t\t", 4), "\t\t\t");
    }

    #[test]
    fn single_newline() {
        assert_eq!(kirei("\n", 4), "\n");
    }

    #[test]
    fn multiple_newlines() {
        assert_eq!(kirei("\n\n\n", 4), "\n\n\n");
    }

    #[test]
    fn single_crlf() {
        assert_eq!(kirei("\r\n", 4), "\r\n");
    }

    #[test]
    fn multiple_crlf() {
        assert_eq!(kirei("\r\n\r\n\r\n", 4), "\r\n\r\n\r\n");
    }
}

#[cfg(test)]
mod kirei_inline_tests {
    use super::test_util::kirei;

    #[test]
    fn single_word() {
        assert_eq!(kirei("halloween", 4), "halloween");
    }

    #[test]
    fn words_surrounded_by_ws() {
        assert_eq!(kirei("   hello   world   ", 4), "   hello   world   ");
    }

    #[test]
    fn words_surrounded_by_multiple_tabs() {
        assert_eq!(
            kirei("\t\t\thello\t\t\tworld\t\t\t", 4),
            "\t\t\thello\t\t\tworld\t\t\t"
        );
    }
}

#[cfg(test)]
mod kirei_block_tests {
    use super::test_util::kirei;

    #[test]
    fn newline_start() {
        assert_eq!(kirei("\nhalloween", 4), "\n    halloween\n");
    }

    #[test]
    fn newline_middle() {
        assert_eq!(kirei("hello\nworld", 4), "\n    hello\n    world\n");
    }

    #[test]
    fn newline_end() {
        assert_eq!(kirei("halloween\n", 4), "\n    halloween\n");
    }

    #[test]
    fn multiple_newline_start() {
        assert_eq!(kirei("\n\n\nhalloween", 4), "\n\n\n    halloween\n");
    }

    #[test]
    fn multiple_newline_middle() {
        assert_eq!(kirei("hello\n\n\nworld", 4), "\n    hello\n\n\n    world\n");
    }

    #[test]
    fn multiple_newline_end() {
        assert_eq!(kirei("halloween\n\n\n", 4), "\n    halloween\n\n\n");
    }

    #[test]
    fn crlf_start() {
        assert_eq!(kirei("\r\nhalloween", 4), "\r\n    halloween\n");
    }

    #[test]
    fn crlf_middle() {
        assert_eq!(kirei("hello\r\nworld", 4), "\n    hello\r\n    world\n");
    }

    #[test]
    fn crlf_end() {
        assert_eq!(kirei("halloween\r\n", 4), "\n    halloween\r\n");
    }

    #[test]
    fn multiple_crlf_start() {
        assert_eq!(
            kirei("\r\n\r\n\r\nhalloween", 4),
            "\r\n\r\n\r\n    halloween\n"
        );
    }

    #[test]
    fn multiple_crlf_middle() {
        assert_eq!(
            kirei("hello\r\n\r\n\r\nworld", 4),
            "\n    hello\r\n\r\n\r\n    world\n"
        );
    }

    #[test]
    fn multiple_crlf_end() {
        assert_eq!(
            kirei("halloween\r\n\r\n\r\n", 4),
            "\n    halloween\r\n\r\n\r\n"
        );
    }
}

#[cfg(test)]
mod kirei_indent_tests {
    use super::test_util::kirei;

    #[test]
    fn no_indent_zero_arg() {
        assert_eq!(kirei("hello\nworld", 0), "\nhello\nworld\n");
    }

    #[test]
    fn leading_indent_zero_arg() {
        assert_eq!(kirei("    hello\nworld", 0), "\n    hello\nworld\n");
    }

    #[test]
    fn leading_indent_zero_arg_2() {
        assert_eq!(kirei("hello\n    world", 0), "\nhello\n    world\n");
    }

    #[test]
    fn trailing_indent_zero_arg() {
        assert_eq!(kirei("hello    \nworld", 0), "\nhello    \nworld\n");
    }

    #[test]
    fn trailing_indent_zero_arg_2() {
        assert_eq!(kirei("hello\nworld    ", 0), "\nhello\nworld    \n");
    }

    #[test]
    fn leading_indent() {
        assert_eq!(kirei("    hello\nworld", 2), "\n      hello\n  world\n");
    }

    #[test]
    fn leading_indent_2() {
        assert_eq!(kirei("hello\n    world", 2), "\n  hello\n      world\n");
    }

    #[test]
    fn trailing_indent() {
        assert_eq!(kirei("hello    \nworld", 2), "\n  hello    \n  world\n");
    }

    #[test]
    fn trailing_indent_2() {
        assert_eq!(kirei("hello\nworld    ", 2), "\n  hello\n  world    \n");
    }

    #[test]
    fn indent_caps_at_max() {
        assert_eq!(
            kirei("foo\nbar", 42),
            format!("\n{}foo\n{}bar\n", " ".repeat(16), " ".repeat(16))
        );
    }
}

#[cfg(test)]
mod kirei_state_tests {
    use askama::{FastWritable, Values};
    use std::fmt::Write;

    use super::Kirei;

    #[test]
    fn inline() {
        struct Baz;

        impl FastWritable for Baz {
            fn write_into(&self, dest: &mut dyn Write, _: &dyn Values) -> askama::Result<()> {
                dest.write_str("halloween")?;
                dest.write_str(" ")?;
                dest.write_str("hello")?;
                dest.write_str(" ")?;
                dest.write_str("world")?;
                Ok(())
            }
        }

        let mut buf = String::new();

        Kirei {
            source: Baz,
            indent_width: 4,
        }
        .write_into(&mut buf, &())
        .unwrap();

        assert_eq!(buf, "halloween hello world");
    }

    #[test]
    fn block() {
        struct Foo;

        impl FastWritable for Foo {
            fn write_into(&self, dest: &mut dyn Write, _: &dyn Values) -> askama::Result<()> {
                dest.write_str("halloween")?;
                dest.write_str(" ")?;
                dest.write_str("hello\n")?;
                dest.write_str("world")?;
                Ok(())
            }
        }

        let mut buf = String::new();

        Kirei {
            source: Foo,
            indent_width: 4,
        }
        .write_into(&mut buf, &())
        .unwrap();

        assert_eq!(buf, "\n    halloween hello\n    world\n");
    }
}
