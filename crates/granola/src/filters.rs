use askama::{FastWritable, Values};
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
pub fn kirei<S: Display>(
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
pub fn bake_attr<'a, V: Display>(
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

impl<S: Display> Display for Kirei<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = KireiWriter::kireine(f, self.indent_width);
        write!(writer, "{}", self.source)?;
        writer.finish()
    }
}

impl<S: FastWritable> FastWritable for Kirei<S> {
    fn write_into(&self, dest: &mut dyn Write, values: &dyn Values) -> askama::Result<()> {
        let mut writer = KireiWriter::kireine(dest, self.indent_width);
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
    /// `is_new_line` tracks whether the last byte written was `\n`, so the
    /// closing newline isn't duplicated when content already ends with one.
    Block {
        is_new_line: bool,
    },
}

/// `Write` adapter implementing the [`kirei`] streaming state machine.
struct KireiWriter<'a, W: Write + ?Sized> {
    dest: &'a mut W,
    indent: &'static str,
    state: KireiState,
}

impl<'a, W: Write + ?Sized> KireiWriter<'a, W> {
    fn kireine(dest: &'a mut W, indent_width: usize) -> Self {
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
            KireiState::Block { is_new_line } => {
                if !is_new_line {
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
            KireiState::Block { is_new_line } => {
                return write_indented(self.dest, s, self.indent, is_new_line);
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

        let mut is_new_line = true;

        let result = write_indented(self.dest, &buffer, self.indent, &mut is_new_line);

        self.state = KireiState::Block { is_new_line };

        result
    }
}

/// Writes `s` to `dest`, prefixing each line with `indent` when `is_new_line`
/// is set — except lines whose only content is the terminator (`\n` or
/// `\r\n`), which pass through unindented. Updates `is_new_line` to reflect
/// whether the last byte written was `\n`.
fn write_indented<W: Write + ?Sized>(
    dest: &mut W,
    s: &str,
    indent: &str,
    is_new_line: &mut bool,
) -> fmt::Result {
    for chunk in s.split_inclusive('\n') {
        if *is_new_line && !matches!(chunk, "\n" | "\r\n") {
            dest.write_str(indent)?;
        }

        dest.write_str(chunk)?;

        *is_new_line = chunk.ends_with('\n');
    }
    Ok(())
}

/// Renders ` name="value"` when `Some`, nothing when `None`.
pub struct OptAttr<'a, V> {
    value: &'a Option<V>,
    name: &'a str,
}

impl<V: Display> Display for OptAttr<'_, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(v) = self.value {
            f.write_char(' ')?;
            f.write_str(self.name)?;
            f.write_str("=\"")?;
            write!(f, "{v}")?;
            f.write_char('"')?;
        }
        Ok(())
    }
}

impl<V: Display> FastWritable for OptAttr<'_, V> {
    fn write_into(&self, dest: &mut dyn Write, _values: &dyn Values) -> askama::Result<()> {
        if let Some(v) = self.value {
            dest.write_char(' ')?;
            dest.write_str(self.name)?;
            dest.write_str("=\"")?;
            write!(dest, "{v}")?;
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

impl Display for BoolAttr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.value {
            f.write_char(' ')?;
            f.write_str(self.name)?;
        }
        Ok(())
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
mod kirei_tests {
    use super::Kirei;
    use std::fmt::{self, Display, Formatter};

    fn kirei(s: &str, indent: usize) -> String {
        Kirei {
            source: s,
            indent_width: indent,
        }
        .to_string()
    }

    struct AskamaIndent<'a> {
        source: &'a str,
        indent_width: usize,
    }

    impl Display for AskamaIndent<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            askama::filters::indent(self.source, self.indent_width, true, false)
                .unwrap()
                .fmt(f)
        }
    }

    fn askama(s: &str, indent: usize) -> String {
        AskamaIndent {
            source: s,
            indent_width: indent,
        }
        .to_string()
    }

    #[test]
    fn test_1() {
        assert_eq!(kirei("", 2), "");
    }

    #[test]
    fn test_2() {
        assert_eq!(kirei("hello", 2), "hello");
    }

    #[test]
    fn test_3() {
        assert_eq!(kirei("hello\nworld", 4), "\n    hello\n    world\n");
    }

    #[test]
    fn test_4() {
        struct Chunked;
        impl Display for Chunked {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("in")?;
                f.write_str("line")
            }
        }
        let out = Kirei {
            source: Chunked,
            indent_width: 2,
        }
        .to_string();
        assert_eq!(out, "inline");
    }

    #[test]
    fn test_5() {
        struct Chunked;
        impl Display for Chunked {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str("hello\n")?;
                f.write_str("wor")?;
                f.write_str("ld\n")
            }
        }
        let out = Kirei {
            source: Chunked,
            indent_width: 2,
        }
        .to_string();
        assert_eq!(out, "\n  hello\n  world\n");
    }

    #[test]
    fn test_6() {
        assert_eq!(kirei("a\n\nb", 2), "\n  a\n\n  b\n");
    }

    #[test]
    fn test_7() {
        assert_eq!(kirei("\n", 2), "\n");
    }

    #[test]
    fn test_8() {
        assert_eq!(kirei("\r\n", 2), "\r\n");
    }

    #[test]
    fn test_9() {
        assert_eq!(
            kirei("a\nb", 20),
            format!("\n{}a\n{}b\n", " ".repeat(16), " ".repeat(16))
        );
    }

    #[test]
    fn test_10() {
        const INPUT: &str = "hello\ngoodbye";

        const KIREI_OUT: &str = "\n  hello\n  goodbye\n";
        const ASKAMA_OUT: &str = "  hello\n  goodbye";

        assert_eq!(kirei(INPUT, 2), KIREI_OUT);
        assert_eq!(askama(INPUT, 2), ASKAMA_OUT);

        assert_ne!(KIREI_OUT, ASKAMA_OUT);
    }

    #[test]
    fn test_11() {
        const INPUT: &str = "no newlines here";

        assert_eq!(kirei(INPUT, 2), INPUT);

        assert_ne!(askama(INPUT, 2), INPUT);
    }

    #[test]
    fn test_12() {
        const INPUT: &str = "\nhello\ngoodbye";

        const KIREI_OUT: &str = "\n  hello\n  goodbye\n";
        const ASKAMA_OUT: &str = "\n  hello\n  goodbye";

        assert_eq!(kirei(INPUT, 2), KIREI_OUT);
        assert_eq!(askama(INPUT, 2), ASKAMA_OUT);

        assert_ne!(KIREI_OUT, ASKAMA_OUT);
    }

    #[test]
    fn test_13() {
        const INPUT: &str = "  hello\ngoodbye\n";

        const KIREI_OUT: &str = "\n    hello\n  goodbye\n";
        const ASKAMA_OUT: &str = "    hello\n  goodbye\n";

        assert_eq!(kirei(INPUT, 2), KIREI_OUT);
        assert_eq!(askama(INPUT, 2), ASKAMA_OUT);

        assert_ne!(KIREI_OUT, ASKAMA_OUT);
    }

    #[test]
    fn test_14() {
        assert_eq!(kirei("a\nb", 0), "\na\nb\n");
    }

    #[test]
    fn test_15() {
        assert_eq!(kirei("\nfoo", 2), "\n  foo\n");
    }

    #[test]
    fn test_16() {
        assert_eq!(kirei("a\n\n\nb", 2), "\n  a\n\n\n  b\n");
    }

    #[test]
    fn test_17() {
        assert_eq!(kirei("  \nfoo", 2), "\n    \n  foo\n");
    }

    #[test]
    fn test_18() {
        assert_eq!(kirei("a\r\nb\r\n", 2), "\n  a\r\n  b\r\n");
    }

    #[test]
    fn test_19() {
        assert_eq!(kirei("a\r\n\r\nb", 2), "\n  a\r\n\r\n  b\n");
    }
}

#[cfg(test)]
mod ws_integration_tests {
    use crate::prelude::*;

    fn em() -> HtmlEm {
        HtmlEm::new("emphasis")
    }
    fn i() -> HtmlI {
        HtmlI::new("idiomatic_text")
    }

    #[test]
    fn test_1() {
        let p: HtmlP = HtmlP::new("");
        assert_eq!(p.bake(), "<p></p>");
    }

    #[test]
    fn test_2() {
        let p: HtmlP = HtmlP::new("hello");
        assert_eq!(p.bake(), "<p>hello</p>");
    }

    #[test]
    fn test_3() {
        let p: HtmlP = HtmlP::new("hello\nworld");
        assert_eq!(
            p.bake(),
            "\
<p>
  hello
  world
</p>"
        );
    }

    #[test]
    fn test_4() {
        let content = bake_inline!["The ", em(), " element."];
        let paragraph: HtmlP = HtmlP::new(content);
        assert_eq!(paragraph.bake(), "<p>The <em>emphasis</em> element.</p>");
    }

    #[test]
    fn test_5() {
        let content = bake_inline![em(), " and ", i(), "."];
        let paragraph: HtmlP = HtmlP::new(content);
        assert_eq!(
            paragraph.bake(),
            "<p><em>emphasis</em> and <i>idiomatic_text</i>.</p>"
        );
    }

    #[test]
    fn test_6() {
        let content = bake_block!["Items:", em(), i()];
        let paragraph: HtmlP = HtmlP::new(content);
        assert_eq!(
            paragraph.bake(),
            "\
<p>
  Items:
  <em>emphasis</em>
  <i>idiomatic_text</i>
</p>"
        );
    }

    #[test]
    fn test_7() {
        let inner: HtmlEm = HtmlEm::new("really");
        let outer: HtmlEm = HtmlEm::new(inner);
        assert_eq!(outer.bake(), "<em><em>really</em></em>");
    }

    #[test]
    fn test_8() {
        let really_surprised = bake_block!["really", "surprised"];
        let inner: HtmlEm = HtmlEm::new(really_surprised);
        let outer: HtmlEm = HtmlEm::new(inner);
        assert_eq!(
            outer.bake(),
            "\
<em>
  <em>
    really
    surprised
  </em>
</em>"
        );
    }

    #[test]
    fn test_9() {
        let e2: HtmlEm = HtmlEm::new("really");
        let e1: HtmlEm = HtmlEm::new(bake_block!["really", e2]);
        let paragraph: HtmlP = HtmlP::new(e1);
        assert_eq!(
            paragraph.bake(),
            "\
<p>
  <em>
    really
    <em>really</em>
  </em>
</p>"
        );
    }

    #[test]
    fn test_10() {
        let span: HtmlSpan = HtmlSpan::new("really\nsurprised");
        let paragraph: HtmlP = HtmlP::new(span);
        assert_eq!(
            paragraph.bake(),
            "\
<p>
  <span>
    really
    surprised
  </span>
</p>"
        );
    }

    #[test]
    fn test_11() {
        let line_1 = bake_inline!["The ", em(), " element."];
        let line_2 = bake_inline!["The ", i(), " element."];
        let content = bake_block![line_1, line_2];
        let paragraph: HtmlP = HtmlP::new(content);
        assert_eq!(
            paragraph.bake(),
            "\
<p>
  The <em>emphasis</em> element.
  The <i>idiomatic_text</i> element.
</p>"
        );
    }

    #[test]
    fn test_12() {
        let span_2: HtmlSpan = HtmlSpan::new("really surprised");
        let span_1: HtmlSpan = HtmlSpan::new("hello\nworld");
        let content = bake_block![span_1, span_2];
        let paragraph: HtmlP = HtmlP::new(content);
        assert_eq!(
            paragraph.bake(),
            "\
<p>
  <span>
    hello
    world
  </span>
  <span>really surprised</span>
</p>"
        );
    }

    #[test]
    fn test_13() {
        let span_2: HtmlSpan = HtmlSpan::new("really surprised");
        let span_1: HtmlSpan = HtmlSpan::new("hello\nworld");
        let content = bake_inline![span_1, span_2];
        let paragraph: HtmlP = HtmlP::new(content);
        assert_eq!(
            paragraph.bake(),
            "\
<p>
  <span>
    hello
    world
  </span><span>really surprised</span>
</p>"
        );
    }
}
