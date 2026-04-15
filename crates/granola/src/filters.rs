use askama::{FastWritable, Values};
use std::fmt::{self, Display, Formatter, Write};

/// Decides between inline and block rendering of element content in a single
/// streaming pass.
///
/// - Empty → nothing (`<div></div>`)
/// - ≤ `threshold` bytes, no newlines → inline (`<div>hello</div>`)
/// - Otherwise → block, each non-blank line indented by `indent_width` spaces
///
/// - Blank lines pass through without indentation
/// - `indent_width` is capped at 16
/// - `threshold` is byte-based
#[askama::filter_fn]
pub fn kirei<S: Display>(
    source: S,
    _env: &dyn Values,
    indent_width: usize,
    threshold: usize,
) -> askama::Result<Kirei<S>> {
    Ok(Kirei {
        source,
        indent_width,
        threshold,
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
    threshold: usize,
}

impl<S: Display> Display for Kirei<S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut writer = KireiWriter::new(f, self.indent_width, self.threshold);
        write!(writer, "{}", self.source)?;
        writer.finish()
    }
}

impl<S: FastWritable> FastWritable for Kirei<S> {
    fn write_into(&self, dest: &mut dyn Write, values: &dyn Values) -> askama::Result<()> {
        let mut writer = KireiWriter::new(dest, self.indent_width, self.threshold);
        self.source.write_into(&mut writer, values)?;
        Ok(writer.finish()?)
    }
}

const MAX_INDENT: usize = 16;
const SPACES: &str = "                "; // MAX_INDENT spaces

fn indent_str(width: usize) -> &'static str {
    &SPACES[..width.min(MAX_INDENT)]
}

/// Buffers until the inline/block decision is forced (newline seen, or byte
/// count exceeds threshold), then switches to streaming.
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
    threshold: usize,
    state: KireiState,
}

impl<'a, W: Write + ?Sized> KireiWriter<'a, W> {
    fn new(dest: &'a mut W, indent_width: usize, threshold: usize) -> Self {
        KireiWriter {
            dest,
            indent: indent_str(indent_width),
            threshold,
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
        if let KireiState::Block {
            ref mut is_new_line,
        } = self.state
        {
            // Stream directly, indenting each non-blank line.
            for chunk in s.split_inclusive('\n') {
                if *is_new_line && !matches!(chunk, "\n" | "\r\n") {
                    self.dest.write_str(self.indent)?;
                }
                self.dest.write_str(chunk)?;
                *is_new_line = chunk.ends_with('\n');
            }
            return Ok(());
        }

        // Undecided: accumulate until the decision threshold is crossed.
        {
            let KireiState::Undecided(ref mut buffer) = self.state else {
                unreachable!()
            };
            buffer.push_str(s);
        }

        let should_transition = match &self.state {
            KireiState::Undecided(buffer) => buffer.contains('\n') || buffer.len() > self.threshold,
            _ => unreachable!(),
        };

        if should_transition {
            // Flush the buffer as the opening of a block, then switch to streaming.
            let buffer = match std::mem::replace(
                &mut self.state,
                KireiState::Block { is_new_line: false },
            ) {
                KireiState::Undecided(b) => b,
                _ => unreachable!(),
            };
            self.dest.write_char('\n')?;
            let mut is_new_line = true;
            for chunk in buffer.split_inclusive('\n') {
                if is_new_line && !matches!(chunk, "\n" | "\r\n") {
                    self.dest.write_str(self.indent)?;
                }
                self.dest.write_str(chunk)?;
                is_new_line = chunk.ends_with('\n');
            }
            self.state = KireiState::Block { is_new_line };
        }

        Ok(())
    }
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
    use super::*;

    fn w(s: &str, indent: usize, threshold: usize) -> String {
        Kirei {
            source: s,
            indent_width: indent,
            threshold,
        }
        .to_string()
    }

    #[test]
    fn empty() {
        assert_eq!(w("", 2, 70), "");
    }

    #[test]
    fn inline() {
        assert_eq!(w("hello", 2, 70), "hello");
    }

    #[test]
    fn inline_at_threshold() {
        let s = "a".repeat(70);
        assert_eq!(w(&s, 2, 70), s);
    }

    #[test]
    fn block_exceeds_threshold() {
        let s = "a".repeat(71);
        assert_eq!(w(&s, 2, 70), format!("\n  {s}\n"));
    }

    #[test]
    fn block_newline_in_content() {
        assert_eq!(w("line1\nline2", 2, 70), "\n  line1\n  line2\n");
    }

    #[test]
    fn block_multiline_trailing_newline() {
        assert_eq!(w("line1\nline2\n", 2, 70), "\n  line1\n  line2\n");
    }

    #[test]
    fn block_indent_width() {
        assert_eq!(w("line1\nline2", 4, 70), "\n    line1\n    line2\n");
    }

    // chunked writes: content written in multiple write_str calls
    #[test]
    fn chunked_inline() {
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
            threshold: 70,
        }
        .to_string();
        assert_eq!(out, "inline");
    }

    #[test]
    fn chunked_threshold_split_across_calls() {
        struct Chunked;
        impl Display for Chunked {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str(&"a".repeat(69))?;
                f.write_str("bb") // pushes to 71, crossing threshold mid-stream
            }
        }
        let out = Kirei {
            source: Chunked,
            indent_width: 2,
            threshold: 70,
        }
        .to_string();
        assert_eq!(out, format!("\n  {}\n", "a".repeat(69) + "bb"));
    }

    // blank lines: not indented, matching indent(2) default behavior
    #[test]
    fn block_blank_line_not_indented() {
        assert_eq!(w("a\n\nb", 2, 70), "\n  a\n\n  b\n");
    }

    // only a newline
    #[test]
    fn only_newline() {
        assert_eq!(w("\n", 2, 70), "\n\n");
    }

    // indent_width clamped at 16
    #[test]
    fn indent_width_capped() {
        assert_eq!(
            w("a\nb", 20, 70),
            format!("\n{}a\n{}b\n", " ".repeat(16), " ".repeat(16))
        );
    }
}
