use askama::{FastWritable, NO_VALUES, Values};
use std::fmt::{self, Display, Formatter, Write};

/// Handles inline and block rendering.
///
/// - No newline: keeps the content untouched
/// - Has newline: indent the content, ensuring it's enclosed by newlines
///     - Blank lines, i.e. `\n` or `\r\n`, pass through without indentation
///     - The indentation is capped at 16
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
    Ok(OptAttr { name, value })
}

/// Renders a boolean as an HTML boolean attribute. See [`BoolAttr`].
#[askama::filter_fn]
pub fn bake_bool_attr<'a>(
    value: &bool,
    _env: &dyn Values,
    name: &'a str,
) -> askama::Result<BoolAttr<'a>> {
    Ok(BoolAttr {
        name,
        value: *value,
    })
}

/// The content type after being piped into [`kirei`] filter.
pub struct Kirei<S> {
    source: S,
    indent_width: usize,
}

/// Forwards to [`write_into`].
///
/// [`write_into`]: FastWritable::write_into
impl<S: FastWritable> Display for Kirei<S> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_into(f, NO_VALUES).map_err(Into::into)
    }
}

impl<S: FastWritable> FastWritable for Kirei<S> {
    fn write_into(&self, dest: &mut dyn Write, values: &dyn Values) -> askama::Result<()> {
        let mut writer = KireiWriter::new(dest, self.indent_width);

        self.source.write_into(&mut writer, values)?;

        writer.finish().map_err(Into::into)
    }
}

const MAX_INDENT: usize = 16;
const SPACES: &str = "                "; // MAX_INDENT spaces

fn indent_str(width: usize) -> &'static str {
    &SPACES[..width.min(MAX_INDENT)]
}

// Buffers until a newline is seen, then switches to streaming block mode.
enum KireiState {
    Inline(String),
    Block { ends_with_newline: bool },
}

// `Write` adapter implementing the `kirei` streaming state machine.
struct KireiWriter<'a, W: Write + ?Sized> {
    dest: &'a mut W,
    indent: &'static str,
    state: KireiState,
}

impl<'a, W: Write + ?Sized> KireiWriter<'a, W> {
    fn new(dest: &'a mut W, indent_width: usize) -> Self {
        Self {
            dest,
            indent: indent_str(indent_width),
            state: KireiState::Inline(String::new()),
        }
    }

    // Inline: Flush buffered content
    // Block: Ensure trailing newline
    fn finish(self) -> fmt::Result {
        match self.state {
            KireiState::Inline(buffer) => {
                if !buffer.is_empty() {
                    self.dest.write_str(&buffer)?;
                }
            }
            KireiState::Block { ends_with_newline } => {
                if !ends_with_newline {
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
            KireiState::Inline(buffer) => {
                buffer.push_str(s);
                if !s.contains('\n') {
                    return Ok(());
                }
                std::mem::take(buffer)
            }
            KireiState::Block { ends_with_newline } => {
                return write_block(self.dest, s, self.indent, ends_with_newline);
            }
        };

        // Buffer contains a newline: emit the opening '\n' unless the body
        // already starts with one, then stream it through the block writer.
        if !(buffer.starts_with('\n') || buffer.starts_with("\r\n")) {
            self.dest.write_char('\n')?;
        }

        let mut ends_with_newline = true;

        let result = write_block(self.dest, &buffer, self.indent, &mut ends_with_newline);

        self.state = KireiState::Block { ends_with_newline };

        result
    }
}

fn write_block<W: Write + ?Sized>(
    dest: &mut W,
    s: &str,
    indent: &str,
    ends_with_newline: &mut bool,
) -> fmt::Result {
    for line in s.split_inclusive('\n') {
        if *ends_with_newline && !matches!(line, "\n" | "\r\n") {
            dest.write_str(indent)?;
        }

        dest.write_str(line)?;

        *ends_with_newline = line.ends_with('\n');
    }
    Ok(())
}

/// Renders ` name="value"` when `Some`, nothing when `None`.
pub struct OptAttr<'a, V> {
    name: &'a str,
    value: &'a Option<V>,
}

/// Forwards to [`write_into`].
///
/// [`write_into`]: FastWritable::write_into
impl<V: FastWritable> Display for OptAttr<'_, V> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_into(f, NO_VALUES).map_err(Into::into)
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
    name: &'a str,
    value: bool,
}

/// Forwards to [`write_into`].
///
/// [`write_into`]: FastWritable::write_into
impl Display for BoolAttr<'_> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.write_into(f, NO_VALUES).map_err(Into::into)
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
            name: "foo",
            value: &value,
        };

        let mut buf = String::new();

        attr.write_into(&mut buf, &()).unwrap();

        assert_eq!(buf, r#" foo="bar""#);
    }

    #[test]
    fn value_is_none() {
        let value: Option<Cow<'static, str>> = None;

        let attr = OptAttr {
            name: "foo",
            value: &value,
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
            name: "disabled",
            value: true,
        };

        let mut buf = String::new();

        attr.write_into(&mut buf, &()).unwrap();

        assert_eq!(buf, " disabled");
    }

    #[test]
    fn value_is_false() {
        let attr = BoolAttr {
            name: "disabled",
            value: false,
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
    fn space() {
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
        struct Foo;

        impl FastWritable for Foo {
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
            source: Foo,
            indent_width: 4,
        }
        .write_into(&mut buf, &())
        .unwrap();

        assert_eq!(buf, "halloween hello world");
    }

    #[test]
    fn block() {
        struct Bar;

        impl FastWritable for Bar {
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
            source: Bar,
            indent_width: 4,
        }
        .write_into(&mut buf, &())
        .unwrap();

        assert_eq!(buf, "\n    halloween hello\n    world\n");
    }
}
