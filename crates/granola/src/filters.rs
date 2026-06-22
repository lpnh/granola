use askama::{FastWritable, NO_VALUES, Values};
use std::fmt::{self, Display, Formatter, Write};

#[askama::filter_fn]
pub fn kirei<S: FastWritable>(source: S, _env: &dyn Values) -> askama::Result<Kirei<S>> {
    Ok(Kirei { source })
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
        self.source.write_into(dest, values)
    }
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

    pub(super) fn kirei(s: &str) -> String {
        let mut buf = String::new();

        Kirei { source: s }.write_into(&mut buf, &()).unwrap();

        buf
    }
}

#[cfg(test)]
mod kirei_tests {
    use super::test_util::kirei;

    #[test]
    fn empty() {
        assert_eq!(kirei(""), "");
    }

    #[test]
    fn space() {
        assert_eq!(kirei("   "), "   ");
    }

    #[test]
    fn single_word() {
        assert_eq!(kirei("halloween"), "halloween");
    }

    #[test]
    fn words_surrounded_by_ws() {
        assert_eq!(kirei("   hello   world   "), "   hello   world   ");
    }

    #[test]
    fn newline_middle() {
        assert_eq!(kirei("hello\nworld"), "hello\nworld");
    }

    #[test]
    fn leading_indent() {
        assert_eq!(kirei("    hello\nworld"), "    hello\nworld");
    }

    #[test]
    fn crlf() {
        assert_eq!(kirei("hello\r\nworld"), "hello\r\nworld");
    }
}

#[cfg(test)]
mod kirei_state_tests {
    use askama::{FastWritable, Values};
    use std::fmt::Write;

    use super::Kirei;

    #[test]
    fn write() {
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

        Kirei { source: Foo }.write_into(&mut buf, &()).unwrap();

        assert_eq!(buf, "halloween hello\nworld");
    }
}
